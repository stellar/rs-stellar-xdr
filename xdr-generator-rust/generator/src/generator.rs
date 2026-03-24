use std::collections::hash_map::Entry;
use std::collections::HashMap;

use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{case_value, field_name, mod_name, source_comment, type_name};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, DefinitionOutput, DefinitionTemplate, EnumOutput, EnumStructMemberOutput,
    GeneratedTemplate, ModTemplate, ModuleEntry, StructMemberOutput, StructOutput, TypeEnumEntry,
    TypeEnumOutput, TypedefAliasOutput, TypedefNewtypeOutput, UnionArmOutput, UnionOutput,
};
use crate::types::{base_type_ref, resolve_type, size_to_string, type_ref};

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
    #[allow(dead_code)]
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

    /// Generate Rust code from the spec and write each definition to its own
    /// file inside `output_dir`, plus a module file that ties them together.
    ///
    /// `module_file` is the path to the module file (e.g. `src/generated.rs`)
    /// and `output_dir` is the directory for per-type files (e.g. `src/generated/`).
    pub fn generate_to_dir(
        &self,
        spec: &XdrSpec,
        module_file: &std::path::Path,
        output_dir: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header = include_str!("../header.rs");
        let (modules, definitions) = self.generate_modules(spec);

        // Ensure the output directory exists.
        std::fs::create_dir_all(output_dir)?;

        // Write each definition (or group of definitions) to its own file.
        for (module, defs) in modules.iter().zip(definitions.into_iter()) {
            let template = DefinitionTemplate { definitions: defs };
            let rendered = template.render()?;
            let file_path = output_dir.join(format!("{}.rs", module.mod_name));
            std::fs::write(&file_path, &rendered)?;
        }

        // Write module file.
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let type_variant_enum = self.generate_type_enum(spec);

        let mod_template = ModTemplate {
            xdr_files_sha256,
            header: header.to_string(),
            modules,
            type_variant_enum,
        };
        let rendered = mod_template.render()?;
        std::fs::write(module_file, &rendered)?;

        Ok(())
    }

    /// Generate module entries and grouped definitions for per-file output.
    ///
    /// When the same type name appears in multiple `#ifdef`/`#else` branches,
    /// definitions are grouped into the same file.
    fn generate_modules(&self, spec: &XdrSpec) -> (Vec<ModuleEntry>, Vec<Vec<DefinitionOutput>>) {
        let mut order: Vec<String> = Vec::new();
        let mut grouped: HashMap<String, Vec<DefinitionOutput>> = HashMap::new();

        for def in spec.all_definitions() {
            let m = match def {
                Definition::Const(c) => mod_name(&c.name),
                _ => mod_name(def.name()),
            };

            let output = self.generate_definition(def);
            match grouped.entry(m.clone()) {
                Entry::Vacant(e) => {
                    order.push(m);
                    e.insert(vec![output]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(output);
                }
            }
        }

        let mut modules = Vec::new();
        let mut definitions = Vec::new();
        for m in order {
            let defs = grouped.remove(&m).unwrap();
            modules.push(ModuleEntry { mod_name: m });
            definitions.push(defs);
        }

        (modules, definitions)
    }

    /// Generate the TypeEnumOutput for the type variant enum.
    fn generate_type_enum(&self, spec: &XdrSpec) -> TypeEnumOutput {
        let mut cfg_by_name: HashMap<String, Option<String>> = HashMap::new();

        for def in spec.all_definitions() {
            if !matches!(def, Definition::Const(_)) {
                let name = type_name(def.name());
                let cfg = self.resolve_cfg(def);
                match cfg_by_name.entry(name) {
                    Entry::Vacant(e) => {
                        e.insert(cfg);
                    }
                    Entry::Occupied(mut e) => {
                        e.insert(None);
                    }
                }
            }
        }

        let types: Vec<TypeEnumEntry> = spec
            .type_names_parent_first()
            .iter()
            .map(|name| {
                let rust_name = type_name(name);
                let cfg = cfg_by_name.get(&rust_name).cloned().flatten();
                TypeEnumEntry {
                    name: rust_name,
                    cfg,
                }
            })
            .collect();

        TypeEnumOutput { types }
    }

    /// Generate output for the entire spec.
    #[allow(dead_code)]
    pub fn generate(&self, spec: &XdrSpec, header: &str) -> GeneratedTemplate {
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let mut definitions: Vec<DefinitionOutput> = Vec::new();
        let mut cfg_by_name: HashMap<String, Option<String>> = HashMap::new();

        for def in spec.all_definitions() {
            // Build cfg_by_name for type enum entries in the same pass.
            if !matches!(def, Definition::Const(_)) {
                let name = type_name(def.name());
                let cfg = self.resolve_cfg(def);
                match cfg_by_name.entry(name) {
                    Entry::Vacant(e) => {
                        e.insert(cfg);
                    }
                    Entry::Occupied(mut e) => {
                        // Same name in multiple cfg branches (e.g. #ifdef/#else)
                        // means the type is always present, so clear the cfg.
                        e.insert(None);
                    }
                }
            }

            let output = self.generate_definition(def);
            definitions.push(output);
        }

        let types: Vec<TypeEnumEntry> = spec
            .type_names_parent_first()
            .iter()
            .map(|name| {
                let rust_name = type_name(name);
                let cfg = cfg_by_name.get(&rust_name).cloned().flatten();
                TypeEnumEntry {
                    name: rust_name,
                    cfg,
                }
            })
            .collect();

        GeneratedTemplate {
            xdr_files_sha256,
            header: header.to_string(),
            definitions,
            type_variant_enum: TypeEnumOutput { types },
        }
    }

    /// Resolve the cfg expression for a definition, rendered as a string.
    ///
    /// This is where additional cfg conditions (e.g. file-based cfg derived
    /// from `def.file_index()`) should be combined with the `#ifdef`-derived
    /// cfg before rendering. Use `CfgExpr::and()` to combine them.
    fn resolve_cfg(&self, def: &Definition) -> Option<String> {
        def.cfg().map(|c| c.render())
    }

    fn generate_definition(&self, def: &Definition) -> DefinitionOutput {
        let cfg = self.resolve_cfg(def);
        match def {
            Definition::Struct(s) => DefinitionOutput::Struct(self.generate_struct(s, cfg)),
            Definition::Enum(e) => DefinitionOutput::Enum(self.generate_enum(e, cfg)),
            Definition::Union(u) => DefinitionOutput::Union(self.generate_union(u, cfg)),
            Definition::Typedef(t) => self.generate_typedef(t, cfg),
            Definition::Const(c) => DefinitionOutput::Const(self.generate_const(c, cfg)),
        }
    }

    fn generate_struct(&self, s: &Struct, cfg: Option<String>) -> StructOutput {
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
            cfg,
        }
    }

    fn generate_enum(&self, e: &Enum, cfg: Option<String>) -> EnumOutput {
        let name = type_name(&e.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let first_uncfg_index = e.members.iter().position(|m| m.cfg.is_none()).unwrap_or(0);
        let members: Vec<EnumStructMemberOutput> = e
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| EnumStructMemberOutput {
                name: type_name(&m.stripped_name),
                value: m.value,
                is_default: i == first_uncfg_index,
                cfg: m.cfg.as_ref().map(|c| c.render()),
            })
            .collect();

        EnumOutput {
            name,
            source_comment: source_comment(&e.source, "Enum"),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            cfg,
        }
    }

    fn generate_union(&self, u: &Union, cfg: Option<String>) -> UnionOutput {
        let name = type_name(&u.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let discriminant_type = type_ref(&u.discriminant.type_, None, &self.type_info);
        let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
            || self.type_info.resolve_typedef_to_builtin(&u.discriminant.type_).is_some();

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

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };
        let default_arm_cfg = u
            .arms
            .first()
            .and_then(|a| a.cfg.as_ref().map(|c| c.render()));

        UnionOutput {
            name,
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            cfg,
            default_arm_cfg,
        }
    }

    fn generate_typedef(&self, t: &Typedef, cfg: Option<String>) -> DefinitionOutput {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            return DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                name,
                source_comment: source_comment(&t.source, "Typedef"),
                type_ref: base_type_ref(&t.type_, None),
                cfg,
            });
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let no_display_fromstr = self.options.no_display_fromstr.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        let resolved = resolve_type(&t.type_, None, &self.type_info, custom_str);

        let size = match &t.type_ {
            xdr_parser::ast::Type::OpaqueFixed { size: s }
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
            type_ref: resolved.type_ref,
            turbofish_type: resolved.turbofish_type,
            serde_as_type: resolved.serde_as_type,
            element_type: resolved.element_type,
            size,
            custom_debug: is_fixed_opaque_type,
            custom_display_fromstr: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            custom_schemars: is_fixed_opaque_type && !custom_str && !no_display_fromstr,
            cfg,
        })
    }

    fn generate_const(&self, c: &Const, cfg: Option<String>) -> ConstOutput {
        let value_str = match c.base {
            IntBase::Hexadecimal => format!("0x{:X}", c.value),
            IntBase::Decimal => c.value.to_string(),
        };
        ConstOutput {
            name: field_name(&c.name).to_uppercase(),
            doc_name: type_name(&c.name),
            source_comment: source_comment(&c.source, "Const"),
            value_str,
            cfg,
        }
    }

    fn generate_member(
        &self,
        m: &StructMember,
        parent: &str,
        custom_str: bool,
    ) -> StructMemberOutput {
        let name = field_name(&m.name);
        let resolved = resolve_type(&m.type_, Some(parent), &self.type_info, custom_str);

        StructMemberOutput {
            name,
            type_ref: resolved.type_ref,
            turbofish_type: resolved.turbofish_type,
            serde_as_type: resolved.serde_as_type,
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

                let resolved = arm
                    .type_
                    .as_ref()
                    .map(|t| resolve_type(t, Some(parent), &self.type_info, custom_str));

                UnionArmOutput {
                    case_name,
                    case_value: case_value_expr,
                    is_void: arm.type_.is_none(),
                    type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                    turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                    serde_as_type: resolved.and_then(|r| r.serde_as_type),
                    cfg: arm.cfg.as_ref().map(|c| c.render()),
                }
            })
            .collect()
    }
}
