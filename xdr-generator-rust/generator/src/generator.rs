use heck::ToSnakeCase;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use askama::Template;
use xdr_parser::ast::{
    Const, Definition, Enum, Struct, StructMember, Type, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{
    case_value, const_name, field_json_rename, field_name, mod_name, source_comment, type_name,
};
use crate::options::RustOptions;
use crate::output::{
    ConstDefinitionOutput, ConstDefinitionTemplate, ConstModEntry, ConstModItem, ConstModTemplate,
    ConstNewtypeOutput, ConstOutput, ConstStructMemberOutput, ConstStructOutput, ConstToXdrTemplate,
    ConstTypeOutput, ConstUnionArmOutput, ConstUnionOutput, ConstWriterMethodOutput,
    ConstWriterOutput, DefinitionOutput, DefinitionTemplate, EnumOutput, EnumStructMemberOutput,
    GeneratedTemplate, ModTemplate, ModuleEntry, StructMemberOutput, StructOutput,
    TypeEnumDefinitionTemplate, TypeEnumEntry, TypeEnumOutput, TypedefAliasOutput,
    TypedefNewtypeOutput, UnionArmOutput, UnionOutput,
};
use crate::types::{base_type_ref, resolve_type, size_to_u32_string, type_ref};

pub struct RustGenerator {
    options: RustOptions,
    type_info: TypeInfo,
    /// Rust type names of generated types that directly or transitively
    /// contain heap-allocated data (`VecM`, `BytesM`, `StringM`, or `Box` for
    /// cyclic references) under some cfg, and therefore have a borrowing form
    /// of their own in the `const` module.
    const_required: HashSet<String>,
}

impl RustGenerator {
    pub fn new(spec: &XdrSpec, options: RustOptions) -> Self {
        let type_info = TypeInfo::build(spec, &type_name);
        let const_required = BorrowAnalysis::build(spec).const_required();
        Self {
            options,
            type_info,
            const_required,
        }
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
        let const_header = include_str!("../const_header.rs");
        let (mut modules, definitions, const_modules) = self.generate_modules(spec);

        // Ensure the output directories exist.
        let const_dir = output_dir.join("const");
        std::fs::create_dir_all(output_dir)?;
        std::fs::create_dir_all(&const_dir)?;

        // All const XDR encoding is emitted as methods on `ConstWriter`, each
        // into the `const` module's file for the type it serializes, so no
        // owned type carries const surface of its own.
        let mut const_methods_by_module: HashMap<String, Vec<ConstWriterMethodOutput>> =
            HashMap::new();
        for m in crate::const_writer::build(spec, &self.type_info, &self.cfg_by_name(spec)).methods {
            const_methods_by_module
                .entry(m.module.clone())
                .or_default()
                .push(m);
        }

        // Write each definition (or group of definitions) to its own file.
        for (module, defs) in modules.iter().zip(definitions.into_iter()) {
            let template = DefinitionTemplate { definitions: defs };
            let rendered = template.render()?;
            let file_path = output_dir.join(format!("{}.rs", module.mod_name));
            std::fs::write(&file_path, &rendered)?;
        }

        // Write the `const` module: a file per definition module that has const
        // content, and a module file naming them and re-exporting the rest.
        let mut const_entries: Vec<ConstModEntry> = Vec::new();
        for const_module in const_modules {
            let methods = const_methods_by_module
                .remove(&const_module.mod_name)
                .unwrap_or_default();
            let has_file = !const_module.definitions.is_empty() || !methods.is_empty();
            if has_file {
                let template = ConstDefinitionTemplate {
                    definitions: const_module.definitions,
                    const_writer: ConstWriterOutput { methods },
                };
                let rendered = template.render()?;
                let file_path = const_dir.join(format!("{}.rs", const_module.mod_name));
                std::fs::write(&file_path, &rendered)?;
            }
            const_entries.push(ConstModEntry {
                mod_name: has_file.then_some(const_module.mod_name),
                items: const_module.items,
            });
        }

        // Every method names the module of the type it serializes, and every
        // definition is grouped into a file under that same name, so the loop
        // above has placed them all.
        assert!(
            const_methods_by_module.is_empty(),
            "const writer methods name modules with no definition file: {:?}",
            const_methods_by_module.keys().collect::<Vec<_>>()
        );

        let const_mod_template = ConstModTemplate {
            header: const_header.to_string(),
            entries: const_entries,
        };
        let rendered = const_mod_template.render()?;
        std::fs::write(output_dir.join("const.rs"), &rendered)?;

        let type_variant_enum = self.generate_type_enum(spec);
        let type_enum_template = TypeEnumDefinitionTemplate { type_variant_enum };
        let rendered = type_enum_template.render()?;
        std::fs::write(output_dir.join("type_enum.rs"), &rendered)?;
        modules.push(ModuleEntry {
            mod_name: "type_enum".to_string(),
        });

        // Write module file.
        let xdr_files_sha256: Vec<(String, String)> = spec
            .files
            .iter()
            .map(|f| (f.name.clone(), f.sha256.clone()))
            .collect();

        let mod_template = ModTemplate {
            xdr_files_sha256,
            header: header.to_string(),
            modules,
        };
        let rendered = mod_template.render()?;
        std::fs::write(module_file, &rendered)?;

        Ok(())
    }

    /// Map each Rust type name to the cfg gating it.
    ///
    /// A name appearing in several `#ifdef` branches is always present, so its
    /// cfg is cleared.
    fn cfg_by_name(&self, spec: &XdrSpec) -> HashMap<String, Option<String>> {
        let mut cfg_by_name: HashMap<String, Option<String>> = HashMap::new();
        for def in spec.all_definitions() {
            if matches!(def, Definition::Const(_)) {
                continue;
            }
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
        cfg_by_name
    }

    /// Generate module entries and grouped definitions for per-file output,
    /// for the owned types and for the `const` module alike.
    ///
    /// When the same type name appears in multiple `#ifdef`/`#else` branches,
    /// definitions are grouped into the same file.
    fn generate_modules(
        &self,
        spec: &XdrSpec,
    ) -> (
        Vec<ModuleEntry>,
        Vec<Vec<DefinitionOutput>>,
        Vec<ConstModuleOutput>,
    ) {
        let mut order: Vec<String> = Vec::new();
        let mut grouped: HashMap<String, Vec<DefinitionOutput>> = HashMap::new();
        let mut grouped_const: HashMap<String, Vec<ConstDefinitionOutput>> = HashMap::new();
        // The names each module defines, in order, with the cfg of each
        // definition of that name, to build the `const` module's re-exports.
        let mut names: HashMap<String, Vec<(ConstName, Option<String>)>> = HashMap::new();

        for def in spec.all_definitions() {
            let m = match def {
                Definition::Const(c) => mod_name(&c.name),
                _ => mod_name(def.name()),
            };

            let (output, const_output) = self.generate_definition(def);
            match grouped.entry(m.clone()) {
                Entry::Vacant(e) => {
                    order.push(m.clone());
                    e.insert(vec![output]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(output);
                }
            }
            // A definition with neither a type of its own nor an encoding
            // contributes nothing to its file.
            if const_output.type_def.is_some() || !const_output.const_to_xdr.is_empty() {
                grouped_const.entry(m.clone()).or_default().push(const_output);
            }
            names
                .entry(m)
                .or_default()
                .push((self.const_name_of(def), self.resolve_cfg(def)));
        }

        let mut modules = Vec::new();
        let mut definitions = Vec::new();
        let mut const_modules = Vec::new();
        for m in order {
            let defs = grouped.remove(&m).unwrap();
            let const_defs = grouped_const.remove(&m).unwrap_or_default();
            let items = Self::const_mod_items(&names.remove(&m).unwrap_or_default());
            modules.push(ModuleEntry {
                mod_name: m.clone(),
            });
            definitions.push(defs);
            const_modules.push(ConstModuleOutput {
                mod_name: m,
                definitions: const_defs,
                items,
            });
        }

        (modules, definitions, const_modules)
    }

    /// How a definition is named in the `const` module, and whether it needs a
    /// name of its own there.
    fn const_name_of(&self, def: &Definition) -> ConstName {
        match def {
            Definition::Const(c) => ConstName::Reexported(const_name(&c.name)),
            _ => {
                let name = type_name(def.name());
                if self.const_required.contains(&name) {
                    ConstName::Defined
                } else {
                    ConstName::Reexported(name)
                }
            }
        }
    }

    /// The names a module contributes to the `const` module directly: the
    /// types it defines that have no borrowing form there, and its XDR consts.
    ///
    /// A name defined in several `#ifdef` branches is present whatever the
    /// configuration, so its alias carries no cfg.
    fn const_mod_items(names: &[(ConstName, Option<String>)]) -> Vec<ConstModItem> {
        let mut items: Vec<ConstModItem> = Vec::new();
        for (name, cfg) in names {
            let name = match name {
                ConstName::Defined => continue,
                ConstName::Reexported(n) => n,
            };
            if let Some(existing) = items.iter_mut().find(|i| &i.name == name) {
                existing.cfg = None;
            } else {
                items.push(ConstModItem {
                    name: name.clone(),
                    cfg: cfg.clone(),
                });
            }
        }
        items
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

            let (output, _) = self.generate_definition(def);
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

    /// Render the `const_xdr_len`/`const_to_xdr` wrapper for a definition.
    ///
    /// The wrapper is emitted inside the `const` module and implemented on the
    /// type as that module names it, matching the receiver the type's
    /// `ConstWriter::write_type_*` method takes.
    fn const_to_xdr(&self, name: &str, cfg: Option<&str>) -> String {
        let template = ConstToXdrTemplate {
            recv: name.to_string(),
            cfg: cfg.map(ToString::to_string),
            write_fn: format!("write_type_{}", name.to_snake_case()),
        };
        template.render().unwrap_or_default()
    }

    /// Resolve the cfg expression for a definition, rendered as a string.
    ///
    /// This is where additional cfg conditions (e.g. file-based cfg derived
    /// from `def.file_index()`) should be combined with the `#ifdef`-derived
    /// cfg before rendering. Use `CfgExpr::and()` to combine them.
    fn resolve_cfg(&self, def: &Definition) -> Option<String> {
        def.cfg().map(|c| c.render())
    }

    /// Generate a definition's owned output and its `const` module output.
    fn generate_definition(&self, def: &Definition) -> (DefinitionOutput, ConstDefinitionOutput) {
        let cfg = self.resolve_cfg(def);
        match def {
            Definition::Struct(s) => {
                let (o, c) = self.generate_struct(s, cfg);
                (DefinitionOutput::Struct(o), c)
            }
            Definition::Enum(e) => {
                let (o, c) = self.generate_enum(e, cfg);
                (DefinitionOutput::Enum(o), c)
            }
            Definition::Union(u) => {
                let (o, c) = self.generate_union(u, cfg);
                (DefinitionOutput::Union(o), c)
            }
            Definition::Typedef(t) => self.generate_typedef(t, cfg),
            Definition::Const(c) => (
                DefinitionOutput::Const(self.generate_const(c, cfg)),
                ConstDefinitionOutput {
                    type_def: None,
                    const_to_xdr: String::new(),
                },
            ),
        }
    }

    fn generate_struct(
        &self,
        s: &Struct,
        cfg: Option<String>,
    ) -> (StructOutput, ConstDefinitionOutput) {
        let name = type_name(&s.name);
        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);

        let (members, const_members): (Vec<StructMemberOutput>, Vec<ConstStructMemberOutput>) = s
            .members
            .iter()
            .map(|m| self.generate_member(m, &name, custom_str))
            .unzip();

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
        let const_output = ConstDefinitionOutput {
            type_def: self.const_required.contains(&name).then(|| {
                ConstTypeOutput::Struct(ConstStructOutput {
                    name: name.clone(),
                    cfg: cfg.clone(),
                    members: const_members,
                })
            }),
            const_to_xdr: self.const_to_xdr(&name, cfg.as_deref()),
        };

        (
            StructOutput {
                name,
                source_comment: source_comment(&s.source, type_kind),
                has_default: !custom_default,
                is_custom_str: custom_str,
                members,
                member_names,
                cfg,
            },
            const_output,
        )
    }

    fn generate_enum(&self, e: &Enum, cfg: Option<String>) -> (EnumOutput, ConstDefinitionOutput) {
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

        // An enum owns no heap data, so the `const` module aliases it rather
        // than giving it a borrowing form.
        let const_output = ConstDefinitionOutput {
            type_def: None,
            const_to_xdr: self.const_to_xdr(&name, cfg.as_deref()),
        };

        (
            EnumOutput {
                name,
                source_comment: source_comment(&e.source, "Enum"),
                has_default: !custom_default,
                is_custom_str: custom_str,
                members,
                cfg,
            },
            const_output,
        )
    }

    fn generate_union(&self, u: &Union, cfg: Option<String>) -> (UnionOutput, ConstDefinitionOutput) {
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

        let (arms, const_arms): (Vec<UnionArmOutput>, Vec<ConstUnionArmOutput>) = u
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
            .unzip();

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };
        let default_arm_cfg = u
            .arms
            .first()
            .and_then(|a| a.cfg.as_ref().map(|c| c.render()));

        let const_output = ConstDefinitionOutput {
            type_def: self.const_required.contains(&name).then(|| {
                ConstTypeOutput::Union(ConstUnionOutput {
                    name: name.clone(),
                    cfg: cfg.clone(),
                    discriminant_type: discriminant_type.clone(),
                    arms: const_arms,
                })
            }),
            const_to_xdr: self.const_to_xdr(&name, cfg.as_deref()),
        };

        (
            UnionOutput {
                name,
                source_comment: source_comment(&u.source, type_kind),
                has_default: !custom_default,
                is_custom_str: custom_str,
                discriminant_type,
                arms,
                cfg,
                default_arm_cfg,
            },
            const_output,
        )
    }

    fn generate_typedef(
        &self,
        t: &Typedef,
        cfg: Option<String>,
    ) -> (DefinitionOutput, ConstDefinitionOutput) {
        let name = type_name(&t.name);

        if is_builtin_type(&t.type_) {
            // A transparent alias to a builtin is served by the builtin's own
            // encoding, so it has none of its own.
            return (
                DefinitionOutput::TypedefAlias(TypedefAliasOutput {
                    name,
                    source_comment: source_comment(&t.source, "Typedef"),
                    type_ref: base_type_ref(&t.type_, None),
                    cfg,
                }),
                ConstDefinitionOutput {
                    type_def: None,
                    const_to_xdr: String::new(),
                },
            );
        }

        let custom_default = self.options.custom_default_impl.contains(&name);
        let custom_str = self.options.custom_str_impl.contains(&name);
        let no_display_fromstr = self.options.no_display_fromstr.contains(&name);
        let is_fixed_opaque_type = is_fixed_opaque(&t.type_);
        let is_fixed_array_type = is_fixed_array(&t.type_);
        let is_var_array_type = is_var_array(&t.type_);

        let resolved = resolve_type(&t.type_, None, &self.type_info, custom_str);

        let size = match &t.type_ {
            xdr_parser::ast::Type::OpaqueFixed(s)
            | xdr_parser::ast::Type::Array { size: s, .. } => Some(size_to_u32_string(s)),
            _ => None,
        };

        let const_output = ConstDefinitionOutput {
            type_def: self.const_required.contains(&name).then(|| {
                ConstTypeOutput::Newtype(ConstNewtypeOutput {
                    name: name.clone(),
                    cfg: cfg.clone(),
                    type_ref: resolved.const_type,
                })
            }),
            const_to_xdr: self.const_to_xdr(&name, cfg.as_deref()),
        };

        (
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
            }),
            const_output,
        )
    }

    fn generate_const(&self, c: &Const, cfg: Option<String>) -> ConstOutput {
        let value_str = match c.base {
            IntBase::Hexadecimal => format!("0x{:X}", c.value),
            IntBase::Decimal => c.value.to_string(),
        };
        ConstOutput {
            name: const_name(&c.name),
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
    ) -> (StructMemberOutput, ConstStructMemberOutput) {
        let name = field_name(&m.name);
        let serde_rename = field_json_rename(&m.name);
        let resolved = resolve_type(&m.type_, Some(parent), &self.type_info, custom_str);

        (
            StructMemberOutput {
                name: name.clone(),
                type_ref: resolved.type_ref,
                turbofish_type: resolved.turbofish_type,
                serde_as_type: resolved.serde_as_type,
                serde_rename,
            },
            ConstStructMemberOutput {
                arbitrary_with: const_arbitrary_with(&resolved.const_type),
                name,
                type_ref: resolved.const_type,
            },
        )
    }

    fn generate_union_arm(
        &self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        discriminant_prefix: &str,
        custom_str: bool,
    ) -> Vec<(UnionArmOutput, ConstUnionArmOutput)> {
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
                let cfg = arm.cfg.as_ref().map(|c| c.render());

                (
                    UnionArmOutput {
                        case_name: case_name.clone(),
                        case_value: case_value_expr.clone(),
                        is_void: arm.type_.is_none(),
                        type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                        turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                        serde_as_type: resolved.as_ref().and_then(|r| r.serde_as_type.clone()),
                        cfg: cfg.clone(),
                    },
                    ConstUnionArmOutput {
                        case_name,
                        case_value: case_value_expr,
                        is_void: arm.type_.is_none(),
                        arbitrary_with: resolved
                            .as_ref()
                            .and_then(|r| const_arbitrary_with(&r.const_type)),
                        type_ref: resolved.map(|r| r.const_type),
                        cfg,
                    },
                )
            })
            .collect()
    }
}

/// The `#[arbitrary(with = ...)]` a const field needs, if any.
///
/// A const type stands in for an owned `Box` with a `&'static` reference, and
/// there is no `Arbitrary` impl for a reference to give the derive. The
/// helpers named here build one the way `Box` does, so the const type consumes
/// the same input bytes as the owned type. Every other const field type has an
/// `Arbitrary` impl the derive finds on its own.
fn const_arbitrary_with(const_type: &str) -> Option<String> {
    if let Some(t) = const_type.strip_prefix("&'static ") {
        Some(format!("arbitrary_ref::<{t}>"))
    } else if let Some(t) = const_type
        .strip_prefix("Option<&'static ")
        .and_then(|t| t.strip_suffix('>'))
    {
        Some(format!("arbitrary_option_ref::<{t}>"))
    } else {
        None
    }
}

/// One definition module's contribution to the `const` module.
struct ConstModuleOutput {
    mod_name: String,
    /// The definitions with const content of their own, which go in the
    /// module's file. Empty where the module contributes only names.
    definitions: Vec<ConstDefinitionOutput>,
    /// The names the module contributes to the `const` module directly.
    items: Vec<ConstModItem>,
}

/// How a definition is named in the `const` module.
enum ConstName {
    /// The `const` module defines a borrowing form under the name itself.
    Defined,
    /// The `const` module re-exports the parent's definition under this name.
    Reexported(String),
}

// =============================================================================
// Borrow analysis
// =============================================================================

/// Borrow analysis over a whole spec, resolving type references by name.
struct BorrowAnalysis<'a> {
    defs_by_name: HashMap<String, Vec<&'a Definition>>,
    /// Whether each name holds heap data under some cfg, keyed by Rust type
    /// name and memoized.
    by_name: HashMap<String, bool>,
    /// Names currently being resolved, for cycle detection.
    stack: HashSet<String>,
}

impl<'a> BorrowAnalysis<'a> {
    fn build(spec: &'a XdrSpec) -> Self {
        let mut defs_by_name: HashMap<String, Vec<&'a Definition>> = HashMap::new();
        for def in spec.all_definitions() {
            defs_by_name
                .entry(type_name(def.name()))
                .or_default()
                .push(def);
        }
        let mut analysis = Self {
            defs_by_name,
            by_name: HashMap::new(),
            stack: HashSet::new(),
        };
        let names: Vec<String> = analysis.defs_by_name.keys().cloned().collect();
        for name in names {
            analysis.of_name(&name);
        }
        analysis
    }

    /// The names that get a `Const` form, i.e. that hold heap data under some
    /// cfg.
    ///
    /// A name whose heap data sits only behind a cfg is included. It has to be:
    /// a container naming it does so unconditionally, and the owned type in
    /// that position cannot be built or serialized in a const context wherever
    /// the cfg turns its heap data on.
    fn const_required(&self) -> HashSet<String> {
        self.by_name
            .iter()
            .filter(|(_, borrows)| **borrows)
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// Whether a name holds heap data under any of its cfg branches.
    ///
    /// A name already on the stack indicates a reference cycle. Valid XDR
    /// breaks cycles with optional or variable-length types, both of which the
    /// generator maps to heap allocations (`Box` or `VecM`), so a type on a
    /// cycle always borrows.
    fn of_name(&mut self, name: &str) -> bool {
        if let Some(b) = self.by_name.get(name) {
            return *b;
        }
        if self.stack.contains(name) {
            return true;
        }
        let Some(defs) = self.defs_by_name.get(name).cloned() else {
            return false;
        };
        self.stack.insert(name.to_string());
        let borrows = defs
            .into_iter()
            .fold(false, |acc, def| acc | self.of_def(def));
        self.stack.remove(name);
        self.by_name.insert(name.to_string(), borrows);
        borrows
    }

    /// Whether a single definition holds heap data. A cfg-gated union arm
    /// counts: the `Const` form is emitted under every cfg, with the arm gated
    /// inside it.
    ///
    /// The folds below use `|` rather than `||` so the walk visits every
    /// member instead of stopping at the first that borrows, leaving what the
    /// analysis memoizes independent of member order.
    fn of_def(&mut self, def: &Definition) -> bool {
        match def {
            Definition::Struct(s) => s
                .members
                .iter()
                .fold(false, |acc, m| acc | self.of_type(&m.type_)),
            Definition::Union(u) => u
                .arms
                .iter()
                .filter_map(|arm| arm.type_.as_ref())
                .fold(false, |acc, t| acc | self.of_type(t)),
            Definition::Typedef(t) => self.of_type(&t.type_),
            Definition::Enum(_) | Definition::Const(_) => false,
        }
    }

    fn of_type(&mut self, type_: &Type) -> bool {
        match type_ {
            Type::OpaqueVar(_) | Type::String(_) | Type::VarArray { .. } => true,
            Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
            | Type::OpaqueFixed(_) => false,
            Type::Ident(name) => self.of_name(&type_name(name)),
            Type::Optional(inner) => self.of_type(inner),
            Type::Array { element_type, .. } => self.of_type(element_type),
        }
    }
}

