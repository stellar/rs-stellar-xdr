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
    /// True when this definition borrows and gets a real `{name}Ref<'a>`.
    pub emit_ref: bool,
    /// The full cfg for the real `Ref` struct, gating it to where it borrows.
    pub ref_cfg: Option<String>,
    pub cfg: Option<String>,
}

/// How a `Ref` field borrows to break a type cycle, and so how its conversion
/// restores the `Box` the owned field holds.
///
/// A cycle has to be broken by indirection somewhere: the owned type uses a
/// `Box` and the `Ref` a reference into data the caller already holds. Only
/// these fields are boxed, so only their conversions add a `Box`.
#[derive(Clone, Copy)]
pub enum CyclicBorrow {
    /// Not cyclic. The owned field holds the value itself.
    NotCyclic,
    /// The `Ref` holds `&'a T` where the owned field holds `Box<T>`.
    Reference,
    /// The `Ref` holds `Option<&'a T>` where the owned field holds
    /// `Option<Box<T>>`.
    OptionalReference,
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
    pub ref_type: String,
    /// Whether the member borrows to break a cycle, and so needs boxing.
    pub cyclic: CyclicBorrow,
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
    /// True when a real `{name}Ref<'a>` enum is emitted, i.e. some arm borrows.
    pub emit_ref: bool,
    /// The full cfg for the real `Ref` enum. When every borrowing arm is behind
    /// a cfg, this is the union's cfg combined with the disjunction of those
    /// arm cfgs, so the enum only exists where its lifetime is actually used.
    pub ref_cfg: Option<String>,
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
    pub ref_type: Option<String>,
    /// Whether the payload borrows to break a cycle, and so needs boxing.
    pub cyclic: CyclicBorrow,
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
    /// True when this definition borrows and gets a real `{name}Ref<'a>`.
    pub emit_ref: bool,
    /// The full cfg for the real `Ref` newtype, gating it to where it borrows.
    pub ref_cfg: Option<String>,
    /// The inner type in the borrowing `Ref` form of the newtype.
    pub ref_type: String,
    /// Whether the inner value borrows to break a cycle, and so needs boxing.
    pub cyclic: CyclicBorrow,
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
