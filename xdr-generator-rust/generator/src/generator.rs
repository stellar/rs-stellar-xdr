use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{case_value, field_name, source_comment, type_name};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, DefinitionOutput, EnumOutput, EnumStructMemberOutput, GeneratedTemplate,
    StructMemberOutput, StructOutput, TypeEnumOutput, TypedefAliasOutput, TypedefNewtypeOutput,
    UnionArmOutput, UnionOutput,
};
use crate::types::{
    base_type_ref, element_type, read_call_type, serde_as_type, size_to_string, type_ref,
};

pub struct RustGenerator {
    options: RustOptions,
    type_info: TypeInfo,
}

impl RustGenerator {
    pub fn new(spec: &XdrSpec, options: RustOptions) -> Self {
        let type_info = TypeInfo::build(spec, &type_name);
        Self { options, type_info }
    }

    /// Generate Rust code from the spec and write it to the output file.
    pub fn generate_to_file(
        &self,
        spec: &XdrSpec,
        output: &std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header = include_str!("../header.rs");
        let template = self.generate(spec, header);
        let rendered = template.render()?;
        std::fs::write(output, rendered)?;
        Ok(())
    }

    /// Generate output for the entire spec.
    pub fn generate(&self, spec: &XdrSpec, header: &str) -> GeneratedTemplate {
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let mut definitions: Vec<DefinitionOutput> = Vec::new();

        for def in spec.all_definitions() {
            let output = self.generate_definition(def);
            definitions.push(output);
        }

        let types: Vec<String> = spec
            .type_names_parent_first()
            .iter()
            .map(|name| type_name(name))
            .collect();

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
        let name = type_name(&s.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let members: Vec<StructMemberOutput> = s
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
            source_comment: source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            member_names,
        }
    }

    fn generate_enum(&self, e: &Enum) -> EnumOutput {
        let name = type_name(&e.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let members: Vec<EnumStructMemberOutput> = e
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| EnumStructMemberOutput {
                name: type_name(&m.stripped_name),
                value: m.value,
                is_first: i == 0,
            })
            .collect();

        EnumOutput {
            name,
            source_comment: source_comment(&e.source, "Enum"),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
        }
    }

    fn generate_union(&self, u: &Union) -> UnionOutput {
        let name = type_name(&u.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let discriminant_type = type_ref(&u.discriminant.type_, None, &self.type_info);
        let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
            || matches!(&u.discriminant.type_, xdr_parser::ast::Type::Ident(n) if {
                self.type_info.definitions.get(&type_name(n))
                    .map(|d| matches!(d, Definition::Typedef(t) if is_builtin_type(&t.type_)))
                    .unwrap_or(false)
            });

        let discriminant_prefix = if !discriminant_is_builtin {
            self.type_info
                .discriminant_enum(&u.discriminant.type_)
                .map(|e| e.member_prefix.clone())
                .unwrap_or_default()
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
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            first_arm_case_name,
            first_arm_type,
        }
    }

    fn generate_typedef(&self, t: &Typedef) -> DefinitionOutput {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                source_comment: source_comment(&t.source, "Typedef"),
                type_ref: base_type_ref(&t.type_, None),
            });
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let no_display_fromstr = self.options.no_display_fromstr.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        let tr = type_ref(&t.type_, None, &self.type_info);
        let rc = read_call_type(&t.type_, None, &self.type_info);
        let serde_as = if custom_str {
            None
        } else {
            serde_as_type(&t.type_, &self.type_info)
        };

        let elem_type = element_type(&t.type_, &self.type_info);
        let size = match &t.type_ {
            xdr_parser::ast::Type::OpaqueFixed(s)
            | xdr_parser::ast::Type::Array { size: s, .. } => Some(size_to_string(s)),
            _ => None,
        };

        DefinitionOutput::TypedefNewtype(TypedefNewtypeOutput {
            name,
            source_comment: source_comment(&t.source, "Typedef"),
            has_default: !custom_default,
            is_var_array: is_var_array_type,
            is_fixed_opaque: is_fixed_opaque_type,
            is_fixed_array: is_fixed_array_type,
            is_custom_str: custom_str,
            type_ref: tr,
            read_call: rc,
            serde_as_type: serde_as,
            element_type: elem_type,
            size,
            custom_debug: is_fixed_opaque_type,
            custom_display_fromstr: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            custom_schemars: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
        })
    }

    fn generate_const(&self, c: &Const) -> ConstOutput {
        let value_str = match c.base {
            IntBase::Hexadecimal => format!("0x{:X}", c.value),
            IntBase::Decimal => c.value.to_string(),
        };
        ConstOutput {
            name: field_name(&c.name).to_uppercase(),
            doc_name: type_name(&c.name),
            source_comment: source_comment(&c.source, "Const"),
            value_str,
        }
    }

    fn generate_member(
        &self,
        m: &StructMember,
        parent: &str,
        custom_str: bool,
    ) -> StructMemberOutput {
        let name = field_name(&m.name);
        let tr = type_ref(&m.type_, Some(parent), &self.type_info);
        let rc = read_call_type(&m.type_, Some(parent), &self.type_info);
        let serde_as = if custom_str {
            None
        } else {
            serde_as_type(&m.type_, &self.type_info)
        };

        StructMemberOutput {
            name,
            type_ref: tr,
            read_call: rc,
            serde_as_type: serde_as,
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
                let (case_name, case_value_expr) = case_value(
                    discriminant_type,
                    discriminant_is_builtin,
                    &case.value,
                    discriminant_prefix,
                );

                let (tr, rc, serde_as) = if let Some(t) = &arm.type_ {
                    let tr = type_ref(t, Some(parent), &self.type_info);
                    let rc = read_call_type(t, Some(parent), &self.type_info);
                    let serde_as = if custom_str {
                        None
                    } else {
                        serde_as_type(t, &self.type_info)
                    };
                    (Some(tr), Some(rc), serde_as)
                } else {
                    (None, None, None)
                };

                UnionArmOutput {
                    case_name,
                    case_value: case_value_expr,
                    is_void: arm.type_.is_none(),
                    type_ref: tr,
                    read_call: rc,
                    serde_as_type: serde_as,
                }
            })
            .collect()
    }
}
