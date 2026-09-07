use std::collections::HashSet;

use xdr_parser::ast::{Size, Type};
use xdr_parser::types::TypeInfo;

use crate::naming::{const_name, type_name};
use crate::output::CyclicBorrow;

/// All resolved Rust type strings for an XDR type, computed together.
pub struct ResolvedType {
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    pub element_type: String,
    /// The Rust type used in the borrowing `Ref` variant of the containing
    /// type, e.g. `VecMRef<'a, OperationRef<'a>, 100>` for `VecM<Operation, 100>`.
    pub ref_type: String,
    /// Whether the `Ref` form borrows to break a cycle, and so whether the
    /// conversion to the owned form has to restore a `Box`.
    pub cyclic: CyclicBorrow,
}

/// Resolve all Rust type information for an XDR type in one call.
///
/// When `custom_str` is true, `serde_as_type` is forced to `None`.
///
/// `ref_required` is the set of Rust type names that have a borrowing `Ref`
/// variant.
pub(crate) fn resolve_type(
    type_: &Type,
    parent: Option<&str>,
    type_info: &TypeInfo,
    custom_str: bool,
    ref_required: &HashSet<String>,
) -> ResolvedType {
    let m = TypeMapping::new(type_, Some(type_info), parent);
    ResolvedType {
        type_ref: m.type_ref(),
        turbofish_type: m.turbofish_type(),
        serde_as_type: if custom_str { None } else { m.serde_as_type() },
        element_type: m.element_type(),
        ref_type: m.ref_type(ref_required),
        cyclic: m.cyclic_borrow(),
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

/// The Rust type that holds this XDR type in a const context: the borrowing
/// `Ref` form where the type owns heap data, the owned type otherwise.
///
/// The `'a` of the `Ref` types is rendered as `'_`, so the result is usable in
/// a function signature. Mirrors [`type_ref`], including the reference wrapping
/// applied where `parent_type` makes the type cyclic.
pub(crate) fn const_ref_type(
    type_: &Type,
    parent_type: Option<&str>,
    type_info: &TypeInfo,
    ref_required: &HashSet<String>,
) -> String {
    TypeMapping::new(type_, Some(type_info), parent_type)
        .ref_type(ref_required)
        .replace("'a", "'_")
}

/// As [`const_ref_type`], but without the reference wrapping for cyclic types.
///
/// This is the form an element takes inside a container such as `VecMRef`,
/// which borrows its elements as a slice rather than individually.
pub(crate) fn const_ref_base_type(
    type_: &Type,
    type_info: &TypeInfo,
    ref_required: &HashSet<String>,
) -> String {
    TypeMapping::new(type_, Some(type_info), None)
        .ref_base_type(ref_required)
        .replace("'a", "'_")
}

/// Convert a Size to a Rust `u32` const generic argument, as used by
/// `BytesM`, `StringM`, and `VecM`. Named sizes refer to the generated const,
/// which is emitted as a `u32`.
pub(crate) fn size_to_u32_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => const_name(name),
    }
}

/// Convert a Size to a Rust `usize` array length. Named sizes refer to the
/// generated const, which is emitted as a `u32` and so needs casting.
pub(crate) fn size_to_usize_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => format!("{{ {} as usize }}", const_name(name)),
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
            Type::OpaqueFixed(size) => format!("[u8; {}]", size_to_usize_string(size)),
            Type::OpaqueVar(max) => match max {
                Some(size) => format!("BytesM::<{}>", size_to_u32_string(size)),
                None => "BytesM".to_string(),
            },
            Type::String(max) => match max {
                Some(size) => format!("StringM::<{}>", size_to_u32_string(size)),
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
                    size_to_usize_string(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).base_type_ref();
                match max_size {
                    Some(size) => format!("VecM<{elem}, {}>", size_to_u32_string(size)),
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

    /// The Rust type used for this XDR type in a borrowing `Ref` type,
    /// without the reference wrapping applied for cyclic types.
    ///
    /// Mirrors `base_type_ref`, mapping heap-owning types to their borrowing
    /// equivalents: `VecM` to `VecMRef`, `BytesM` to `BytesMRef`, `StringM` to
    /// `StringMRef`, and idents of types with a `Ref` variant to that variant.
    fn ref_base_type(&self, ref_required: &HashSet<String>) -> String {
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
                Some(size) => format!("BytesMRef<'a, {}>", size_to_u32_string(size)),
                None => "BytesMRef<'a>".to_string(),
            },
            Type::String(max) => match max {
                Some(size) => format!("StringMRef<'a, {}>", size_to_u32_string(size)),
                None => "StringMRef<'a>".to_string(),
            },
            Type::Ident(_) => {
                if let Some(ti) = self.type_info {
                    if let Some(builtin) = ti.resolve_typedef_to_builtin(self.type_) {
                        return self.child(builtin).ref_base_type(ref_required);
                    }
                }
                if let Type::Ident(name) = self.type_ {
                    let name = type_name(name);
                    if ref_required.contains(&name) {
                        format!("{name}Ref<'a>")
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
                    self.child(inner).ref_base_type(ref_required)
                )
            }
            Type::Array { element_type, size } => {
                format!(
                    "[{}; {}]",
                    self.child(element_type).ref_base_type(ref_required),
                    size_to_usize_string(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).ref_base_type(ref_required);
                match max_size {
                    Some(size) => format!("VecMRef<'a, {elem}, {}>", size_to_u32_string(size)),
                    None => format!("VecMRef<'a, {elem}>"),
                }
            }
        }
    }

    /// How the `Ref` form of this type breaks a cycle, if it does.
    ///
    /// Mirrors the cyclic branching of [`Self::ref_type`] and
    /// [`Self::type_ref`]: those decide where a `&'a` and a `Box` appear, and
    /// this reports it so the conversion between the two forms can restore the
    /// `Box`. A cyclic array or `VecM` is already indirect, so neither form
    /// wraps it and there is nothing to restore.
    fn cyclic_borrow(&self) -> CyclicBorrow {
        if !self.is_cyclic() {
            return CyclicBorrow::NotCyclic;
        }
        match self.type_ {
            Type::Optional(_) => CyclicBorrow::OptionalReference,
            Type::Array { .. } | Type::VarArray { .. } => CyclicBorrow::NotCyclic,
            _ => CyclicBorrow::Reference,
        }
    }

    /// The Rust type used for this XDR type in a borrowing `Ref` type.
    ///
    /// Mirrors `type_ref`: where the owned type wraps cyclic references in
    /// `Box`, the `Ref` type uses a plain reference instead.
    fn ref_type(&self, ref_required: &HashSet<String>) -> String {
        let base = self.ref_base_type(ref_required);

        if !self.is_cyclic() {
            return base;
        }

        match self.type_ {
            Type::Optional(inner) => {
                let inner_ref = self.child(inner).ref_base_type(ref_required);
                format!("Option<&'a {inner_ref}>")
            }
            Type::Array { .. } | Type::VarArray { .. } => base,
            _ => format!("&'a {base}"),
        }
    }

    fn turbofish_type(&self) -> String {
        let cyclic = self.is_cyclic();

        match self.type_ {
            Type::OpaqueFixed(size) => {
                format!("<[u8; {}]>", size_to_usize_string(size))
            }
            Type::Array { element_type, size } => {
                let elem = self.child(element_type).base_type_ref();
                format!("<[{elem}; {}]>", size_to_usize_string(size))
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
                    Some(size) => format!("VecM::<{elem}, {}>", size_to_u32_string(size)),
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
                    size_to_usize_string(size)
                )
            }
            Type::VarArray {
                element_type,
                max_size,
            } => {
                let elem = self.child(element_type).serde_type_ref(number_wrapper);
                match max_size {
                    Some(size) => format!("VecM<{elem}, {}>", size_to_u32_string(size)),
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
