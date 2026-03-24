use xdr_parser::ast::{Size, Type};
use xdr_parser::types::TypeInfo;

use crate::naming::type_name;

/// All resolved Rust type strings for an XDR type, computed together.
pub struct ResolvedType {
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    pub element_type: String,
}

/// Resolve all Rust type information for an XDR type in one call.
///
/// When `custom_str` is true, `serde_as_type` is forced to `None`.
pub(crate) fn resolve_type(
    type_: &Type,
    parent: Option<&str>,
    type_info: &TypeInfo,
    custom_str: bool,
) -> ResolvedType {
    let m = TypeMapping::new(type_, Some(type_info), parent);
    ResolvedType {
        type_ref: m.type_ref(),
        turbofish_type: m.turbofish_type(),
        serde_as_type: if custom_str { None } else { m.serde_as_type() },
        element_type: m.element_type(),
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
        Size::Literal { literal: n } => n.to_string(),
        Size::Named { named: name } => type_name(name),
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

    /// If this is an Ident that's a typedef to a builtin, return a child mapping for the builtin.
    fn resolved_builtin(&self) -> Option<Self> {
        self.type_info.and_then(|ti| {
            ti.resolve_typedef_to_builtin(self.type_)
                .map(|builtin| self.child(builtin))
        })
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
            Type::OpaqueFixed { size } => format!("[u8; {}]", self.resolve_size(size)),
            Type::OpaqueVar { max_size: max } => match max {
                Some(size) => format!("BytesM::<{}>", self.resolve_size(size)),
                None => "BytesM".to_string(),
            },
            Type::String { max_size: max } => match max {
                Some(size) => format!("StringM::<{}>", self.resolve_size(size)),
                None => "StringM".to_string(),
            },
            Type::Ident { ident: name } => {
                if let Some(child) = self.resolved_builtin() {
                    return child.base_type_ref();
                }
                type_name(name)
            }
            Type::Optional { element_type: inner } => {
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
            Type::Optional { element_type: inner } => {
                let inner_ref = self.child(inner).base_type_ref();
                format!("Option<Box<{inner_ref}>>")
            }
            Type::Array { .. } | Type::VarArray { .. } => base,
            _ => format!("Box<{base}>"),
        }
    }

    fn turbofish_type(&self) -> String {
        let cyclic = self.is_cyclic();

        match self.type_ {
            Type::OpaqueFixed { size } => {
                format!("<[u8; {}]>", self.resolve_size(size))
            }
            Type::Array { element_type, size } => {
                let elem = self.child(element_type).base_type_ref();
                format!("<[{elem}; {}]>", self.resolve_size(size))
            }
            Type::Optional { element_type: inner } => {
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
            Type::OpaqueFixed { size: _ } | Type::OpaqueVar { max_size: _ } | Type::String { max_size: _ } => "u8".to_string(),
            Type::Array { element_type, .. } | Type::VarArray { element_type, .. } => {
                self.child(element_type).base_type_ref()
            }
            Type::Ident { ident: name } => {
                if let Some(child) = self.resolved_builtin() {
                    return child.base_type_ref();
                }
                type_name(name)
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
            Type::Ident { ident: _ } => {
                self.resolved_builtin().and_then(|child| child.base_numeric_type())
            }
            Type::Optional { element_type: inner } => self.child(inner).base_numeric_type(),
            Type::Array { element_type, .. } => self.child(element_type).base_numeric_type(),
            Type::VarArray { element_type, .. } => self.child(element_type).base_numeric_type(),
            _ => None,
        }
    }

    fn serde_type_ref(&self, number_wrapper: &str) -> String {
        match self.type_ {
            Type::Hyper | Type::UnsignedHyper => number_wrapper.to_string(),
            Type::Ident { ident: _ } => {
                if let Some(child) = self.resolved_builtin() {
                    return child.serde_type_ref(number_wrapper);
                }
                self.base_type_ref()
            }
            Type::Optional { element_type: inner } => {
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
        Type::Ident { ident: name } => Some(type_name(name)),
        Type::Optional { element_type: inner } => extract_ident_name(inner),
        Type::Array { element_type, .. } => extract_ident_name(element_type),
        Type::VarArray { element_type, .. } => extract_ident_name(element_type),
        _ => None,
    }
}
