//! Code generator that prepares data for templates and renders output.

use crate::ast::{
    CaseValue, Const, Definition, Enum, Member, Size, Struct, Type, Typedef, Union, UnionArm,
    XdrSpec,
};
use crate::types::{
    base_rust_type_ref, element_type_for_vec, is_builtin_type, is_fixed_array, is_fixed_opaque,
    is_var_array, rust_field_name, rust_read_call_type, rust_type_name, rust_type_ref,
    serde_field_attr, Options, TypeInfo,
};
use askama::Template;
use sha2::{Digest, Sha256};

/// Data for rendering the main generated file.
#[derive(Template)]
#[template(path = "generated.rs.jinja", escape = "none")]
pub struct GeneratedTemplate {
    pub xdr_files_sha256: Vec<(String, String)>,
    pub header: String,
    pub definitions: Vec<DefinitionOutput>,
    pub type_variant_enum: TypeEnumOutput,
}

/// Output for a single definition.
pub enum DefinitionOutput {
    Struct(StructOutput),
    Enum(EnumOutput),
    Union(UnionOutput),
    TypedefAlias(TypedefAliasOutput),
    TypedefNewtype(TypedefNewtypeOutput),
    Const(ConstOutput),
}

/// Data for rendering a struct.
pub struct StructOutput {
    pub name: String,
    pub source_comment: String,
    pub derive_attrs: String,
    pub serde_attrs: String,
    pub schemars_attr: Option<String>,
    pub members: Vec<MemberOutput>,
    pub custom_deserialize_impl: Option<String>,
}

pub struct MemberOutput {
    pub name: String,
    pub type_ref: String,
    pub read_call: String,
    pub serde_attr: Option<String>,
}

/// Data for rendering an enum.
pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub derive_attrs: String,
    pub serde_attrs: String,
    pub schemars_attr: Option<String>,
    pub members: Vec<EnumMemberOutput>,
}

pub struct EnumMemberOutput {
    pub name: String,
    pub value: i32,
    pub is_first: bool,
}

/// Data for rendering a union.
pub struct UnionOutput {
    pub name: String,
    pub source_comment: String,
    pub derive_attrs: String,
    pub serde_attrs: String,
    pub schemars_attr: Option<String>,
    pub discriminant_type: String,
    pub discriminant_is_builtin: bool,
    pub arms: Vec<UnionArmOutput>,
    pub default_impl: Option<String>,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub read_call: Option<String>,
}

/// Data for a typedef that's a simple type alias.
pub struct TypedefAliasOutput {
    pub name: String,
    pub type_ref: String,
}

/// Data for a typedef that's a newtype struct.
pub struct TypedefNewtypeOutput {
    pub name: String,
    pub source_comment: String,
    pub derive_attrs: String,
    pub serde_attrs: String,
    pub schemars_attr: Option<String>,
    pub type_ref: String,
    pub read_call: String,
    pub serde_field_attr: Option<String>,
    pub is_fixed_opaque: bool,
    pub is_fixed_array: bool,
    pub is_var_array: bool,
    pub element_type: String,
    pub size: Option<String>,
    pub custom_debug: bool,
    pub custom_display_fromstr: bool,
    pub custom_schemars: bool,
}

/// Data for a const.
pub struct ConstOutput {
    pub name: String,
    pub value: i64,
}

/// Data for the TypeVariant and Type enums.
pub struct TypeEnumOutput {
    pub types: Vec<String>,
}

/// Generator for producing Rust code from XDR specs.
pub struct Generator {
    pub options: Options,
    pub type_info: TypeInfo,
}

impl Generator {
    pub fn new(spec: &XdrSpec, options: Options) -> Self {
        let type_info = TypeInfo::build(spec);
        Self { options, type_info }
    }

    /// Generate output for the entire spec.
    pub fn generate(
        &self,
        spec: &XdrSpec,
        input_files: &[(String, String)], // (path, content)
        header: &str,
    ) -> GeneratedTemplate {
        // Compute SHA256 hashes for input files
        let xdr_files_sha256: Vec<(String, String)> = input_files
            .iter()
            .map(|(path, content)| {
                let mut hasher = Sha256::new();
                hasher.update(content.as_bytes());
                let hash = format!("{:x}", hasher.finalize());
                (path.clone(), hash)
            })
            .collect();

        // Collect all type names for the type enum
        let mut types: Vec<String> = Vec::new();
        let mut definitions: Vec<DefinitionOutput> = Vec::new();

        for def in spec.all_definitions() {
            let name = rust_type_name(def.name());

            // Only add non-const types to the type enum
            if !matches!(def, Definition::Const(_)) {
                types.push(name.clone());
            }

            let output = self.generate_definition(def);
            definitions.push(output);
        }

        GeneratedTemplate {
            xdr_files_sha256,
            header: header.to_string(),
            definitions,
            type_variant_enum: TypeEnumOutput { types },
        }
    }

    fn generate_definition(&self, def: &Definition) -> DefinitionOutput {
        match def {
            Definition::Struct(s) => DefinitionOutput::Struct(self.generate_struct(s)),
            Definition::Enum(e) => DefinitionOutput::Enum(self.generate_enum(e)),
            Definition::Union(u) => DefinitionOutput::Union(self.generate_union(u)),
            Definition::Typedef(t) => self.generate_typedef(t),
            Definition::Const(c) => DefinitionOutput::Const(self.generate_const(c)),
        }
    }

    fn generate_struct(&self, s: &Struct) -> StructOutput {
        let name = rust_type_name(&s.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let derive_attrs = if custom_default {
            "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]".to_string()
        } else {
            r#"#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"#
                .to_string()
        };

        let serde_attrs = if custom_str {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay))]"#.to_string()
        } else {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]"#.to_string()
        };

        let schemars_attr = if custom_str {
            None
        } else {
            Some(r#"#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]"#.to_string())
        };

        let members: Vec<MemberOutput> = s
            .members
            .iter()
            .map(|m| self.generate_member(m, &name, custom_str))
            .collect();

        let custom_deserialize_impl = if custom_str {
            Some(self.generate_custom_deserialize(&name, &s.members))
        } else {
            None
        };

        StructOutput {
            name,
            source_comment: format_source_comment(&s.source, "Struct"),
            derive_attrs,
            serde_attrs,
            schemars_attr,
            members,
            custom_deserialize_impl,
        }
    }

    fn generate_member(&self, m: &Member, parent: &str, custom_str: bool) -> MemberOutput {
        let name = rust_field_name(&m.name);
        let type_ref = rust_type_ref(&m.type_, Some(parent), &self.type_info);
        let read_call = rust_read_call_type(&m.type_, Some(parent), &self.type_info);
        let serde_attr = if custom_str {
            None
        } else {
            serde_field_attr(&m.type_)
        };

        MemberOutput {
            name,
            type_ref,
            read_call,
            serde_attr,
        }
    }

    fn generate_custom_deserialize(&self, name: &str, members: &[Member]) -> String {
        let fields: Vec<String> = members
            .iter()
            .map(|m| {
                let field_name = rust_field_name(&m.name);
                let type_ref = rust_type_ref(&m.type_, Some(name), &self.type_info);
                format!("                {field_name}: {type_ref},")
            })
            .collect();

        let field_names: Vec<String> = members.iter().map(|m| rust_field_name(&m.name)).collect();

        format!(
            r#"#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {{
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct {name} {{
{fields}
        }}
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum {name}OrString<'a> {{
            Str(&'a str),
            String(String),
            {name}({name}),
        }}
        match {name}OrString::deserialize(deserializer)? {{
            {name}OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            {name}OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            {name}OrString::{name}({name} {{
                {field_list}
            }}) => Ok(self::{name} {{
                {field_list}
            }}),
        }}
    }}
}}"#,
            name = name,
            fields = fields.join("\n"),
            field_list = field_names.join(", ") + ","
        )
    }

    fn generate_enum(&self, e: &Enum) -> EnumOutput {
        let name = rust_type_name(&e.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let derive_attrs = if custom_default {
            "#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]".to_string()
        } else {
            r#"#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"#
                .to_string()
        };

        let serde_attrs = if custom_str {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]"#.to_string()
        } else {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]"#.to_string()
        };

        let schemars_attr = if custom_str {
            None
        } else {
            Some(r#"#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]"#.to_string())
        };

        let members: Vec<EnumMemberOutput> = e
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| EnumMemberOutput {
                name: rust_type_name(&m.name),
                value: m.value,
                is_first: i == 0,
            })
            .collect();

        EnumOutput {
            name,
            source_comment: format_source_comment(&e.source, "Enum"),
            derive_attrs,
            serde_attrs,
            schemars_attr,
            members,
        }
    }

    fn generate_union(&self, u: &Union) -> UnionOutput {
        let name = rust_type_name(&u.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let discriminant_type = rust_type_ref(&u.discriminant.type_, None, &self.type_info);
        let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
            || matches!(&u.discriminant.type_, Type::Ident(n) if {
                // Check if it resolves to a builtin
                self.type_info.definitions.get(&rust_type_name(n))
                    .map(|d| matches!(d, Definition::Typedef(t) if is_builtin_type(&t.type_)))
                    .unwrap_or(false)
            });

        let derive_attrs =
            "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]".to_string();

        let serde_attrs = if custom_str {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]"#.to_string()
        } else {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]"#.to_string()
        };

        let schemars_attr = if custom_str {
            None
        } else {
            Some(r#"#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]"#.to_string())
        };

        let arms: Vec<UnionArmOutput> = u
            .arms
            .iter()
            .flat_map(|arm| {
                self.generate_union_arm(arm, &name, &discriminant_type, discriminant_is_builtin)
            })
            .collect();

        let default_impl = if !custom_default && !arms.is_empty() {
            let first_arm = &arms[0];
            if first_arm.is_void {
                Some(format!(
                    r#"#[cfg(feature = "alloc")]
impl Default for {name} {{
    fn default() -> Self {{
        Self::{}
    }}
}}"#,
                    first_arm.case_name
                ))
            } else {
                Some(format!(
                    r#"#[cfg(feature = "alloc")]
impl Default for {name} {{
    fn default() -> Self {{
        Self::{}({}::default())
    }}
}}"#,
                    first_arm.case_name,
                    first_arm.read_call.as_ref().unwrap()
                ))
            }
        } else {
            None
        };

        UnionOutput {
            name,
            source_comment: format_source_comment(&u.source, "Union"),
            derive_attrs,
            serde_attrs,
            schemars_attr,
            discriminant_type,
            discriminant_is_builtin,
            arms,
            default_impl,
        }
    }

    fn generate_union_arm(
        &self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
    ) -> Vec<UnionArmOutput> {
        arm.cases
            .iter()
            .map(|case| {
                let (case_name, case_value) = match &case.value {
                    CaseValue::Ident(name) => {
                        let rust_name = rust_type_name(name);
                        let value = if discriminant_is_builtin {
                            rust_name.clone()
                        } else {
                            format!("{discriminant_type}::{rust_name}")
                        };
                        (rust_name, value)
                    }
                    CaseValue::Literal(n) => {
                        let case_name = format!("V{n}");
                        let value = if discriminant_is_builtin {
                            n.to_string()
                        } else {
                            format!("{discriminant_type}({n})")
                        };
                        (case_name, value)
                    }
                };

                let (type_ref, read_call) = if let Some(t) = &arm.type_ {
                    let type_ref = rust_type_ref(t, Some(parent), &self.type_info);
                    let read_call = rust_read_call_type(t, Some(parent), &self.type_info);
                    (Some(type_ref), Some(read_call))
                } else {
                    (None, None)
                };

                UnionArmOutput {
                    case_name,
                    case_value,
                    is_void: arm.type_.is_none(),
                    type_ref,
                    read_call,
                }
            })
            .collect()
    }

    fn generate_typedef(&self, t: &Typedef) -> DefinitionOutput {
        let name = rust_type_name(&t.name);

        // Simple type alias for builtins
        if is_builtin_type(&t.type_) {
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                type_ref: base_rust_type_ref(&t.type_),
            });
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        // Derive attributes
        let derive_attrs = if custom_default {
            if is_var_array_type {
                "#[derive(Default)]".to_string()
            } else {
                String::new()
            }
        } else if is_var_array_type {
            "#[derive(Default)]".to_string()
        } else {
            r#"#[cfg_attr(feature = "alloc", derive(Default))]"#.to_string()
        };

        let base_derives = "#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]";
        let derive_attrs = if derive_attrs.is_empty() {
            base_derives.to_string()
        } else {
            format!("{derive_attrs}\n{base_derives}")
        };

        // Add Debug unless it's fixed opaque (custom Debug)
        let derive_attrs = if is_fixed_opaque_type {
            derive_attrs
        } else {
            format!("{derive_attrs}\n#[derive(Debug)]")
        };

        // Serde attributes
        let serde_attrs = if is_fixed_opaque_type || custom_str {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]"#.to_string()
        } else {
            r#"#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]"#.to_string()
        };

        // Schemars
        let schemars_attr = if is_fixed_opaque_type || custom_str {
            None
        } else {
            Some(r#"#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]"#.to_string())
        };

        let type_ref = rust_type_ref(&t.type_, None, &self.type_info);
        let read_call = rust_read_call_type(&t.type_, None, &self.type_info);
        let serde_field_attr = if custom_str {
            None
        } else {
            serde_field_attr(&t.type_)
        };

        let element_type = element_type_for_vec(&t.type_);
        let size = match &t.type_ {
            Type::OpaqueFixed(s) | Type::Array { size: s, .. } => Some(size_to_string(s)),
            _ => None,
        };

        DefinitionOutput::TypedefNewtype(TypedefNewtypeOutput {
            name,
            source_comment: format_source_comment(&t.source, "Typedef"),
            derive_attrs,
            serde_attrs,
            schemars_attr,
            type_ref,
            read_call,
            serde_field_attr,
            is_fixed_opaque: is_fixed_opaque_type,
            is_fixed_array: is_fixed_array_type,
            is_var_array: is_var_array_type,
            element_type,
            size,
            custom_debug: is_fixed_opaque_type,
            custom_display_fromstr: is_fixed_opaque_type && !custom_str,
            custom_schemars: is_fixed_opaque_type && !custom_str,
        })
    }

    fn generate_const(&self, c: &Const) -> ConstOutput {
        ConstOutput {
            name: rust_field_name(&c.name).to_uppercase(),
            value: c.value,
        }
    }
}

fn format_source_comment(source: &str, kind: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let lines: Vec<&str> = source.lines().collect();
    let formatted: Vec<String> = lines.iter().map(|l| format!("/// {l}")).collect();
    format!(
        "/// is an XDR {kind} defined as:\n///\n/// ```text\n{}\n/// ```\n///",
        formatted.join("\n")
    )
}

fn size_to_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => rust_type_name(name),
    }
}
