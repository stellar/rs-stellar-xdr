//! Shadow types for the JSON IR output format.
//!
//! These types are designed for consumers (Go, TypeScript, etc.), not for the
//! Rust parser's internal representation. They are constructed from the AST
//! types with computed properties (fixed_size, resolved sizes, resolved case
//! values) embedded directly.

use serde::Serialize;
use std::collections::HashMap;
use xdr_parser::ast;

/// Top-level IR output.
#[derive(Serialize)]
pub struct IR {
    pub version: u32,
    pub files: Vec<XdrFile>,
    pub resolved_features: Vec<String>,
    pub definitions: Vec<Definition>,
}

#[derive(Serialize)]
pub struct XdrFile {
    pub name: String,
    pub sha256: String,
}

/// A definition with computed properties embedded.
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Definition {
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Typedef(Typedef),
    Const(Const),
}

#[derive(Serialize)]
pub struct Struct {
    pub name: String,
    pub fixed_size: Option<u32>,
    pub source: String,
    pub file_index: usize,
    pub fields: Vec<Field>,
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: TypeRef,
}

#[derive(Serialize)]
pub struct Union {
    pub name: String,
    pub fixed_size: Option<u32>,
    pub source: String,
    pub file_index: usize,
    pub discriminant: Field,
    pub arms: Vec<UnionArm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arm: Option<UnionDefaultArm>,
}

#[derive(Serialize)]
pub struct UnionArm {
    pub cases: Vec<UnionCase>,
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<TypeRef>,
}

#[derive(Serialize)]
pub struct UnionCase {
    pub value: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct UnionDefaultArm {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<TypeRef>,
}

#[derive(Serialize)]
pub struct Enum {
    pub name: String,
    pub fixed_size: Option<u32>,
    pub source: String,
    pub file_index: usize,
    pub member_prefix: String,
    pub members: Vec<EnumMember>,
}

#[derive(Serialize)]
pub struct EnumMember {
    pub name: String,
    pub value: i64,
}

#[derive(Serialize)]
pub struct Typedef {
    pub name: String,
    pub fixed_size: Option<u32>,
    pub source: String,
    pub file_index: usize,
    #[serde(rename = "type")]
    pub type_: TypeRef,
}

#[derive(Serialize)]
pub struct Const {
    pub name: String,
    pub value: i64,
    pub source: String,
    pub file_index: usize,
}

/// Type reference — describes the type of a field, arm, or typedef target.
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeRef {
    Int,
    UnsignedInt,
    Hyper,
    UnsignedHyper,
    Float,
    Double,
    Bool,
    OpaqueFixed {
        size: u64,
    },
    OpaqueVar {
        #[serde(skip_serializing_if = "Option::is_none")]
        max_size: Option<u64>,
    },
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        max_size: Option<u64>,
    },
    Ref {
        name: String,
    },
    Array {
        element: Box<TypeRef>,
        count: u64,
    },
    VarArray {
        element: Box<TypeRef>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_count: Option<u64>,
    },
    Optional {
        element: Box<TypeRef>,
    },
}

/// Build IR definitions from the AST spec and computed properties.
pub fn build_definitions(
    spec: &ast::XdrSpec,
    computed: &xdr_parser::types::ComputedProperties,
) -> Vec<Definition> {
    // Build enum member value lookup for resolving union case idents.
    let mut enum_member_values: HashMap<&str, i32> = HashMap::new();
    for def in spec.all_definitions() {
        if let ast::Definition::Enum(e) = def {
            for m in &e.members {
                enum_member_values.insert(&m.name, m.value);
            }
        }
    }

    // Build const value lookup for resolving named sizes.
    let const_values = &computed.const_values;

    spec.all_definitions()
        .map(|def| convert_definition(def, computed, const_values, &enum_member_values))
        .collect()
}

fn convert_definition(
    def: &ast::Definition,
    computed: &xdr_parser::types::ComputedProperties,
    const_values: &HashMap<String, i64>,
    enum_member_values: &HashMap<&str, i32>,
) -> Definition {
    let fixed_size = computed.types.get(def.name()).and_then(|p| p.fixed_wire_size);
    match def {
        ast::Definition::Struct(s) => Definition::Struct(Struct {
            name: s.name.clone(),
            fixed_size,
            source: s.source.clone(),
            file_index: s.file_index,
            fields: s.members.iter().map(|m| Field {
                name: m.name.clone(),
                type_: convert_type(&m.type_, const_values),
            }).collect(),
        }),
        ast::Definition::Union(u) => Definition::Union(Union {
            name: u.name.clone(),
            fixed_size,
            source: u.source.clone(),
            file_index: u.file_index,
            discriminant: Field {
                name: u.discriminant.name.clone(),
                type_: convert_type(&u.discriminant.type_, const_values),
            },
            arms: u.arms.iter().map(|arm| {
                let cases = arm.cases.iter().map(|c| match &c.value {
                    ast::UnionCaseValue::Literal(literal) => UnionCase {
                        value: i64::from(*literal),
                        name: None,
                    },
                    ast::UnionCaseValue::Ident(ident) => {
                        let value = enum_member_values.get(ident.as_str())
                            .map(|&v| i64::from(v))
                            .or_else(|| const_values.get(ident).copied())
                            .unwrap_or_else(|| panic!(
                                "union {}: unresolved case ident '{ident}'", u.name
                            ));
                        UnionCase { value, name: Some(ident.clone()) }
                    }
                }).collect();
                UnionArm {
                    cases,
                    name: arm.name.clone(),
                    type_: arm.type_.as_ref().map(|t| convert_type(t, const_values)),
                }
            }).collect(),
            default_arm: None, // default arms not yet supported by parser
        }),
        ast::Definition::Enum(e) => Definition::Enum(Enum {
            name: e.name.clone(),
            fixed_size,
            source: e.source.clone(),
            file_index: e.file_index,
            member_prefix: e.member_prefix.clone(),
            members: e.members.iter().map(|m| EnumMember {
                name: m.name.clone(),
                value: i64::from(m.value),
            }).collect(),
        }),
        ast::Definition::Typedef(t) => Definition::Typedef(Typedef {
            name: t.name.clone(),
            fixed_size,
            source: t.source.clone(),
            file_index: t.file_index,
            type_: convert_type(&t.type_, const_values),
        }),
        ast::Definition::Const(c) => Definition::Const(Const {
            name: c.name.clone(),
            value: c.value,
            source: c.source.clone(),
            file_index: c.file_index,
        }),
    }
}

/// Convert an AST type to an IR type reference, resolving named sizes to integers.
fn convert_type(ty: &ast::Type, const_values: &HashMap<String, i64>) -> TypeRef {
    match ty {
        ast::Type::Int => TypeRef::Int,
        ast::Type::UnsignedInt => TypeRef::UnsignedInt,
        ast::Type::Hyper => TypeRef::Hyper,
        ast::Type::UnsignedHyper => TypeRef::UnsignedHyper,
        ast::Type::Float => TypeRef::Float,
        ast::Type::Double => TypeRef::Double,
        ast::Type::Bool => TypeRef::Bool,
        ast::Type::OpaqueFixed(size) => TypeRef::OpaqueFixed {
            size: resolve_size(size, const_values),
        },
        ast::Type::OpaqueVar(max_size) => TypeRef::OpaqueVar {
            max_size: max_size.as_ref().map(|s| resolve_size(s, const_values)),
        },
        ast::Type::String(max_size) => TypeRef::String {
            max_size: max_size.as_ref().map(|s| resolve_size(s, const_values)),
        },
        ast::Type::Ident(ident) => TypeRef::Ref {
            name: ident.clone(),
        },
        ast::Type::Array { element_type, size } => TypeRef::Array {
            element: Box::new(convert_type(element_type, const_values)),
            count: resolve_size(size, const_values),
        },
        ast::Type::VarArray { element_type, max_size } => TypeRef::VarArray {
            element: Box::new(convert_type(element_type, const_values)),
            max_count: max_size.as_ref().map(|s| resolve_size(s, const_values)),
        },
        ast::Type::Optional(element_type) => TypeRef::Optional {
            element: Box::new(convert_type(element_type, const_values)),
        },
    }
}

/// Resolve a size to a u64, looking up named constants.
fn resolve_size(size: &ast::Size, const_values: &HashMap<String, i64>) -> u64 {
    match size {
        ast::Size::Literal(literal) => u64::from(*literal),
        ast::Size::Named(named) => {
            let v = *const_values
                .get(named)
                .unwrap_or_else(|| panic!("unresolved size constant '{named}'"));
            u64::try_from(v).unwrap_or_else(|_| panic!("size constant '{named}' is negative: {v}"))
        }
    }
}
