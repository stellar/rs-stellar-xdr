//! Type resolution, reference computation, and attribute generation.

use crate::ast::{Definition, Size, Type, XdrSpec};
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::collections::{HashMap, HashSet};

/// Configuration options for code generation.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Types that have custom Default implementations (skip derive(Default))
    pub custom_default_impl: HashSet<String>,
    /// Types that have custom FromStr/Display implementations (use SerializeDisplay)
    pub custom_str_impl: HashSet<String>,
    /// Types that should NOT have Display/FromStr/schemars generated (have special handling elsewhere)
    pub no_display_fromstr: HashSet<String>,
}

/// Type information collected from the XDR spec.
pub struct TypeInfo {
    /// All type names defined in the spec
    pub type_names: HashSet<String>,
    /// Map from type name to its definition
    pub definitions: HashMap<String, Definition>,
    /// Map from type name to the types it references in its fields
    pub type_field_types: HashMap<String, Vec<String>>,
    /// Map from const name to its value
    pub const_values: HashMap<String, i64>,
}

impl TypeInfo {
    /// Build type information from an XDR spec.
    pub fn build(spec: &XdrSpec) -> Self {
        let mut type_names = HashSet::new();
        let mut definitions = HashMap::new();
        let mut type_field_types: HashMap<String, Vec<String>> = HashMap::new();
        let mut const_values = HashMap::new();

        // Collect all definitions
        for def in spec.all_definitions() {
            let name = rust_type_name(def.name());
            type_names.insert(name.clone());
            definitions.insert(name.clone(), def.clone());

            // Collect field types for cyclic detection
            let field_types = collect_field_types(def);
            type_field_types.insert(name, field_types);

            // Collect const values
            if let Definition::Const(c) = def {
                const_values.insert(c.name.clone(), c.value);
            }
        }

        Self {
            type_names,
            definitions,
            type_field_types,
            const_values,
        }
    }

    /// Check if `type_with_fields` has a cyclic reference to `target_type`.
    pub fn is_cyclic(&self, type_with_fields: &str, target_type: &str) -> bool {
        self.is_cyclic_inner(type_with_fields, target_type, &mut HashSet::new())
    }

    /// Resolve a size to a string, using const values for named sizes.
    pub fn size_to_literal(&self, size: &Size) -> String {
        match size {
            Size::Literal(n) => n.to_string(),
            Size::Named(name) => {
                // Look up the const value and return the literal
                if let Some(&value) = self.const_values.get(name) {
                    value.to_string()
                } else {
                    // Fallback to the name if not found (shouldn't happen)
                    rust_type_name(name)
                }
            }
        }
    }

    fn is_cyclic_inner(
        &self,
        type_with_fields: &str,
        target_type: &str,
        seen: &mut HashSet<String>,
    ) -> bool {
        if seen.contains(type_with_fields) {
            return false;
        }
        seen.insert(type_with_fields.to_string());

        if let Some(field_types) = self.type_field_types.get(type_with_fields) {
            for ft in field_types {
                if ft == target_type {
                    return true;
                }
                if self.is_cyclic_inner(ft, target_type, seen) {
                    return true;
                }
            }
        }
        false
    }
}

/// Collect the base type names referenced in a definition's fields.
fn collect_field_types(def: &Definition) -> Vec<String> {
    match def {
        Definition::Struct(s) => s
            .members
            .iter()
            .filter_map(|m| base_type_name(&m.type_))
            .collect(),
        Definition::Union(u) => u
            .arms
            .iter()
            .filter_map(|arm| arm.type_.as_ref().and_then(base_type_name))
            .collect(),
        // Note: Typedefs are NOT included in cycle detection, matching Ruby behavior.
        // Only structs and unions have their field types tracked for cycle detection.
        Definition::Typedef(_) | Definition::Enum(_) | Definition::Const(_) => vec![],
    }
}

/// Get the base type name from a Type (for cyclic detection).
fn base_type_name(type_: &Type) -> Option<String> {
    match type_ {
        Type::Ident(name) => Some(rust_type_name(name)),
        Type::Optional(inner) => base_type_name(inner),
        Type::Array { element_type, .. } => base_type_name(element_type),
        Type::VarArray { element_type, .. } => base_type_name(element_type),
        _ => None,
    }
}

/// Convert an XDR name to a Rust type name (UpperCamelCase).
pub fn rust_type_name(name: &str) -> String {
    let name = escape_name(name);
    name.to_upper_camel_case()
}

/// Convert an XDR name to a Rust field name (snake_case).
pub fn rust_field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    // Apply escape AFTER snake_case since to_snake_case strips trailing underscores
    escape_field_name(&snake)
}

/// Escape reserved names for type names.
fn escape_name(name: &str) -> String {
    match name {
        "type" => "type_".to_string(),
        "Error" => "SError".to_string(),
        _ => name.to_string(),
    }
}

/// Escape reserved names for field names (after snake_case conversion).
fn escape_field_name(name: &str) -> String {
    match name {
        "type" => "type_".to_string(),
        _ => name.to_string(),
    }
}

/// Check if a type is a builtin (maps directly to a Rust primitive).
pub fn is_builtin_type(type_: &Type) -> bool {
    matches!(
        type_,
        Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
    )
}

/// Check if a type is a fixed-length opaque array.
pub fn is_fixed_opaque(type_: &Type) -> bool {
    matches!(type_, Type::OpaqueFixed(_))
}

/// Check if a type is a fixed-length array (including fixed opaque).
pub fn is_fixed_array(type_: &Type) -> bool {
    matches!(type_, Type::OpaqueFixed(_) | Type::Array { .. })
}

/// Check if a type is a variable-length array.
pub fn is_var_array(type_: &Type) -> bool {
    matches!(
        type_,
        Type::OpaqueVar(_) | Type::String(_) | Type::VarArray { .. }
    )
}

/// Get the Rust type reference for an XDR type.
/// Wraps in Box for cyclic :simple and :optional types (matching Ruby behavior).
/// Does NOT wrap arrays/var_arrays even if cyclic.
pub fn rust_type_ref(type_: &Type, parent_type: Option<&str>, type_info: &TypeInfo) -> String {
    let base = base_rust_type_ref_with_info(type_, type_info);

    // Check for cyclic reference (only for simple and optional types)
    let is_cyclic = if let Some(parent) = parent_type {
        if let Some(base_name) = base_type_name(type_) {
            type_info.is_cyclic(&base_name, parent)
        } else {
            false
        }
    } else {
        false
    };

    match type_ {
        Type::Optional(inner) => {
            let inner_ref = base_rust_type_ref_with_info(inner, type_info);
            if is_cyclic {
                format!("Option<Box<{inner_ref}>>")
            } else {
                format!("Option<{inner_ref}>")
            }
        }
        Type::Array { element_type, size } => {
            // Arrays are NOT wrapped in Box even if cyclic
            let elem = base_rust_type_ref_with_info(element_type, type_info);
            let size = type_info.size_to_literal(size);
            format!("[{elem}; {size}]")
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            // VarArrays are NOT wrapped in Box even if cyclic
            let elem = base_rust_type_ref_with_info(element_type, type_info);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", type_info.size_to_literal(size)),
                None => format!("VecM<{elem}>"),
            }
        }
        _ => {
            // Simple types: wrap in Box if cyclic
            if is_cyclic {
                format!("Box<{base}>")
            } else {
                base
            }
        }
    }
}

/// Get the base Rust type reference (without Box/Option wrapping).
/// Uses const names for sizes - use base_rust_type_ref_with_info for literal sizes.
pub fn base_rust_type_ref(type_: &Type) -> String {
    match type_ {
        Type::Int => "i32".to_string(),
        Type::UnsignedInt => "u32".to_string(),
        Type::Hyper => "i64".to_string(),
        Type::UnsignedHyper => "u64".to_string(),
        Type::Float => "f32".to_string(),
        Type::Double => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::OpaqueFixed(size) => format!("[u8; {}]", size_to_rust(size)),
        Type::OpaqueVar(max) => match max {
            Some(size) => format!("BytesM::<{}>", size_to_rust(size)),
            None => "BytesM".to_string(),
        },
        Type::String(max) => match max {
            Some(size) => format!("StringM::<{}>", size_to_rust(size)),
            None => "StringM".to_string(),
        },
        Type::Ident(name) => rust_type_name(name),
        Type::Optional(inner) => format!("Option<{}>", base_rust_type_ref(inner)),
        Type::Array { element_type, size } => {
            format!(
                "[{}; {}]",
                base_rust_type_ref(element_type),
                size_to_rust(size)
            )
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            let elem = base_rust_type_ref(element_type);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", size_to_rust(size)),
                None => format!("VecM<{elem}>"),
            }
        }
        Type::AnonymousUnion {
            discriminant, arms, ..
        } => {
            // This should never happen - anonymous unions are extracted during parsing
            panic!(
                "AnonymousUnion should have been extracted during parsing. \
                 Discriminant: {:?}, Arms count: {}",
                discriminant.name,
                arms.len()
            )
        }
    }
}

/// Get the base Rust type reference with type_info for resolving const sizes to literals.
fn base_rust_type_ref_with_info(type_: &Type, type_info: &TypeInfo) -> String {
    match type_ {
        Type::Int => "i32".to_string(),
        Type::UnsignedInt => "u32".to_string(),
        Type::Hyper => "i64".to_string(),
        Type::UnsignedHyper => "u64".to_string(),
        Type::Float => "f32".to_string(),
        Type::Double => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::OpaqueFixed(size) => format!("[u8; {}]", type_info.size_to_literal(size)),
        Type::OpaqueVar(max) => match max {
            Some(size) => format!("BytesM::<{}>", type_info.size_to_literal(size)),
            None => "BytesM".to_string(),
        },
        Type::String(max) => match max {
            Some(size) => format!("StringM::<{}>", type_info.size_to_literal(size)),
            None => "StringM".to_string(),
        },
        Type::Ident(name) => rust_type_name(name),
        Type::Optional(inner) => {
            format!("Option<{}>", base_rust_type_ref_with_info(inner, type_info))
        }
        Type::Array { element_type, size } => {
            format!(
                "[{}; {}]",
                base_rust_type_ref_with_info(element_type, type_info),
                type_info.size_to_literal(size)
            )
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            let elem = base_rust_type_ref_with_info(element_type, type_info);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", type_info.size_to_literal(size)),
                None => format!("VecM<{elem}>"),
            }
        }
        Type::AnonymousUnion {
            discriminant, arms, ..
        } => {
            panic!(
                "AnonymousUnion should have been extracted during parsing. \
                 Discriminant: {:?}, Arms count: {}",
                discriminant.name,
                arms.len()
            )
        }
    }
}

/// Get the type to use in a read_xdr call (handles turbofish syntax).
pub fn rust_read_call_type(
    type_: &Type,
    parent_type: Option<&str>,
    type_info: &TypeInfo,
) -> String {
    let ref_type = rust_type_ref(type_, parent_type, type_info);

    // Handle special cases for turbofish
    if ref_type.starts_with('[') && ref_type.ends_with(']') {
        format!("<{ref_type}>")
    } else if ref_type.starts_with("Box<") {
        format!("Box::{}", &ref_type[3..])
    } else if ref_type.starts_with("Option<Box<") {
        format!("Option::<Box<{}>>", &ref_type[11..ref_type.len() - 2])
    } else if ref_type.starts_with("Option<") {
        format!("Option::<{}>", &ref_type[7..ref_type.len() - 1])
    } else if ref_type.starts_with("VecM<") {
        // VecM<T> -> VecM::<T> or VecM<T, N> -> VecM::<T, N>
        format!("VecM::{}", &ref_type[4..])
    } else if ref_type.starts_with("BytesM<") {
        format!("BytesM::{}", &ref_type[6..])
    } else if ref_type.starts_with("BytesM") && ref_type.len() == 6 {
        ref_type
    } else if ref_type.starts_with("StringM<") {
        format!("StringM::{}", &ref_type[7..])
    } else if ref_type.starts_with("StringM") && ref_type.len() == 7 {
        ref_type
    } else {
        ref_type
    }
}

fn size_to_rust(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => rust_type_name(name),
    }
}

/// Get the element type for a VecM/array (for conversion impls).
pub fn element_type_for_vec(type_: &Type) -> String {
    match type_ {
        Type::OpaqueFixed(_) | Type::OpaqueVar(_) | Type::String(_) => "u8".to_string(),
        Type::Array { element_type, .. } | Type::VarArray { element_type, .. } => {
            base_rust_type_ref(element_type)
        }
        Type::Ident(name) => rust_type_name(name),
        _ => "u8".to_string(),
    }
}

/// Generate serde field attributes for i64/u64 fields.
pub fn serde_field_attr(type_: &Type) -> Option<String> {
    let base = get_base_numeric_type(type_);
    match base.as_deref() {
        Some("i64") | Some("u64") => {
            let ref_type = rust_type_ref_for_serde(type_, "NumberOrString");
            Some(format!(
                r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_as(as = "{ref_type}"))]"#
            ))
        }
        _ => None,
    }
}

fn get_base_numeric_type(type_: &Type) -> Option<String> {
    match type_ {
        Type::Hyper => Some("i64".to_string()),
        Type::UnsignedHyper => Some("u64".to_string()),
        Type::Optional(inner) => get_base_numeric_type(inner),
        Type::Array { element_type, .. } => get_base_numeric_type(element_type),
        Type::VarArray { element_type, .. } => get_base_numeric_type(element_type),
        _ => None,
    }
}

fn rust_type_ref_for_serde(type_: &Type, number_wrapper: &str) -> String {
    match type_ {
        Type::Hyper | Type::UnsignedHyper => number_wrapper.to_string(),
        Type::Optional(inner) => {
            format!("Option<{}>", rust_type_ref_for_serde(inner, number_wrapper))
        }
        Type::Array { element_type, size } => {
            format!(
                "[{}; {}]",
                rust_type_ref_for_serde(element_type, number_wrapper),
                size_to_rust(size)
            )
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            let elem = rust_type_ref_for_serde(element_type, number_wrapper);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", size_to_rust(size)),
                None => format!("VecM<{elem}>"),
            }
        }
        _ => base_rust_type_ref(type_),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_type_name() {
        assert_eq!(rust_type_name("public_key"), "PublicKey");
        assert_eq!(
            rust_type_name("PUBLIC_KEY_TYPE_ED25519"),
            "PublicKeyTypeEd25519"
        );
    }

    #[test]
    fn test_rust_field_name() {
        assert_eq!(rust_field_name("publicKey"), "public_key");
        assert_eq!(rust_field_name("type"), "type_");
    }

    #[test]
    fn test_base_rust_type_ref() {
        assert_eq!(base_rust_type_ref(&Type::Int), "i32");
        assert_eq!(base_rust_type_ref(&Type::UnsignedHyper), "u64");
        assert_eq!(
            base_rust_type_ref(&Type::OpaqueFixed(Size::Literal(32))),
            "[u8; 32]"
        );
    }
}
