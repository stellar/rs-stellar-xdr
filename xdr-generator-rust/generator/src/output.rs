use askama::Template;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "generated.rs.jinja", escape = "none")]
pub struct GeneratedTemplate {
    pub xdr_files_sha256: Vec<(String, String)>,
    pub header: String,
    pub definitions: Vec<DefinitionOutput>,
    pub type_variant_enum: TypeEnumOutput,
}

#[derive(Template)]
#[template(path = "mod.rs.jinja", escape = "none")]
pub struct ModTemplate {
    pub xdr_files_sha256: Vec<(String, String)>,
    pub header: String,
    pub modules: Vec<ModuleEntry>,
}

#[derive(Template)]
#[template(path = "type_enum_definition.rs.jinja", escape = "none")]
pub struct TypeEnumDefinitionTemplate {
    pub type_variant_enum: TypeEnumOutput,
}

pub struct ModuleEntry {
    pub mod_name: String,
}

#[derive(Template)]
#[template(path = "definition.rs.jinja", escape = "none")]
pub struct DefinitionTemplate {
    pub definitions: Vec<DefinitionOutput>,
    /// The `ConstWriter` methods serializing the types defined in this file.
    pub const_writer: ConstWriterOutput,
}

/// The `const_xdr_len` / `const_to_xdr` wrapper emitted on a type, which sets
/// up a `ConstWriter` and calls that type's `write_type_*` method.
#[derive(Template)]
#[template(path = "const_to_xdr.rs.jinja", escape = "none")]
pub struct ConstToXdrTemplate {
    /// The receiver the wrapper is implemented on, e.g. `MemoView<'_>`.
    pub recv: String,
    pub cfg: Option<String>,
    /// The `ConstWriter` method the wrapper calls.
    pub write_fn: String,
}

#[derive(Template)]
#[template(path = "const_writer.rs.jinja", escape = "none")]
pub struct ConstWriterTemplate {
    pub const_writer: ConstWriterOutput,
}

/// The `ConstWriter` methods that serialize each generated type.
pub struct ConstWriterOutput {
    pub methods: Vec<ConstWriterMethodOutput>,
}

/// One `ConstWriter::write_xdr_*` method.
pub struct ConstWriterMethodOutput {
    pub name: String,
    /// Generic parameters, e.g. `<const MAX: u32>` for the `VecM` methods.
    pub generics: String,
    /// The type of the value parameter, e.g. `&TransactionView<'_>`.
    pub param_type: String,
    pub body: String,
    pub cfg: Option<String>,
    pub doc: String,
    /// The module the method is emitted into: the one holding the type it
    /// serializes. `None` for a wrapper over a builtin, which has no type file.
    pub module: Option<String>,
}

pub enum DefinitionOutput {
    Struct(StructOutput),
    Enum(EnumOutput),
    Union(UnionOutput),
    TypedefAlias(TypedefAliasOutput),
    TypedefNewtype(TypedefNewtypeOutput),
    Const(ConstOutput),
}

pub struct StructOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<StructMemberOutput>,
    pub member_names: String,
    /// True when this definition borrows and gets a real `{name}View<'a>`.
    pub emit_view: bool,
    /// The full cfg for the real `View` struct, gating it to where it borrows.
    pub view_cfg: Option<String>,
    pub cfg: Option<String>,
    /// The rendered `const_xdr_len`/`const_to_xdr` wrapper impl block.
    pub const_to_xdr: String,
}

pub struct StructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    /// The correct SEP-51 JSON key when the Rust field name was keyword-escaped
    /// (e.g. `type_` -> JSON `type`). `None` when the name was not escaped.
    pub serde_rename: Option<String>,
    /// The member's type in the borrowing `View` form of the parent type.
    pub view_type_ref: String,
    /// Expression converting the member from `View` form to owned form.
    pub from_view_expr: String,
}

pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<EnumStructMemberOutput>,
    pub cfg: Option<String>,
    /// The rendered `const_xdr_len`/`const_to_xdr` wrapper impl block.
    pub const_to_xdr: String,
}

pub struct EnumStructMemberOutput {
    pub name: String,
    pub value: i32,
    pub is_default: bool,
    pub cfg: Option<String>,
}

pub struct UnionOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub discriminant_type: String,
    pub arms: Vec<UnionArmOutput>,
    /// True when a real `{name}View<'a>` enum is emitted, i.e. some arm borrows.
    pub emit_view: bool,
    /// The full cfg for the real `View` enum. When every borrowing arm is behind
    /// a cfg, this is the union's cfg combined with the disjunction of those
    /// arm cfgs, so the enum only exists where its lifetime is actually used.
    pub view_cfg: Option<String>,
    pub cfg: Option<String>,
    /// Cfg for the first arm, used to gate the Default impl when the
    /// default variant is behind a cfg.
    pub default_arm_cfg: Option<String>,
    /// The rendered `const_xdr_len`/`const_to_xdr` wrapper impl block.
    pub const_to_xdr: String,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub turbofish_type: Option<String>,
    pub serde_as_type: Option<String>,
    /// The arm's payload type in the borrowing `View` form of the parent type.
    pub view_type_ref: Option<String>,
    /// Expression converting the payload from `View` form to owned form, with
    /// the payload bound by reference to `value`.
    pub from_view_expr: Option<String>,
    pub cfg: Option<String>,
}

pub struct TypedefAliasOutput {
    pub name: String,
    pub source_comment: String,
    pub type_ref: String,
    pub cfg: Option<String>,
}

pub struct TypedefNewtypeOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_var_array: bool,
    pub is_fixed_opaque: bool,
    pub is_fixed_array: bool,
    pub is_custom_str: bool,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    pub element_type: String,
    pub size: Option<String>,
    pub custom_debug: bool,
    pub custom_display_fromstr: bool,
    pub custom_schemars: bool,
    /// True when this definition borrows and gets a real `{name}View<'a>`.
    pub emit_view: bool,
    /// The full cfg for the real `View` newtype, gating it to where it borrows.
    pub view_cfg: Option<String>,
    /// The inner type in the borrowing `View` form of the newtype.
    pub view_type_ref: String,
    /// Expression converting the inner value from `View` form to owned form.
    pub from_view_expr: String,
    pub cfg: Option<String>,
    /// The rendered `const_xdr_len`/`const_to_xdr` wrapper impl block.
    pub const_to_xdr: String,
}

pub struct ConstOutput {
    pub name: String,
    pub doc_name: String,
    pub source_comment: String,
    pub value_str: String,
    pub cfg: Option<String>,
}

pub struct TypeEnumOutput {
    pub types: Vec<TypeEnumEntry>,
}

pub struct TypeEnumEntry {
    pub name: String,
    pub cfg: Option<String>,
}
