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
    /// True when the type contains heap-allocated data and a borrowing
    /// `{name}Ref<'a>` type is generated for it.
    pub requires_ref: bool,
    /// True when the Ref struct has no lifetime-using member (the type
    /// requires a Ref variant only because a same-named definition in another
    /// cfg branch contains heap data) and needs a phantom member to bind `'a`.
    pub ref_needs_phantom: bool,
    pub cfg: Option<String>,
}

pub struct StructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub turbofish_type: String,
    pub serde_as_type: Option<String>,
    /// The correct SEP-51 JSON key when the Rust field name was keyword-escaped
    /// (e.g. `type_` -> JSON `type`). `None` when the name was not escaped.
    pub serde_rename: Option<String>,
    /// The member's type in the borrowing `Ref` form of the parent type.
    pub ref_type_ref: String,
    /// Expression converting the member from `Ref` form to owned form.
    pub from_ref_expr: String,
    /// Const-encoding statements serializing this member via a `ConstWriter`.
    pub const_write: String,
}

pub struct EnumOutput {
    pub name: String,
    pub source_comment: String,
    pub has_default: bool,
    pub is_custom_str: bool,
    pub members: Vec<EnumStructMemberOutput>,
    pub cfg: Option<String>,
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
    /// Const-encoding statements serializing the discriminant (bound to `d`)
    /// via a `ConstWriter`.
    pub discriminant_const_write: String,
    /// True when the type contains heap-allocated data and a borrowing
    /// `{name}Ref<'a>` type is generated for it.
    pub requires_ref: bool,
    /// True when the `Ref` enum gets an uninhabited phantom variant to bind
    /// the `'a` lifetime when all lifetime-using arms are compiled out.
    pub ref_needs_phantom: bool,
    /// The cfg for the phantom variant (the negation of the lifetime-using
    /// arms' cfgs), or `None` for an unconditional phantom variant.
    pub ref_phantom_cfg: Option<String>,
    pub cfg: Option<String>,
    /// Cfg for the first arm, used to gate the Default impl when the
    /// default variant is behind a cfg.
    pub default_arm_cfg: Option<String>,
}

pub struct UnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub turbofish_type: Option<String>,
    pub serde_as_type: Option<String>,
    /// The arm's payload type in the borrowing `Ref` form of the parent type.
    pub ref_type_ref: Option<String>,
    /// Expression converting the payload from `Ref` form to owned form, with
    /// the payload bound by reference to `value`.
    pub from_ref_expr: Option<String>,
    /// Const-encoding statements serializing this arm's payload (bound by
    /// reference to `v`) via a `ConstWriter`; `None` for a void arm.
    pub const_write: Option<String>,
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
    /// True when the type contains heap-allocated data and a borrowing
    /// `{name}Ref<'a>` type is generated for it.
    pub requires_ref: bool,
    /// True when the Ref newtype's inner type does not use `'a` (the type
    /// requires a Ref variant only because a same-named definition in another
    /// cfg branch contains heap data) and needs a phantom member to bind it.
    pub ref_needs_phantom: bool,
    /// The inner type in the borrowing `Ref` form of the newtype.
    pub ref_type_ref: String,
    /// Expression converting the inner value from `Ref` form to owned form.
    pub from_ref_expr: String,
    /// Const-encoding statements serializing the inner value via a
    /// `ConstWriter`.
    pub const_write: String,
    pub cfg: Option<String>,
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
