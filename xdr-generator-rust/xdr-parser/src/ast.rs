//! AST types for XDR definitions.

use crate::lexer::IntBase;

/// Metadata about a parsed XDR source file.
#[derive(Debug, Clone, PartialEq)]
pub struct XdrFile {
    pub name: String,
    pub sha256: String,
}

/// The root of a parsed XDR file or collection of files.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XdrSpec {
    pub files: Vec<XdrFile>,
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

    /// Get type names (excluding consts) ordered so parents appear before children.
    ///
    /// The parser emits nested types before their parents (so the compiler sees
    /// them before they're referenced). This method provides the opposite order:
    /// parents first, then children — useful for type variant enums.
    pub fn type_names_parent_first(&self) -> Vec<&str> {
        use std::collections::{HashMap, HashSet};

        let mut children_of: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut all: Vec<(&str, Option<&str>)> = Vec::new();

        for def in self.all_definitions() {
            if matches!(def, Definition::Const(_)) {
                continue;
            }
            let name = def.name();
            let parent = def.parent();
            if let Some(p) = parent {
                children_of.entry(p).or_default().push(name);
            }
            all.push((name, parent));
        }

        let mut result: Vec<&str> = Vec::new();
        let mut added: HashSet<&str> = HashSet::new();

        fn add_with_children<'a>(
            name: &'a str,
            result: &mut Vec<&'a str>,
            added: &mut HashSet<&'a str>,
            children_of: &HashMap<&'a str, Vec<&'a str>>,
        ) {
            if added.contains(name) {
                return;
            }
            added.insert(name);
            result.push(name);
            if let Some(children) = children_of.get(name) {
                for child in children {
                    add_with_children(child, result, added, children_of);
                }
            }
        }

        for (name, parent) in &all {
            if parent.is_none() {
                add_with_children(name, &mut result, &mut added, &children_of);
            }
        }

        // Add any remaining types that weren't reached
        for (name, _) in &all {
            if !added.contains(name) {
                result.push(name);
            }
        }

        result
    }
}

/// A namespace containing definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Namespace {
    pub name: String,
    pub definitions: Vec<Definition>,
}

/// A top-level definition.
#[derive(Debug, Clone, PartialEq)]
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

    /// Get the file index this definition was parsed from.
    pub fn file_index(&self) -> usize {
        match self {
            Definition::Struct(s) => s.file_index,
            Definition::Enum(e) => e.file_index,
            Definition::Union(u) => u.file_index,
            Definition::Typedef(t) => t.file_index,
            Definition::Const(c) => c.file_index,
        }
    }
}

/// A struct definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub members: Vec<StructMember>,
    /// Original XDR source text for documentation.
    pub source: String,
    /// True if this is a nested/inline struct extracted from a union arm.
    pub is_nested: bool,
    /// Name of the parent type if this is nested, for ordering purposes.
    pub parent: Option<String>,
    /// Index into `XdrSpec::files` for the file this was parsed from.
    pub file_index: usize,
}

/// An enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
    /// The common prefix shared by all member names, up to the last underscore.
    pub member_prefix: String,
    /// Original XDR source text for documentation.
    pub source: String,
    /// Index into `XdrSpec::files` for the file this was parsed from.
    pub file_index: usize,
}

impl Enum {
    /// Create a new Enum, computing stripped member names from the common prefix.
    pub fn new(name: String, members: Vec<(String, i32)>, source: String) -> Self {
        let names: Vec<&str> = members.iter().map(|(n, _)| n.as_str()).collect();
        let member_prefix = find_common_prefix(&names).to_string();
        let members = members
            .into_iter()
            .map(|(name, value)| EnumMember {
                stripped_name: strip_prefix(&name, &member_prefix),
                name,
                value,
            })
            .collect();
        Self {
            name,
            members,
            member_prefix,
            source,
            file_index: 0,
        }
    }
}

/// A union definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub name: String,
    pub discriminant: UnionDiscriminant,
    pub arms: Vec<UnionArm>,
    /// Original XDR source text for documentation.
    pub source: String,
    /// True if this is a nested/inline union extracted from a struct field.
    pub is_nested: bool,
    /// Name of the parent type if this is nested, for ordering purposes.
    pub parent: Option<String>,
    /// Index into `XdrSpec::files` for the file this was parsed from.
    pub file_index: usize,
}

/// A typedef definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
    /// Original XDR source text for documentation.
    pub source: String,
    /// Index into `XdrSpec::files` for the file this was parsed from.
    pub file_index: usize,
}

/// A const definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub name: String,
    pub value: i64,
    /// The base (radix) of the literal in the source.
    pub base: IntBase,
    /// Original XDR source text for documentation.
    pub source: String,
    /// Index into `XdrSpec::files` for the file this was parsed from.
    pub file_index: usize,
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
}

/// A member of a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct StructMember {
    pub name: String,
    pub type_: Type,
}

/// A member of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub name: String,
    /// The member name with the common enum prefix stripped.
    pub stripped_name: String,
    pub value: i32,
}

/// The discriminant of a union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionDiscriminant {
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
    pub value: UnionCaseValue,
}

/// Value for a union case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionCaseValue {
    /// Named identifier (typically an enum variant)
    Ident(String),
    /// Literal integer value
    Literal(i32),
}

impl UnionCaseValue {
    /// Get the identifier name with the common prefix stripped.
    /// Returns `None` for literal values.
    pub fn stripped_ident(&self, prefix: &str) -> Option<String> {
        match self {
            UnionCaseValue::Ident(name) => Some(strip_prefix(name, prefix)),
            UnionCaseValue::Literal(_) => None,
        }
    }
}

/// A size specification, either a literal number or a named constant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Literal(u32),
    Named(String),
}

// =============================================================================
// Prefix stripping helpers
// =============================================================================

/// Find the common prefix shared by all names, up to and including the last
/// shared underscore. Returns an empty string if no common underscore-delimited
/// prefix exists.
fn find_common_prefix<'a>(names: &[&'a str]) -> &'a str {
    if names.len() <= 1 {
        return "";
    }

    let first = names[0];

    let common_len = names.iter().skip(1).fold(first.len(), |len, name| {
        first
            .bytes()
            .zip(name.bytes())
            .take(len)
            .take_while(|(a, b)| a == b)
            .count()
    });

    let common = &first[..common_len];

    if let Some(last_underscore) = common.rfind('_') {
        &first[..=last_underscore]
    } else {
        ""
    }
}

/// Strip a prefix from a name. If the result would start with a digit, keep the
/// first character of the prefix to avoid invalid identifiers.
fn strip_prefix(name: &str, prefix: &str) -> String {
    if !prefix.is_empty() && name.starts_with(prefix) {
        let stripped = &name[prefix.len()..];
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
