//! AST types for XDR definitions.

use crate::lexer::IntBase;

/// The root of a parsed XDR file or collection of files.
#[derive(Debug, Clone, Default)]
pub struct XdrSpec {
    pub namespaces: Vec<Namespace>,
    pub definitions: Vec<Definition>,
}

impl XdrSpec {
    /// Get all definitions, including those in namespaces.
    pub fn all_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions
            .iter()
            .chain(self.namespaces.iter().flat_map(|ns| &ns.definitions))
    }
}

/// A namespace containing definitions.
#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub definitions: Vec<Definition>,
}

/// A top-level definition.
#[derive(Debug, Clone)]
pub enum Definition {
    Struct(Struct),
    Enum(Enum),
    Union(Union),
    Typedef(Typedef),
    Const(Const),
}

impl Definition {
    /// Get the name of this definition.
    pub fn name(&self) -> &str {
        match self {
            Definition::Struct(s) => &s.name,
            Definition::Enum(e) => &e.name,
            Definition::Union(u) => &u.name,
            Definition::Typedef(t) => &t.name,
            Definition::Const(c) => &c.name,
        }
    }

    /// Check if this definition is nested (inline struct/union extracted from parent).
    pub fn is_nested(&self) -> bool {
        match self {
            Definition::Struct(s) => s.is_nested,
            Definition::Union(u) => u.is_nested,
            // Enums, typedefs, and consts are never nested
            Definition::Enum(_) | Definition::Typedef(_) | Definition::Const(_) => false,
        }
    }

    /// Get the parent type name if this is a nested definition.
    pub fn parent(&self) -> Option<&str> {
        match self {
            Definition::Struct(s) => s.parent.as_deref(),
            Definition::Union(u) => u.parent.as_deref(),
            // Enums, typedefs, and consts have no parent
            Definition::Enum(_) | Definition::Typedef(_) | Definition::Const(_) => None,
        }
    }
}

/// A struct definition.
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub members: Vec<Member>,
    /// Original XDR source text for documentation.
    pub source: String,
    /// True if this is a nested/inline struct extracted from a union arm.
    pub is_nested: bool,
    /// Name of the parent type if this is nested, for ordering purposes.
    pub parent: Option<String>,
}

/// An enum definition.
#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
    /// Original XDR source text for documentation.
    pub source: String,
}

/// A union definition.
#[derive(Debug, Clone)]
pub struct Union {
    pub name: String,
    pub discriminant: Discriminant,
    pub arms: Vec<UnionArm>,
    pub default_arm: Option<Box<UnionArm>>,
    /// Original XDR source text for documentation.
    pub source: String,
    /// True if this is a nested/inline union extracted from a struct field.
    pub is_nested: bool,
    /// Name of the parent type if this is nested, for ordering purposes.
    pub parent: Option<String>,
}

/// A typedef definition.
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
    /// Original XDR source text for documentation.
    pub source: String,
}

/// A const definition.
#[derive(Debug, Clone)]
pub struct Const {
    pub name: String,
    pub value: i64,
    /// The base (radix) of the literal in the source.
    pub base: IntBase,
    /// Original XDR source text for documentation.
    pub source: String,
}

/// XDR type specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// `int` - 32-bit signed integer
    Int,
    /// `unsigned int` - 32-bit unsigned integer
    UnsignedInt,
    /// `hyper` - 64-bit signed integer
    Hyper,
    /// `unsigned hyper` - 64-bit unsigned integer
    UnsignedHyper,
    /// `float` - 32-bit floating point
    Float,
    /// `double` - 64-bit floating point
    Double,
    /// `bool` - boolean
    Bool,
    /// `opaque[N]` - fixed-length opaque data
    OpaqueFixed(Size),
    /// `opaque<N>` or `opaque<>` - variable-length opaque data
    OpaqueVar(Option<Size>),
    /// `string<N>` or `string<>` - variable-length string
    String(Option<Size>),
    /// Reference to another type by name
    Ident(String),
    /// `T*` - optional type
    Optional(Box<Type>),
    /// `T[N]` - fixed-length array
    Array { element_type: Box<Type>, size: Size },
    /// `T<N>` or `T<>` - variable-length array
    VarArray {
        element_type: Box<Type>,
        max_size: Option<Size>,
    },
    /// Anonymous union inline in a struct - will be extracted during parsing
    /// Note: Uses Box to avoid recursive type issues
    AnonymousUnion {
        discriminant: Box<Discriminant>,
        arms: Vec<UnionArm>,
        default_arm: Option<Box<UnionArm>>,
    },
}

/// A member of a struct.
#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub type_: Type,
}

/// A member of an enum.
#[derive(Debug, Clone)]
pub struct EnumMember {
    pub name: String,
    pub value: i32,
}

/// The discriminant of a union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discriminant {
    pub name: String,
    pub type_: Type,
}

/// An arm of a union (one or more cases with the same type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionArm {
    pub cases: Vec<UnionCase>,
    /// The type for this arm. None means `void`.
    pub type_: Option<Type>,
}

/// A case in a union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionCase {
    /// The case value - either an identifier (enum variant) or a literal.
    pub value: CaseValue,
}

/// Value for a union case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseValue {
    /// Named identifier (typically an enum variant)
    Ident(String),
    /// Literal integer value
    Literal(i32),
}

/// A size specification, either a literal number or a named constant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Literal(u32),
    Named(String),
}
