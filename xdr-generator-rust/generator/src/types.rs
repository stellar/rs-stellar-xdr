use xdr_parser::ast::{Size, Type};
use xdr_parser::types::TypeInfo;

use crate::naming::type_name;

/// Get the Rust type reference for an XDR type.
/// Wraps in Box for cyclic simple and optional types.
pub(crate) fn type_ref(type_: &Type, parent_type: Option<&str>, type_info: &TypeInfo) -> String {
    let base = base_type_ref(type_, Some(type_info));

    let cyclic = parent_type
        .and_then(|parent| extract_ident_name(type_).map(|name| type_info.is_cyclic(&name, parent)))
        .unwrap_or(false);

    if !cyclic {
        return base;
    }

    match type_ {
        Type::Optional(inner) => {
            let inner_ref = base_type_ref(inner, Some(type_info));
            format!("Option<Box<{inner_ref}>>")
        }
        Type::Array { .. } | Type::VarArray { .. } => base,
        _ => format!("Box<{base}>"),
    }
}

/// Get the base Rust type reference (without Box wrapping).
pub(crate) fn base_type_ref(type_: &Type, type_info: Option<&TypeInfo>) -> String {
    let resolve_size = |size: &Size| -> String {
        match type_info {
            Some(ti) => ti.size_to_literal(size),
            None => size_to_string(size),
        }
    };
    match type_ {
        Type::Int => "i32".to_string(),
        Type::UnsignedInt => "u32".to_string(),
        Type::Hyper => "i64".to_string(),
        Type::UnsignedHyper => "u64".to_string(),
        Type::Float => "f32".to_string(),
        Type::Double => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::OpaqueFixed(size) => format!("[u8; {}]", resolve_size(size)),
        Type::OpaqueVar(max) => match max {
            Some(size) => format!("BytesM::<{}>", resolve_size(size)),
            None => "BytesM".to_string(),
        },
        Type::String(max) => match max {
            Some(size) => format!("StringM::<{}>", resolve_size(size)),
            None => "StringM".to_string(),
        },
        Type::Ident(_) => {
            if let Some(ti) = type_info {
                if let Some(builtin) = ti.resolve_typedef_to_builtin(type_) {
                    return base_type_ref(builtin, type_info);
                }
            }
            if let Type::Ident(name) = type_ {
                type_name(name)
            } else {
                unreachable!()
            }
        }
        Type::Optional(inner) => format!("Option<{}>", base_type_ref(inner, type_info)),
        Type::Array { element_type, size } => {
            format!(
                "[{}; {}]",
                base_type_ref(element_type, type_info),
                resolve_size(size)
            )
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            let elem = base_type_ref(element_type, type_info);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", resolve_size(size)),
                None => format!("VecM<{elem}>"),
            }
        }
    }
}

/// Get the type expression for a read_xdr call (handles turbofish syntax).
pub(crate) fn read_call_type(
    type_: &Type,
    parent_type: Option<&str>,
    type_info: &TypeInfo,
) -> String {
    let cyclic = parent_type
        .and_then(|parent| extract_ident_name(type_).map(|name| type_info.is_cyclic(&name, parent)))
        .unwrap_or(false);

    match type_ {
        Type::OpaqueFixed(size) => {
            format!("<[u8; {}]>", type_info.size_to_literal(size))
        }
        Type::Array { element_type, size } => {
            let elem = base_type_ref(element_type, Some(type_info));
            format!("<[{elem}; {}]>", type_info.size_to_literal(size))
        }
        Type::Optional(inner) => {
            let inner_ref = base_type_ref(inner, Some(type_info));
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
            let elem = base_type_ref(element_type, Some(type_info));
            match max_size {
                Some(size) => format!("VecM::<{elem}, {}>", type_info.size_to_literal(size)),
                None => format!("VecM::<{elem}>"),
            }
        }
        _ if cyclic => {
            let base = base_type_ref(type_, Some(type_info));
            format!("Box::<{base}>")
        }
        _ => base_type_ref(type_, Some(type_info)),
    }
}

/// Get the element type for a VecM/array (for conversion impls).
pub(crate) fn element_type(type_: &Type, type_info: &TypeInfo) -> String {
    match type_ {
        Type::OpaqueFixed(_) | Type::OpaqueVar(_) | Type::String(_) => "u8".to_string(),
        Type::Array { element_type, .. } | Type::VarArray { element_type, .. } => {
            base_type_ref(element_type, Some(type_info))
        }
        Type::Ident(_) => {
            if let Some(builtin) = type_info.resolve_typedef_to_builtin(type_) {
                base_type_ref(builtin, Some(type_info))
            } else if let Type::Ident(name) = type_ {
                type_name(name)
            } else {
                unreachable!()
            }
        }
        _ => "u8".to_string(),
    }
}

/// Get the serde_as type for i64/u64 fields.
pub(crate) fn serde_as_type(type_: &Type, type_info: &TypeInfo) -> Option<String> {
    let base = base_numeric_type(type_, type_info);
    match base.as_deref() {
        Some("i64") | Some("u64") => Some(serde_type_ref(type_, "NumberOrString", type_info)),
        _ => None,
    }
}

/// Convert a Size to a Rust string representation.
pub(crate) fn size_to_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => type_name(name),
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

fn base_numeric_type(type_: &Type, type_info: &TypeInfo) -> Option<String> {
    match type_ {
        Type::Hyper => Some("i64".to_string()),
        Type::UnsignedHyper => Some("u64".to_string()),
        Type::Ident(_) => {
            if let Some(builtin) = type_info.resolve_typedef_to_builtin(type_) {
                base_numeric_type(builtin, type_info)
            } else {
                None
            }
        }
        Type::Optional(inner) => base_numeric_type(inner, type_info),
        Type::Array { element_type, .. } => base_numeric_type(element_type, type_info),
        Type::VarArray { element_type, .. } => base_numeric_type(element_type, type_info),
        _ => None,
    }
}

fn serde_type_ref(type_: &Type, number_wrapper: &str, type_info: &TypeInfo) -> String {
    match type_ {
        Type::Hyper | Type::UnsignedHyper => number_wrapper.to_string(),
        Type::Ident(_) => {
            if let Some(builtin) = type_info.resolve_typedef_to_builtin(type_) {
                serde_type_ref(builtin, number_wrapper, type_info)
            } else {
                base_type_ref(type_, Some(type_info))
            }
        }
        Type::Optional(inner) => {
            format!(
                "Option<{}>",
                serde_type_ref(inner, number_wrapper, type_info)
            )
        }
        Type::Array { element_type, size } => {
            format!(
                "[{}; {}]",
                serde_type_ref(element_type, number_wrapper, type_info),
                size_to_string(size)
            )
        }
        Type::VarArray {
            element_type,
            max_size,
        } => {
            let elem = serde_type_ref(element_type, number_wrapper, type_info);
            match max_size {
                Some(size) => format!("VecM<{elem}, {}>", size_to_string(size)),
                None => format!("VecM<{elem}>"),
            }
        }
        _ => base_type_ref(type_, Some(type_info)),
    }
}
