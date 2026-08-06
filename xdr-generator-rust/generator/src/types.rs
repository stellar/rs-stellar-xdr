use std::collections::HashSet;

use xdr_parser::ast::{Definition, Size, Type};
use xdr_parser::types::TypeInfo;

use crate::naming::type_name;

/// All resolved Rust type strings for an XDR type, computed together.
pub struct ResolvedType {
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    pub element_type: String,
    /// The Rust type used in the borrowing `View` variant of the containing
    /// type, e.g. `VecMView<'a, OperationView<'a>, 100>` for `VecM<Operation, 100>`.
    pub view_type_ref: String,
    /// An expression converting `access` (a place expression of the
    /// `view_type_ref` type) into the owned `type_ref` type.
    pub from_view_expr: String,
}

/// Resolve all Rust type information for an XDR type in one call.
///
/// When `custom_str` is true, `serde_as_type` is forced to `None`.
///
/// `view_required` is the set of Rust type names that have a borrowing `View`
/// variant. `access` is the place expression used to build `from_view_expr`,
/// and `access_is_ref` is true when `access` is a reference to the value (a
/// `match` binding) rather than a place of it.
pub(crate) fn resolve_type(
    type_: &Type,
    parent: Option<&str>,
    type_info: &TypeInfo,
    custom_str: bool,
    view_required: &HashSet<String>,
    access: &str,
    access_is_ref: bool,
) -> ResolvedType {
    let m = TypeMapping::new(type_, Some(type_info), parent);
    ResolvedType {
        type_ref: m.type_ref(),
        turbofish_type: m.turbofish_type(),
        serde_as_type: if custom_str { None } else { m.serde_as_type() },
        element_type: m.element_type(),
        view_type_ref: m.view_type_ref(view_required),
        from_view_expr: m.from_view_expr(view_required, access, access_is_ref),
    }
}

/// Get the Rust type reference for an XDR type.
/// Wraps in Box for cyclic simple and optional types.
pub(crate) fn type_ref(type_: &Type, parent_type: Option<&str>, type_info: &TypeInfo) -> String {
    TypeMapping::new(type_, Some(type_info), parent_type).type_ref()
}

/// Get the base Rust type reference (without Box wrapping).
pub(crate) fn base_type_ref(type_: &Type, type_info: Option<&TypeInfo>) -> String {
    TypeMapping::new(type_, type_info, None).base_type_ref()
}

/// Convert a Size to a Rust string representation.
pub(crate) fn size_to_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => type_name(name),
    }
}

// =============================================================================
// TypeMapping — wraps a &Type with context for Rust type resolution
// =============================================================================

/// Pairs an XDR `Type` with the contextual information needed for Rust type
/// mapping: type metadata for resolution, and optional parent type for cyclic
/// detection.
struct TypeMapping<'a> {
    type_: &'a Type,
    type_info: Option<&'a TypeInfo>,
    parent_type: Option<&'a str>,
}

impl<'a> TypeMapping<'a> {
    fn new(type_: &'a Type, type_info: Option<&'a TypeInfo>, parent_type: Option<&'a str>) -> Self {
        Self {
            type_,
            type_info,
            parent_type,
        }
    }

    /// Create a child mapping for a sub-type, inheriting context.
    fn child(&self, type_: &'a Type) -> Self {
        Self {
            type_,
            type_info: self.type_info,
            parent_type: self.parent_type,
        }
    }

    fn resolve_size(&self, size: &Size) -> String {
        match self.type_info {
            Some(ti) => ti.size_to_literal(size),
            None => size_to_string(size),
        }
    }

    fn is_cyclic(&self) -> bool {
        self.parent_type
            .and_then(|parent| {
                self.type_info.and_then(|ti| {
                    extract_ident_name(self.type_).map(|name| ti.is_cyclic(&name, parent))
                })
            })
            .unwrap_or(false)
    }

    // --- Public concerns ---

    fn base_type_ref(&self) -> String {
        match self.type_ {
            Type::Int => "i32".to_string(),
            Type::UnsignedInt => "u32".to_string(),
            Type::Hyper => "i64".to_string(),
            Type::UnsignedHyper => "u64".to_string(),
            Type::Float => "f32".to_string(),
            Type::Double => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::OpaqueFixed(size) => format!("[u8; {}]", self.resolve_size(size)),
            Type::OpaqueVar(max) => match max {
                Some(size) => format!("BytesM::<{}>", self.resolve_size(size)),
                None => "BytesM".to_string(),
            },
            Type::String(max) => match max {
                Some(size) => format!("StringM::<{}>", self.resolve_size(size)),
                None => "StringM".to_string(),
            },
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).base_type_ref();
                    }
                }
                if let Type::Ident(name) = self.type_ {
                    type_name(name)
                } else {
                    unreachable!()
                }
            }
            Type::Optional(inner) => {
                format!("Option<{}>", self.child(inner).base_type_ref())
            }
            Type::Array { element_type, size } => {
                format!(
                    "[{}; {}]",
                    self.child(element_type).base_type_ref(),
                    self.resolve_size(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).base_type_ref();
                match max_size {
                    Some(size) => format!("VecM<{elem}, {}>", self.resolve_size(size)),
                    None => format!("VecM<{elem}>"),
                }
            }
        }
    }

    fn type_ref(&self) -> String {
        let base = self.base_type_ref();

        if !self.is_cyclic() {
            return base;
        }

        match self.type_ {
            Type::Optional(inner) => {
                let inner_ref = self.child(inner).base_type_ref();
                format!("Option<Box<{inner_ref}>>")
            }
            Type::Array { .. } | Type::VarArray { .. } => base,
            _ => format!("Box<{base}>"),
        }
    }

    /// The Rust type used for this XDR type in a borrowing `View` type,
    /// without the reference wrapping applied for cyclic types.
    ///
    /// Mirrors `base_type_ref`, mapping heap-owning types to their borrowing
    /// equivalents: `VecM` to `VecMView`, `BytesM` to `BytesMView`, `StringM` to
    /// `StringMView`, and idents of types with a `View` variant to that variant.
    fn view_base_type_ref(&self, view_required: &HashSet<String>) -> String {
        match self.type_ {
            Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
            | Type::OpaqueFixed(_) => self.base_type_ref(),
            Type::OpaqueVar(max) => match max {
                Some(size) => format!("BytesMView<'a, {}>", self.resolve_size(size)),
                None => "BytesMView<'a>".to_string(),
            },
            Type::String(max) => match max {
                Some(size) => format!("StringMView<'a, {}>", self.resolve_size(size)),
                None => "StringMView<'a>".to_string(),
            },
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).view_base_type_ref(view_required);
                    }
                }
                if let Type::Ident(name) = self.type_ {
                    let name = type_name(name);
                    if view_required.contains(&name) {
                        format!("{name}View<'a>")
                    } else {
                        name
                    }
                } else {
                    unreachable!()
                }
            }
            Type::Optional(inner) => {
                format!(
                    "Option<{}>",
                    self.child(inner).view_base_type_ref(view_required)
                )
            }
            Type::Array { element_type, size } => {
                format!(
                    "[{}; {}]",
                    self.child(element_type).view_base_type_ref(view_required),
                    self.resolve_size(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).view_base_type_ref(view_required);
                match max_size {
                    Some(size) => format!("VecMView<'a, {elem}, {}>", self.resolve_size(size)),
                    None => format!("VecMView<'a, {elem}>"),
                }
            }
        }
    }

    /// The Rust type used for this XDR type in a borrowing `View` type.
    ///
    /// Mirrors `type_ref`: where the owned type wraps cyclic references in
    /// `Box`, the `View` type uses a plain reference instead.
    fn view_type_ref(&self, view_required: &HashSet<String>) -> String {
        let base = self.view_base_type_ref(view_required);

        if !self.is_cyclic() {
            return base;
        }

        match self.type_ {
            Type::Optional(inner) => {
                let inner_ref = self.child(inner).view_base_type_ref(view_required);
                format!("Option<&'a {inner_ref}>")
            }
            Type::Array { .. } | Type::VarArray { .. } => base,
            _ => format!("&'a {base}"),
        }
    }

    /// Whether the `View` mapping of this type borrows data (uses the `'a`
    /// lifetime) rather than being the same owned type.
    fn view_borrows(&self, view_required: &HashSet<String>) -> bool {
        self.view_type_ref(view_required).contains("'a")
    }

    /// Whether the owned Rust form of this type is `Copy`.
    ///
    /// Builtins and fixed opaques are `Copy`, as are generated enums, options
    /// and fixed arrays of `Copy` types, and typedef aliases of builtins.
    /// Generated structs, unions, and typedef newtypes derive `Clone` but not
    /// `Copy`.
    fn is_copy(&self) -> bool {
        if self.is_cyclic() {
            // Wrapped in `Box` in the owned form.
            return false;
        }
        match self.type_ {
            Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
            | Type::OpaqueFixed(_) => true,
            Type::OpaqueVar(_) | Type::String(_) | Type::VarArray { .. } => false,
            Type::Optional(inner) => self.child(inner).is_copy(),
            Type::Array { element_type, .. } => self.child(element_type).is_copy(),
            Type::Ident(name) => {
                if let Some(ti) = self.type_info {
                    if ti.resolve_typedef_to_builtin(self.type_).is_some() {
                        return true;
                    }
                    matches!(
                        ti.definitions.get(&type_name(name)),
                        Some(Definition::Enum(_))
                    )
                } else {
                    false
                }
            }
        }
    }

    /// The expression for reading a `Copy` value out of `access`.
    fn copy_expr(access: &str, access_is_ref: bool) -> String {
        if access_is_ref {
            format!("*{access}")
        } else {
            access.to_string()
        }
    }

    /// An expression converting `access` (a place expression of this type's
    /// `view_type_ref` form) into the owned `type_ref` form.
    ///
    /// When `access_is_ref` is true, `access` is a reference to the `Ref`
    /// form (a `match` binding) rather than a place of it.
    fn from_view_expr(
        &self,
        view_required: &HashSet<String>,
        access: &str,
        access_is_ref: bool,
    ) -> String {
        let cyclic = self.is_cyclic();
        match self.type_ {
            Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
            | Type::OpaqueFixed(_) => Self::copy_expr(access, access_is_ref),
            Type::OpaqueVar(_) => format!("{access}.to_bytesm()"),
            Type::String(_) => format!("{access}.to_stringm()"),
            Type::VarArray { element_type, .. } => {
                if self.child(element_type).view_borrows(view_required) {
                    format!("{access}.to_vecm_from()")
                } else {
                    format!("{access}.to_vecm()")
                }
            }
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).from_view_expr(
                            view_required,
                            access,
                            access_is_ref,
                        );
                    }
                }
                if let Type::Ident(name) = self.type_ {
                    let name = type_name(name);
                    if cyclic {
                        // View form is `&'a {name}View<'a>`, owned form is `Box<{name}>`.
                        if access_is_ref {
                            format!("Box::new((*{access}).into())")
                        } else {
                            format!("Box::new({access}.into())")
                        }
                    } else if view_required.contains(&name) {
                        if access_is_ref {
                            format!("{access}.into()")
                        } else {
                            format!("(&{access}).into()")
                        }
                    } else if self.is_copy() {
                        Self::copy_expr(access, access_is_ref)
                    } else {
                        format!("{access}.clone()")
                    }
                } else {
                    unreachable!()
                }
            }
            Type::Optional(inner) => {
                if cyclic {
                    // View form is `Option<&'a TView<'a>>`, owned form is `Option<Box<T>>`.
                    format!("{access}.map(|v| Box::new(v.into()))")
                } else if self.child(inner).view_borrows(view_required) {
                    format!("{access}.as_ref().map(Into::into)")
                } else if self.is_copy() {
                    Self::copy_expr(access, access_is_ref)
                } else {
                    format!("{access}.clone()")
                }
            }
            Type::Array { element_type, .. } => {
                if self.child(element_type).view_borrows(view_required) {
                    format!("core::array::from_fn(|i| (&{access}[i]).into())")
                } else if self.is_copy() {
                    Self::copy_expr(access, access_is_ref)
                } else {
                    format!("{access}.clone()")
                }
            }
        }
    }

    fn turbofish_type(&self) -> String {
        let cyclic = self.is_cyclic();

        match self.type_ {
            Type::OpaqueFixed(size) => {
                format!("<[u8; {}]>", self.resolve_size(size))
            }
            Type::Array { element_type, size } => {
                let elem = self.child(element_type).base_type_ref();
                format!("<[{elem}; {}]>", self.resolve_size(size))
            }
            Type::Optional(inner) => {
                let inner_ref = self.child(inner).base_type_ref();
                if cyclic {
                    format!("Option::<Box<{inner_ref}>>")
                } else {
                    format!("Option::<{inner_ref}>")
                }
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).base_type_ref();
                match max_size {
                    Some(size) => format!("VecM::<{elem}, {}>", self.resolve_size(size)),
                    None => format!("VecM::<{elem}>"),
                }
            }
            _ if cyclic => {
                let base = self.base_type_ref();
                format!("Box::<{base}>")
            }
            _ => self.base_type_ref(),
        }
    }

    fn element_type(&self) -> String {
        match self.type_ {
            Type::OpaqueFixed(_) | Type::OpaqueVar(_) | Type::String(_) => "u8".to_string(),
            Type::Array { element_type, .. } | Type::VarArray { element_type, .. } => {
                self.child(element_type).base_type_ref()
            }
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).base_type_ref();
                    }
                }
                if let Type::Ident(name) = self.type_ {
                    type_name(name)
                } else {
                    unreachable!()
                }
            }
            _ => "u8".to_string(),
        }
    }

    fn serde_as_type(&self) -> Option<String> {
        let base = self.base_numeric_type();
        match base.as_deref() {
            Some("i64") | Some("u64") => Some(self.serde_type_ref("NumberOrString")),
            _ => None,
        }
    }

    // --- Internal helpers ---

    fn base_numeric_type(&self) -> Option<String> {
        match self.type_ {
            Type::Hyper => Some("i64".to_string()),
            Type::UnsignedHyper => Some("u64".to_string()),
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).base_numeric_type();
                    }
                }
                None
            }
            Type::Optional(inner) => self.child(inner).base_numeric_type(),
            Type::Array { element_type, .. } => self.child(element_type).base_numeric_type(),
            Type::VarArray { element_type, .. } => self.child(element_type).base_numeric_type(),
            _ => None,
        }
    }

    fn serde_type_ref(&self, number_wrapper: &str) -> String {
        match self.type_ {
            Type::Hyper | Type::UnsignedHyper => number_wrapper.to_string(),
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).serde_type_ref(number_wrapper);
                    }
                }
                self.base_type_ref()
            }
            Type::Optional(inner) => {
                format!(
                    "Option<{}>",
                    self.child(inner).serde_type_ref(number_wrapper)
                )
            }
            Type::Array { element_type, size } => {
                format!(
                    "[{}; {}]",
                    self.child(element_type).serde_type_ref(number_wrapper),
                    size_to_string(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).serde_type_ref(number_wrapper);
                match max_size {
                    Some(size) => format!("VecM<{elem}, {}>", size_to_string(size)),
                    None => format!("VecM<{elem}>"),
                }
            }
            _ => self.base_type_ref(),
        }
    }
}

/// Extract the resolved type name from a Type for cyclic detection.
fn extract_ident_name(type_: &Type) -> Option<String> {
    match type_ {
        Type::Ident(name) => Some(type_name(name)),
        Type::Optional(inner) => extract_ident_name(inner),
        Type::Array { element_type, .. } => extract_ident_name(element_type),
        Type::VarArray { element_type, .. } => extract_ident_name(element_type),
        _ => None,
    }
}
