use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use askama::Template;
use xdr_parser::ast::{
    CfgExpr, Const, Definition, Enum, Struct, StructMember, Type, Typedef, Union, UnionArm, XdrSpec,
};
use xdr_parser::lexer::IntBase;
use xdr_parser::types::{is_builtin_type, is_fixed_array, is_fixed_opaque, is_var_array, TypeInfo};

use crate::naming::{
    case_value, const_name, field_json_rename, field_name, mod_name, source_comment, type_name,
};
use crate::options::RustOptions;
use crate::output::{
    ConstOutput, DefinitionOutput, DefinitionTemplate, EnumOutput, EnumStructMemberOutput,
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
    /// cyclic references) under some cfg, and therefore have a borrowing `View`
    /// form generated for them.
    view_required: HashSet<String>,
    /// Per-definition borrow conditions, keyed by Rust type name and the
    /// definition's cfg, which distinguishes same-named `#ifdef`/`#else`
    /// branches from one another.
    def_borrow: HashMap<(String, Option<String>), BorrowCfg>,
}

impl RustGenerator {
    pub fn new(spec: &XdrSpec, options: RustOptions) -> Self {
        let type_info = TypeInfo::build(spec, &type_name);
        let mut analysis = BorrowAnalysis::build(spec);
        let view_required = analysis.view_required();
        let def_borrow = spec
            .all_definitions()
            .map(|def| {
                let key = (type_name(def.name()), def.cfg().map(CfgExpr::render));
                (key, analysis.of_def(def))
            })
            .collect();
        Self {
            options,
            type_info,
            view_required,
            def_borrow,
        }
    }

    /// How to emit the borrowing `View` form of a definition.
    fn view_emit_for(&self, name: &str, cfg: Option<&str>) -> ViewEmit {
        let borrow = self
            .def_borrow
            .get(&(name.to_string(), cfg.map(ToString::to_string)))
            .copied()
            .unwrap_or(BorrowCfg::Never);
        view_emit(self.view_required.contains(name), &borrow, cfg)
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
        let (mut modules, definitions) = self.generate_modules(spec);

        // Ensure the output directory exists.
        std::fs::create_dir_all(output_dir)?;

        // Write each definition (or group of definitions) to its own file.
        for (module, defs) in modules.iter().zip(definitions.into_iter()) {
            let template = DefinitionTemplate { definitions: defs };
            let rendered = template.render()?;
            let file_path = output_dir.join(format!("{}.rs", module.mod_name));
            std::fs::write(&file_path, &rendered)?;
        }

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
        let r = self.view_emit_for(&name, cfg.as_deref());
        StructOutput {
            name,
            source_comment: source_comment(&s.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            members,
            member_names,
            emit_view: r.emit_view,
            view_cfg: r.view_cfg,
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

        let type_kind = if u.is_nested { "NestedUnion" } else { "Union" };
        let default_arm_cfg = u
            .arms
            .first()
            .and_then(|a| a.cfg.as_ref().map(|c| c.render()));

        let r = self.view_emit_for(&name, cfg.as_deref());

        UnionOutput {
            name,
            source_comment: source_comment(&u.source, type_kind),
            has_default: !custom_default,
            is_custom_str: custom_str,
            discriminant_type,
            arms,
            emit_view: r.emit_view,
            view_cfg: r.view_cfg,
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

        let resolved = resolve_type(
            &t.type_,
            None,
            &self.type_info,
            custom_str,
            &self.view_required,
            "v.0",
            false,
        );

        let size = match &t.type_ {
            xdr_parser::ast::Type::OpaqueFixed(s)
            | xdr_parser::ast::Type::Array { size: s, .. } => Some(size_to_u32_string(s)),
            _ => None,
        };

        let r = self.view_emit_for(&name, cfg.as_deref());

        DefinitionOutput::TypedefNewtype(TypedefNewtypeOutput {
            name: name.clone(),
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
            emit_view: r.emit_view,
            view_cfg: r.view_cfg,
            view_type_ref: resolved.view_type_ref,
            from_view_expr: resolved.from_view_expr,
            cfg,
        })
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
    ) -> StructMemberOutput {
        let name = field_name(&m.name);
        let serde_rename = field_json_rename(&m.name);
        let resolved = resolve_type(
            &m.type_,
            Some(parent),
            &self.type_info,
            custom_str,
            &self.view_required,
            &format!("v.{name}"),
            false,
        );

        StructMemberOutput {
            name,
            type_ref: resolved.type_ref,
            turbofish_type: resolved.turbofish_type,
            serde_as_type: resolved.serde_as_type,
            serde_rename,
            view_type_ref: resolved.view_type_ref,
            from_view_expr: resolved.from_view_expr,
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

                let resolved = arm.type_.as_ref().map(|t| {
                    resolve_type(
                        t,
                        Some(parent),
                        &self.type_info,
                        custom_str,
                        &self.view_required,
                        "value",
                        true,
                    )
                });

                UnionArmOutput {
                    case_name,
                    case_value: case_value_expr,
                    is_void: arm.type_.is_none(),
                    type_ref: resolved.as_ref().map(|r| r.type_ref.clone()),
                    turbofish_type: resolved.as_ref().map(|r| r.turbofish_type.clone()),
                    view_type_ref: resolved.as_ref().map(|r| r.view_type_ref.clone()),
                    from_view_expr: resolved.as_ref().map(|r| r.from_view_expr.clone()),
                    serde_as_type: resolved.and_then(|r| r.serde_as_type),
                    cfg: arm.cfg.as_ref().map(|c| c.render()),
                }
            })
            .collect()
    }
}

// =============================================================================
// Borrow analysis
// =============================================================================

/// Whether a type holds heap-allocated data, and so whether its `View` form
/// would use its `'a` lifetime.
///
/// The distinction that matters is unconditional: only a type that borrows
/// under every cfg gets a `View` form. One that borrows under some cfgs would
/// need a cfg-gated `View`, which any unconditional container of it would name
/// unconditionally and so reference where it does not exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BorrowCfg {
    /// Holds no heap-allocated data under any cfg.
    Never,
    /// Holds heap-allocated data under every cfg.
    Always,
    /// Holds heap-allocated data only under some cfgs, because it sits behind a
    /// cfg-gated union arm or is reached through a type that does.
    Sometimes,
}

impl BorrowCfg {
    fn or(self, other: BorrowCfg) -> BorrowCfg {
        match (self, other) {
            (BorrowCfg::Always, _) | (_, BorrowCfg::Always) => BorrowCfg::Always,
            (BorrowCfg::Never, o) | (o, BorrowCfg::Never) => o,
            (BorrowCfg::Sometimes, BorrowCfg::Sometimes) => BorrowCfg::Sometimes,
        }
    }

    /// Restrict to also require `cfg`, as for a cfg-gated union arm.
    fn and_cfg(self, cfg: Option<&str>) -> BorrowCfg {
        if cfg.is_none() {
            return self;
        }
        match self {
            BorrowCfg::Never => BorrowCfg::Never,
            BorrowCfg::Always | BorrowCfg::Sometimes => BorrowCfg::Sometimes,
        }
    }
}

/// Borrow analysis over a whole spec, resolving type references by name.
struct BorrowAnalysis<'a> {
    defs_by_name: HashMap<String, Vec<&'a Definition>>,
    /// Memoized per-name results, keyed by Rust type name.
    by_name: HashMap<String, BorrowCfg>,
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

    /// The names that get a `View` form, i.e. that borrow under every cfg.
    ///
    /// A name that only borrows under some cfgs is excluded. Its `View` form
    /// would have to be cfg-gated, and an always-borrowing type containing it
    /// names that form unconditionally, so the reference would dangle wherever
    /// the cfg is off. Excluding it leaves containers holding the owned type in
    /// that position, which is correct under every cfg.
    fn view_required(&self) -> HashSet<String> {
        self.by_name
            .iter()
            .filter(|(_, b)| **b == BorrowCfg::Always)
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// The borrow condition for a name, across all of its cfg branches.
    ///
    /// A name already on the stack indicates a reference cycle. Valid XDR
    /// breaks cycles with optional or variable-length types, both of which the
    /// generator maps to heap allocations (`Box` or `VecM`), so a type on a
    /// cycle always borrows.
    fn of_name(&mut self, name: &str) -> BorrowCfg {
        if let Some(b) = self.by_name.get(name) {
            return *b;
        }
        if self.stack.contains(name) {
            return BorrowCfg::Always;
        }
        let Some(defs) = self.defs_by_name.get(name).cloned() else {
            return BorrowCfg::Never;
        };
        self.stack.insert(name.to_string());
        let mut borrow = BorrowCfg::Never;
        for def in defs {
            let def_cfg = def.cfg().map(CfgExpr::render);
            borrow = borrow.or(self.of_def(def).and_cfg(def_cfg.as_deref()));
        }
        self.stack.remove(name);
        self.by_name.insert(name.to_string(), borrow);
        borrow
    }

    /// The borrow condition contributed by a single definition, excluding the
    /// definition's own cfg, which callers apply where the type is emitted.
    fn of_def(&mut self, def: &Definition) -> BorrowCfg {
        let def_cfg = def.cfg().map(CfgExpr::render);
        match def {
            Definition::Struct(s) => s
                .members
                .iter()
                .fold(BorrowCfg::Never, |acc, m| acc.or(self.of_type(&m.type_))),
            Definition::Union(u) => u.arms.iter().fold(BorrowCfg::Never, |acc, arm| {
                let Some(t) = arm.type_.as_ref() else {
                    return acc;
                };
                // An arm cfg equal to the definition's own cfg adds no further
                // condition, since the definition is already gated on it.
                let arm_cfg = arm
                    .cfg
                    .as_ref()
                    .map(CfgExpr::render)
                    .filter(|c| Some(c) != def_cfg.as_ref());
                acc.or(self.of_type(t).and_cfg(arm_cfg.as_deref()))
            }),
            Definition::Typedef(t) => self.of_type(&t.type_),
            Definition::Enum(_) | Definition::Const(_) => BorrowCfg::Never,
        }
    }

    fn of_type(&mut self, type_: &Type) -> BorrowCfg {
        match type_ {
            Type::OpaqueVar(_) | Type::String(_) | Type::VarArray { .. } => BorrowCfg::Always,
            Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
            | Type::OpaqueFixed(_) => BorrowCfg::Never,
            Type::Ident(name) => self.of_name(&type_name(name)),
            Type::Optional(inner) => self.of_type(inner),
            Type::Array { element_type, .. } => self.of_type(element_type),
        }
    }
}

/// How a definition's borrowing `View` form is emitted.
///
/// A `{name}View<'a>` is emitted only where the definition borrows, so its `'a`
/// is always used. Where it does not borrow, nothing is emitted: the owned type
/// is already the whole value, and a heap-free type has nothing to borrow.
struct ViewEmit {
    emit_view: bool,
    view_cfg: Option<String>,
}

/// Decide how to emit the `View` form of one definition.
///
/// `has_view` is whether the type's name has a `View` form at all, and `borrow`
/// is this definition's borrow condition excluding its own `def_cfg`.
fn view_emit(has_view: bool, borrow: &BorrowCfg, def_cfg: Option<&str>) -> ViewEmit {
    let mut emit = ViewEmit {
        emit_view: false,
        view_cfg: None,
    };
    if !has_view {
        return emit;
    }
    match borrow {
        // Nothing to borrow in this branch, or nothing to borrow under some
        // cfg. Either way this definition emits no View form.
        BorrowCfg::Never | BorrowCfg::Sometimes => {}
        BorrowCfg::Always => {
            emit.emit_view = true;
            emit.view_cfg = def_cfg.map(ToString::to_string);
        }
    }
    emit
}
