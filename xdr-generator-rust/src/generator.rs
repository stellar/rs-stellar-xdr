//! Code generator that prepares data for templates and renders output.

use crate::ast::{
    CaseValue, Const, Definition, Enum, IfdefCondition, Member, Size, Struct, Type, Typedef, Union,
    UnionArm, XdrSpec,
};
use crate::lexer::IntBase;
use crate::types::{
    base_rust_type_ref, element_type_for_vec, is_builtin_type, is_fixed_array, is_fixed_opaque,
    is_var_array, rust_field_name, rust_read_call_type, rust_type_name, rust_type_ref,
    serde_as_type, Options, TypeInfo,
};
use askama::Template;
use heck::ToSnakeCase;
use sha2::{Digest, Sha256};

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

        // Collect all type names for the type enum.
        // The types list needs to be in "parent before nested" order for TypeVariant,
        // but definitions stay in "nested before parent" order for file output.
        //
        // Build parent-child relationships and then traverse parent-first.
        use std::collections::HashMap;

        // First pass: collect definitions and build parent-child map
        let mut definitions: Vec<DefinitionOutput> = Vec::new();
        let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
        // (name, parent, feature)
        let mut all_type_names: Vec<(String, Option<String>, Option<String>)> = Vec::new();

        for def in spec.all_definitions() {
            let name = rust_type_name(def.name());

            // Only add non-const types to the type enum
            if !matches!(def, Definition::Const(_)) {
                let parent = def.parent().map(|p| rust_type_name(p));
                let feature = ifdefs_to_cfg(def.ifdefs());
                all_type_names.push((name.clone(), parent.clone(), feature));

                if let Some(ref parent_name) = parent {
                    children_of
                        .entry(parent_name.clone())
                        .or_default()
                        .push(name.clone());
                }
            }

            let output = self.generate_definition(def);
            definitions.push(output);
        }

        // Second pass: build types list with parent-before-children ordering
        // Use a recursive helper to add each type and then its children
        let mut types: Vec<TypeEnumEntry> = Vec::new();
        let mut added: std::collections::HashSet<String> = std::collections::HashSet::new();

        // Build a map from name -> feature for lookup.
        // If a name appears with different features (e.g., ifdef/else), both
        // definitions are always available, so no cfg gate is needed.
        let mut feature_map: HashMap<String, Option<String>> = HashMap::new();
        for (n, _, f) in &all_type_names {
            match feature_map.get(n) {
                None => {
                    feature_map.insert(n.clone(), f.clone());
                }
                Some(existing) if existing != f => {
                    // Different features on the same type name means it's always present
                    feature_map.insert(n.clone(), None);
                }
                _ => {}
            }
        }

        fn add_type_and_children(
            name: &str,
            types: &mut Vec<TypeEnumEntry>,
            added: &mut std::collections::HashSet<String>,
            children_of: &HashMap<String, Vec<String>>,
            feature_map: &HashMap<String, Option<String>>,
        ) {
            if added.contains(name) {
                return;
            }
            added.insert(name.to_string());
            types.push(TypeEnumEntry {
                name: name.to_string(),
                feature: feature_map.get(name).cloned().flatten(),
            });

            // Add children in the order they appear in children_of
            if let Some(children) = children_of.get(name) {
                for child in children {
                    add_type_and_children(child, types, added, children_of, feature_map);
                }
            }
        }

        // Process types in definition order, but only add root types (no parent)
        // and let recursion handle adding children
        for (name, parent, _) in &all_type_names {
            if parent.is_none() {
                add_type_and_children(name, &mut types, &mut added, &children_of, &feature_map);
            }
        }

        // Add any remaining types that weren't reached (shouldn't happen, but just in case)
        for (name, _, _) in &all_type_names {
            if !added.contains(name) {
                types.push(TypeEnumEntry {
                    name: name.clone(),
                    feature: feature_map.get(name).cloned().flatten(),
                });
            }
        }

        let namespace = spec
            .namespaces
            .first()
            .map(|ns| ns.name.clone())
            .unwrap_or_default();

        GeneratedTemplate {
            namespace,
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

        let members: Vec<MemberOutput> = s
            .members
            .iter()
            .map(|m| self.generate_member(m, &name, custom_str))
            .collect();

        let member_names: String = members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let type_kind = if s.is_nested {
            "NestedStruct"
        } else {
            "Struct"
        };
        StructOutput {
            name,
            source_comment: format_source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            member_names,
            feature: ifdefs_to_cfg(&s.ifdefs),
        }
    }

    fn generate_enum(&self, e: &Enum) -> EnumOutput {
        let name = rust_type_name(&e.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        // Find common prefix to strip from member names
        let member_names: Vec<&str> = e.members.iter().map(|m| m.name.as_str()).collect();
        let prefix = find_common_prefix(&member_names);

        let members: Vec<EnumMemberOutput> = e
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let stripped = strip_prefix(&m.name, prefix);
                EnumMemberOutput {
                    name: rust_type_name(&stripped),
                    value: m.value,
                    is_first: i == 0,
                    feature: ifdefs_to_cfg(&m.ifdefs),
                }
            })
            .collect();

        EnumOutput {
            name,
            source_comment: format_source_comment(&e.source, "Enum"),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            feature: ifdefs_to_cfg(&e.ifdefs),
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

        // Find the discriminant enum's common prefix for stripping case names
        let discriminant_prefix = if !discriminant_is_builtin {
            if let Type::Ident(enum_name) = &u.discriminant.type_ {
                if let Some(Definition::Enum(enum_def)) =
                    self.type_info.definitions.get(&rust_type_name(enum_name))
                {
                    let member_names: Vec<&str> =
                        enum_def.members.iter().map(|m| m.name.as_str()).collect();
                    find_common_prefix(&member_names).to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let arms: Vec<UnionArmOutput> = u
            .arms
            .iter()
            .flat_map(|arm| {
                self.generate_union_arm(
                    arm,
                    &name,
                    &discriminant_type,
                    discriminant_is_builtin,
                    &discriminant_prefix,
                    custom_str,
                )
            })
            .collect();

        // For default impl: get the case name and type of the first arm
        let first_arm_case_name = if !arms.is_empty() {
            arms[0].case_name.clone()
        } else {
            String::new()
        };
        let first_arm_type = if !arms.is_empty() && !arms[0].is_void {
            arms[0].read_call.clone()
        } else {
            None
        };

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };

        UnionOutput {
            name,
            source_comment: format_source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            discriminant_is_builtin,
            arms,
            first_arm_case_name,
            first_arm_type,
            feature: ifdefs_to_cfg(&u.ifdefs),
        }
    }

    fn generate_typedef(&self, t: &Typedef) -> DefinitionOutput {
        let name = rust_type_name(&t.name);

        // Simple type alias only for exact primitive type names (Uint64, Int64, etc.)
        // Other typedefs to primitives (like Duration, TimePoint) become newtypes
        let is_primitive_alias = matches!(
            name.as_str(),
            "Uint64" | "Int64" | "Uint32" | "Int32" | "Float" | "Double"
        );
        if is_primitive_alias && is_builtin_type(&t.type_) {
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                source_comment: format_source_comment(&t.source, "Typedef"),
                type_ref: base_rust_type_ref(&t.type_),
                feature: ifdefs_to_cfg(&t.ifdefs),
            });
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let no_display_fromstr = self.options.no_display_fromstr.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        let type_ref = rust_type_ref(&t.type_, None, &self.type_info);
        let read_call = rust_read_call_type(&t.type_, None, &self.type_info);
        let serde_as = if custom_str {
            None
        } else {
            serde_as_type(&t.type_)
        };

        let element_type = element_type_for_vec(&t.type_);
        let size = match &t.type_ {
            Type::OpaqueFixed(s) | Type::Array { size: s, .. } => Some(size_to_string(s)),
            _ => None,
        };

        DefinitionOutput::TypedefNewtype(TypedefNewtypeOutput {
            name,
            source_comment: format_source_comment(&t.source, "Typedef"),
            has_default: !custom_default,
            is_var_array: is_var_array_type,
            is_fixed_opaque: is_fixed_opaque_type,
            is_fixed_array: is_fixed_array_type,
            is_custom_str: custom_str,
            type_ref,
            read_call,
            serde_as_type: serde_as,
            element_type,
            size,
            custom_debug: is_fixed_opaque_type,
            custom_display_fromstr: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            custom_schemars: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            feature: ifdefs_to_cfg(&t.ifdefs),
        })
    }

    fn generate_const(&self, c: &Const) -> ConstOutput {
        let value_str = match c.base {
            IntBase::Hexadecimal => format!("0x{:X}", c.value),
            IntBase::Decimal => c.value.to_string(),
        };
        ConstOutput {
            name: rust_field_name(&c.name).to_uppercase(),
            doc_name: rust_type_name(&c.name),
            source_comment: format_source_comment(&c.source, "Const"),
            value_str,
            feature: ifdefs_to_cfg(&c.ifdefs),
        }
    }

    fn generate_member(&self, m: &Member, parent: &str, custom_str: bool) -> MemberOutput {
        let name = rust_field_name(&m.name);
        let type_ref = rust_type_ref(&m.type_, Some(parent), &self.type_info);
        let read_call = rust_read_call_type(&m.type_, Some(parent), &self.type_info);
        // For custom_str types, we skip serde_as since they use SerializeDisplay
        let serde_as = if custom_str {
            None
        } else {
            serde_as_type(&m.type_)
        };

        MemberOutput {
            name,
            type_ref,
            read_call,
            serde_as_type: serde_as,
            feature: ifdefs_to_cfg(&m.ifdefs),
        }
    }

    fn generate_union_arm(
        &self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        discriminant_prefix: &str,
        custom_str: bool,
    ) -> Vec<UnionArmOutput> {
        arm.cases
            .iter()
            .map(|case| {
                let (case_name, case_value) = match &case.value {
                    CaseValue::Ident(name) => {
                        // Strip common prefix from case name
                        let stripped = strip_prefix(name, discriminant_prefix);
                        let rust_name = rust_type_name(&stripped);
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

                let (type_ref, read_call, serde_as) = if let Some(t) = &arm.type_ {
                    let type_ref = rust_type_ref(t, Some(parent), &self.type_info);
                    let read_call = rust_read_call_type(t, Some(parent), &self.type_info);
                    // Get serde_as type for i64/u64 types (unless custom_str)
                    let serde_as = if custom_str { None } else { serde_as_type(t) };
                    (Some(type_ref), Some(read_call), serde_as)
                } else {
                    (None, None, None)
                };

                UnionArmOutput {
                    case_name,
                    case_value,
                    is_void: arm.type_.is_none(),
                    type_ref,
                    read_call,
                    serde_as_type: serde_as,
                    feature: ifdefs_to_cfg(&arm.ifdefs),
                }
            })
            .collect()
    }
}

/// Data for rendering the main generated file.
#[derive(Template)]
#[template(path = "generated.rs.jinja", escape = "none")]
pub struct GeneratedTemplate {
    pub namespace: String,
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
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<MemberOutput>,
    /// Comma-separated list of member names for destructuring (e.g., "id, ed25519,")
    pub member_names: String,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

pub struct MemberOutput {
    pub name: String,
    pub type_ref: String,
    pub read_call: String,
    /// The serde_as type for i64/u64 fields (e.g., "NumberOrString").
    pub serde_as_type: Option<String>,
    /// The `#[cfg(...)]` condition for this member, if any.
    pub feature: Option<String>,
}

/// Data for rendering an enum.
pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<EnumMemberOutput>,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

pub struct EnumMemberOutput {
    pub name: String,
    pub value: i32,
    pub is_first: bool,
    /// The `#[cfg(...)]` condition for this member, if any.
    pub feature: Option<String>,
}

/// Data for rendering a union.
pub struct UnionOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub discriminant_type: String,
    pub discriminant_is_builtin: bool,
    pub arms: Vec<UnionArmOutput>,
    /// For default impl: case name of the first arm.
    pub first_arm_case_name: String,
    /// For default impl: type to call ::default() on (read_call of first arm, if non-void)
    pub first_arm_type: Option<String>,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub read_call: Option<String>,
    /// The serde_as type for i64/u64 fields (e.g., "NumberOrString").
    pub serde_as_type: Option<String>,
    /// The `#[cfg(...)]` condition for this arm, if any.
    pub feature: Option<String>,
}

/// Data for a typedef that's a simple type alias.
pub struct TypedefAliasOutput {
    pub name: String,
    pub source_comment: String,
    pub type_ref: String,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

/// Data for a typedef that's a newtype struct.
pub struct TypedefNewtypeOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_var_array: bool,
    pub is_fixed_opaque: bool,
    pub is_fixed_array: bool,
    pub is_custom_str: bool,
    pub type_ref: String,
    pub read_call: String,
    /// The serde_as type for i64/u64 fields (e.g., "NumberOrString").
    pub serde_as_type: Option<String>,
    pub element_type: String,
    pub size: Option<String>,
    /// Fixed opaque types have custom Debug impl (hex format).
    pub custom_debug: bool,
    pub custom_display_fromstr: bool,
    pub custom_schemars: bool,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

/// Data for a const.
pub struct ConstOutput {
    /// The Rust const name (SCREAMING_SNAKE_CASE).
    pub name: String,
    /// The name for the doc comment (UpperCamelCase).
    pub doc_name: String,
    pub source_comment: String,
    /// The formatted value string (decimal or hex).
    pub value_str: String,
    /// The `#[cfg(...)]` condition for this definition, if any.
    pub feature: Option<String>,
}

/// Data for the TypeVariant and Type enums.
pub struct TypeEnumOutput {
    pub types: Vec<TypeEnumEntry>,
}

/// An entry in the TypeVariant/Type enum, with optional cfg condition.
pub struct TypeEnumEntry {
    pub name: String,
    /// The `#[cfg(...)]` condition for this type, if any.
    pub feature: Option<String>,
}

/// Convert a list of ifdef conditions to a Rust `#[cfg(...)]` attribute value.
/// Returns None if the list is empty.
fn ifdefs_to_cfg(ifdefs: &[IfdefCondition]) -> Option<String> {
    if ifdefs.is_empty() {
        return None;
    }
    let parts: Vec<String> = ifdefs
        .iter()
        .map(|c| {
            let feature = c.name.to_snake_case();
            if c.negated {
                format!("not(feature = \"{feature}\")")
            } else {
                format!("feature = \"{feature}\"")
            }
        })
        .collect();
    if parts.len() == 1 {
        Some(parts[0].clone())
    } else {
        Some(format!("all({})", parts.join(", ")))
    }
}

fn format_source_comment(source: &str, kind: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    // Filter out empty lines at the start and trim the source
    let trimmed = source.trim();
    let lines: Vec<&str> = trimmed.lines().collect();
    let formatted: Vec<String> = lines.iter().map(|l| format!("/// {l}")).collect();
    // The template outputs `/// {name}` so we start with ` is an XDR` (note the space)
    format!(
        " is an XDR {kind} defined as:\n///\n/// ```text\n{}\n/// ```\n///",
        formatted.join("\n")
    )
}

fn size_to_string(size: &Size) -> String {
    match size {
        Size::Literal(n) => n.to_string(),
        Size::Named(name) => rust_type_name(name),
    }
}

/// Find the common prefix to strip from enum member names.
/// Returns the prefix up to and including the last underscore that is common to all names.
fn find_common_prefix<'a>(names: &[&'a str]) -> &'a str {
    if names.is_empty() || names.len() == 1 {
        return "";
    }

    let first = names[0];

    // Find longest common prefix among all names
    let common_len = names.iter().skip(1).fold(first.len(), |len, name| {
        first
            .bytes()
            .zip(name.bytes())
            .take(len)
            .take_while(|(a, b)| a == b)
            .count()
    });

    let common = &first[..common_len];

    // Find the last underscore in the common prefix
    // If found, strip up to and including the underscore
    if let Some(last_underscore) = common.rfind('_') {
        &first[..=last_underscore]
    } else {
        ""
    }
}

/// Strip common prefix from an enum member name.
/// If the result would start with a digit, keep the first letter of the prefix.
fn strip_prefix(name: &str, prefix: &str) -> String {
    if !prefix.is_empty() && name.starts_with(prefix) {
        let stripped = &name[prefix.len()..];
        // If result starts with digit, keep the first letter of the prefix
        // e.g., "BINARY_FUSE_FILTER_8_BIT" with prefix "BINARY_FUSE_FILTER_" -> "B8_BIT"
        if stripped.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            if let Some(first_char) = prefix.chars().next() {
                return format!("{first_char}{stripped}");
            }
        }
        stripped.to_string()
    } else {
        name.to_string()
    }
}
