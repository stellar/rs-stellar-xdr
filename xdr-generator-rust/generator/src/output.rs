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

/// The `const` module file: the header, and an entry per definition module.
#[derive(Template)]
#[template(path = "const_mod.rs.jinja", escape = "none")]
pub struct ConstModTemplate {
    pub header: String,
    pub entries: Vec<ConstModEntry>,
}

/// What one definition module contributes to the `const` module.
pub struct ConstModEntry {
    /// The module's file, when it has const content of its own.
    pub mod_name: Option<String>,
    /// The names the module contributes to the `const` module directly, rather
    /// than through its file.
    pub items: Vec<ConstModItem>,
}

/// A name the `const` module takes from its parent unchanged: a type with no
/// borrowing form, or an XDR const.
///
/// It is re-exported rather than aliased so that the name works in both
/// namespaces: a `pub type` alias cannot be used to construct a tuple struct
/// or name a unit variant.
pub struct ConstModItem {
    pub name: String,
    pub cfg: Option<String>,
}

/// One definition module's file inside the `const` module.
#[derive(Template)]
#[template(path = "const_definition.rs.jinja", escape = "none")]
pub struct ConstDefinitionTemplate {
    pub definitions: Vec<ConstDefinitionOutput>,
    /// The `ConstWriter` methods serializing the types defined in this file.
    pub const_writer: ConstWriterOutput,
}

/// The const-module form of one definition.
pub struct ConstDefinitionOutput {
    /// The borrowing type the definition gets in the `const` module, where it
    /// owns heap data. Where it does not, the `const` module aliases the owned
    /// type instead and this is `None`.
    pub type_def: Option<ConstTypeOutput>,
    /// The rendered `const_xdr_len`/`const_to_xdr` wrapper impl block, empty
    /// for a definition with no const encoding of its own.
    pub const_to_xdr: String,
}

/// A borrowing type definition in the `const` module.
pub enum ConstTypeOutput {
    Struct(ConstStructOutput),
    Union(ConstUnionOutput),
    Newtype(ConstNewtypeOutput),
}

pub struct ConstStructOutput {
    pub name: String,
    pub cfg: Option<String>,
    pub members: Vec<ConstStructMemberOutput>,
}

pub struct ConstStructMemberOutput {
    pub name: String,
    pub type_ref: String,
    pub arbitrary_with: Option<String>,
}

pub struct ConstUnionOutput {
    pub name: String,
    pub cfg: Option<String>,
    pub discriminant_type: String,
    pub arms: Vec<ConstUnionArmOutput>,
}

pub struct ConstUnionArmOutput {
    pub case_name: String,
    pub case_value: String,
    pub is_void: bool,
    pub type_ref: Option<String>,
    pub arbitrary_with: Option<String>,
    pub cfg: Option<String>,
}

pub struct ConstNewtypeOutput {
    pub name: String,
    pub cfg: Option<String>,
    pub type_ref: String,
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

/// The `const_xdr_len` / `const_to_xdr` wrapper emitted on a type, which sets
/// up a `ConstWriter` and calls that type's `write_type_*` method.
#[derive(Template)]
#[template(path = "const_to_xdr.rs.jinja", escape = "none")]
pub struct ConstToXdrTemplate {
    /// The receiver the wrapper is implemented on, e.g. `Memo`, naming the
    /// type as the `const` module names it.
    pub recv: String,
    pub cfg: Option<String>,
    /// The `ConstWriter` method the wrapper calls.
    pub write_fn: String,
}

/// The `ConstWriter` methods that serialize each generated type.
pub struct ConstWriterOutput {
    pub methods: Vec<ConstWriterMethodOutput>,
}

/// One `ConstWriter::write_*` method: what it serializes and how.
pub struct ConstWriterMethodOutput {
    pub name: String,
    /// Generic parameters, e.g. `<const MAX: u32>` for the `VecM` methods.
    pub generics: String,
    /// The type of the value parameter, e.g. `&Transaction`, naming the type
    /// as the `const` module names it.
    pub param_type: String,
    pub cfg: Option<String>,
    /// The module the method is emitted into: the one holding the type it
    /// serializes. Every method has one, because a wrapper over a builtin has
    /// no type file and so is hand-written on `ConstWriter` instead.
    pub module: String,
    /// What the method serializes, for its doc comment.
    pub subject: ConstSubject,
    pub body: ConstWriterBody,
}

/// What a `ConstWriter` method serializes.
pub enum ConstSubject {
    /// A type defined in the `.x` files, by Rust name.
    Type(String),
    /// An optional value; `owned` is the owned Rust type of the whole option.
    Option { inner: ConstDocName, owned: String },
    /// A variable-length array; `elem` is the owned Rust type of an element.
    Vec { inner: ConstDocName, elem: String },
}

/// How a type is named in a generated doc comment.
pub struct ConstDocName {
    pub name: String,
    /// Whether the name is a type from the `.x` files, and so an intra-doc
    /// link, rather than a builtin shown as plain code.
    pub link: bool,
}

/// The body of a `ConstWriter` method, by the shape of the type it serializes.
pub enum ConstWriterBody {
    /// A struct: each member in order. A typedef newtype is one of these with
    /// a single member, its inner value.
    Struct(Vec<ConstEncode>),
    /// An enum: its discriminant value as an XDR int.
    Enum,
    /// A union: its discriminant, then the payload of the selected arm.
    Union {
        /// The type matched on, named as the `const` module names it.
        scrutinee: String,
        discriminant: ConstEncode,
        arms: Vec<ConstUnionArm>,
    },
    /// An `Option`: a presence flag, then the value when present.
    Option(ConstEncode),
    /// A `VecM`: its length, then each element.
    Vec(ConstEncode),
}

pub struct ConstUnionArm {
    pub cfg: Option<String>,
    pub case_name: String,
    /// The encoding of the arm's payload, bound by reference to `value`; `None`
    /// for a void arm.
    pub payload: Option<ConstEncode>,
}

/// One call to a `ConstWriter` method, serializing one value.
pub struct ConstEncode {
    /// The fixed-array loops the call sits inside, outermost first.
    pub loops: Vec<ConstLoop>,
    /// The expression naming the value: a place such as `v.foo` or `s[i]`, or
    /// a `match` binding.
    pub acc: String,
    /// Whether `acc` is a binding that already holds a reference to the value.
    pub is_ref: bool,
    /// The `ConstWriter` method to call.
    pub method: String,
    /// How the method takes the value.
    pub pass: ConstPass,
}

/// A `while` loop over a fixed-size array.
pub struct ConstLoop {
    pub index: String,
    pub len: String,
}

/// How a `ConstWriter` method takes the value passed to it.
pub enum ConstPass {
    /// By value: the value is `Copy`.
    Value,
    /// By reference.
    Ref,
    /// As it is: the value is already a reference, the const form of a cyclic
    /// type.
    AsIs,
    /// As the byte slice the `const` module's `BytesM`/`StringM` exposes.
    Slice,
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
