//! Language-agnostic type resolution and classification utilities.

use crate::ast::{Definition, Enum, Size, Type, XdrSpec};
use std::collections::{HashMap, HashSet};

/// Type information collected from the XDR spec.
pub struct TypeInfo {
    /// Map from type name (in target language form) to its definition.
    pub definitions: HashMap<String, Definition>,
    /// Map from type name to the types it references in its fields.
    pub type_field_types: HashMap<String, Vec<String>>,
    /// Map from const name to its value.
    pub const_values: HashMap<String, i64>,
}

impl TypeInfo {
    /// Build type information from an XDR spec.
    ///
    /// `type_name_fn` converts XDR names to the target language's type names.
    pub fn build(spec: &XdrSpec, type_name_fn: &dyn Fn(&str) -> String) -> Self {
        let mut definitions = HashMap::new();
        let mut type_field_types: HashMap<String, Vec<String>> = HashMap::new();
        let mut const_values = HashMap::new();

        for def in spec.all_definitions() {
            let name = type_name_fn(def.name());
            definitions.insert(name.clone(), def.clone());

            let field_types = collect_field_types(def, type_name_fn);
            type_field_types.insert(name, field_types);

            if let Definition::Const(c) = def {
                const_values.insert(c.name.clone(), c.value);
            }
        }

        Self {
            definitions,
            type_field_types,
            const_values,
        }
    }

    /// Resolve a size to a literal string, using const values for named sizes.
    pub fn size_to_literal(&self, size: &Size) -> String {
        match size {
            Size::Literal { literal: n } => n.to_string(),
            Size::Named { named: name } => {
                if let Some(&value) = self.const_values.get(name) {
                    value.to_string()
                } else {
                    name.clone()
                }
            }
        }
    }

    /// If `type_` is a `Type::Ident` that refers to a typedef whose underlying
    /// type is a builtin, return that builtin type.
    pub fn resolve_typedef_to_builtin<'a>(&'a self, type_: &Type) -> Option<&'a Type> {
        if let Type::Ident { ident: name } = type_ {
            for def in self.definitions.values() {
                if let Definition::Typedef(t) = def {
                    if t.name == *name && is_builtin_type(&t.type_) {
                        return Some(&t.type_);
                    }
                }
            }
        }
        None
    }

    /// Look up the enum definition for a discriminant type.
    ///
    /// If the discriminant is an `Ident` referring to an enum, returns that enum.
    /// Useful for getting the member prefix to strip from union case names.
    pub fn discriminant_enum(&self, discriminant_type: &Type) -> Option<&Enum> {
        if let Type::Ident { ident: name } = discriminant_type {
            for def in self.definitions.values() {
                if let Definition::Enum(e) = def {
                    if e.name == *name {
                        return Some(e);
                    }
                }
            }
        }
        None
    }

    /// Check if `type_with_fields` has a cyclic reference to `target_type`.
    pub fn is_cyclic(&self, type_with_fields: &str, target_type: &str) -> bool {
        self.is_cyclic_inner(type_with_fields, target_type, &mut HashSet::new())
    }

    fn is_cyclic_inner(
        &self,
        type_with_fields: &str,
        target_type: &str,
        seen: &mut HashSet<String>,
    ) -> bool {
        if seen.contains(type_with_fields) {
            return false;
        }
        seen.insert(type_with_fields.to_string());

        if let Some(field_types) = self.type_field_types.get(type_with_fields) {
            for ft in field_types {
                if ft == target_type {
                    return true;
                }
                if self.is_cyclic_inner(ft, target_type, seen) {
                    return true;
                }
            }
        }
        false
    }
}

// =============================================================================
// Wire size computation (XDR RFC 4506)
// =============================================================================

/// The kind of a top-level XDR definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefKind {
    Struct,
    Union,
    Enum,
    Typedef,
    Const,
}

/// Computed properties for each type definition, suitable for serialization into an IR.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TypeProperties {
    pub kind: DefKind,
    /// Total XDR wire size in bytes if the type is fixed-size, `None` if variable.
    /// When `None`, the consumer must read wire data (length prefixes, discriminants)
    /// to determine the size at runtime.
    pub fixed_wire_size: Option<u32>,
}

/// Pre-computed properties for the entire XDR spec.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComputedProperties {
    /// Per-type properties (fixed wire size).
    pub types: HashMap<String, TypeProperties>,
    /// Resolved constant values (const name → integer value).
    pub const_values: HashMap<String, i64>,
}

impl TypeInfo {
    /// Compute all properties for the spec.
    pub fn compute_properties(&self) -> ComputedProperties {
        let mut wire_sizes: HashMap<String, Option<u32>> = HashMap::new();
        let mut in_progress: HashSet<String> = HashSet::new();

        // Compute fixed_wire_size for all definitions
        for name in self.definitions.keys() {
            self.compute_wire_size(name, &mut wire_sizes, &mut in_progress);
        }

        // Compute per-type properties
        let mut types = HashMap::new();
        for (name, def) in &self.definitions {
            let kind = match def {
                Definition::Struct(_) => DefKind::Struct,
                Definition::Union(_) => DefKind::Union,
                Definition::Enum(_) => DefKind::Enum,
                Definition::Typedef(_) => DefKind::Typedef,
                Definition::Const(_) => DefKind::Const,
            };
            types.insert(
                name.clone(),
                TypeProperties {
                    kind,
                    fixed_wire_size: wire_sizes.get(name).copied().flatten(),
                },
            );
        }

        ComputedProperties {
            types,
            const_values: self.const_values.clone(),
        }
    }

    fn compute_wire_size(
        &self,
        name: &str,
        cache: &mut HashMap<String, Option<u32>>,
        in_progress: &mut HashSet<String>,
    ) -> Option<u32> {
        if let Some(&cached) = cache.get(name) {
            return cached;
        }
        if in_progress.contains(name) {
            return None; // cycle — variable size
        }
        in_progress.insert(name.to_string());

        let result = match self.definitions.get(name) {
            Some(Definition::Struct(s)) => {
                let mut total: u32 = 0;
                for member in &s.members {
                    match self.type_wire_size(&member.type_, cache, in_progress) {
                        Some(sz) => total += sz,
                        None => {
                            in_progress.remove(name);
                            cache.insert(name.to_string(), None);
                            return None;
                        }
                    }
                }
                Some(total)
            }
            Some(Definition::Union(u)) => {
                let disc_size: u32 = 4;
                let mut arm_sizes: Vec<Option<u32>> = Vec::new();
                for arm in &u.arms {
                    match &arm.type_ {
                        None => arm_sizes.push(Some(0)), // void
                        Some(t) => arm_sizes.push(self.type_wire_size(t, cache, in_progress)),
                    }
                }
                if arm_sizes.is_empty()
                    || arm_sizes.iter().any(|s| s.is_none())
                    || arm_sizes.iter().filter_map(|s| *s).collect::<HashSet<_>>().len() != 1
                {
                    None
                } else {
                    Some(disc_size + arm_sizes[0].unwrap())
                }
            }
            Some(Definition::Enum(_)) => Some(4),
            Some(Definition::Typedef(t)) => self.type_wire_size(&t.type_, cache, in_progress),
            Some(Definition::Const(_)) | None => None,
        };

        in_progress.remove(name);
        cache.insert(name.to_string(), result);
        result
    }

    fn type_wire_size(
        &self,
        type_: &Type,
        cache: &mut HashMap<String, Option<u32>>,
        in_progress: &mut HashSet<String>,
    ) -> Option<u32> {
        match type_ {
            Type::Int | Type::UnsignedInt | Type::Bool | Type::Float => Some(4),
            Type::Hyper | Type::UnsignedHyper | Type::Double => Some(8),
            Type::OpaqueFixed { size } => {
                let sz = self.resolve_size(size);
                Some((sz + 3) & !3) // pad to 4-byte boundary
            }
            Type::Array { element_type, size } => {
                let elem_sz = self.type_wire_size(element_type, cache, in_progress)?;
                let count = self.resolve_size(size);
                Some(elem_sz * count)
            }
            Type::Ident { ident: name } => self.compute_wire_size(name, cache, in_progress),
            // Variable-size types
            Type::OpaqueVar { max_size: _ } | Type::String { max_size: _ } | Type::VarArray { .. } | Type::Optional { element_type: _ } => {
                None
            }
        }
    }

    fn resolve_size(&self, size: &Size) -> u32 {
        match size {
            Size::Literal { literal: n } => *n,
            Size::Named { named: name } => self
                .const_values
                .get(name)
                .map(|&v| v as u32)
                .unwrap_or(0),
        }
    }
}

// =============================================================================
// Type classification utilities (language-agnostic)
// =============================================================================

/// Check if a type is a builtin (maps directly to a primitive in any language).
pub fn is_builtin_type(type_: &Type) -> bool {
    matches!(
        type_,
        Type::Int
            | Type::UnsignedInt
            | Type::Hyper
            | Type::UnsignedHyper
            | Type::Float
            | Type::Double
            | Type::Bool
    )
}

/// Check if a type is a fixed-length opaque array.
pub fn is_fixed_opaque(type_: &Type) -> bool {
    matches!(type_, Type::OpaqueFixed { size: _ })
}

/// Check if a type is a fixed-length array (including fixed opaque).
pub fn is_fixed_array(type_: &Type) -> bool {
    matches!(type_, Type::OpaqueFixed { size: _ } | Type::Array { .. })
}

/// Check if a type is a variable-length array.
pub fn is_var_array(type_: &Type) -> bool {
    matches!(
        type_,
        Type::OpaqueVar { max_size: _ } | Type::String { max_size: _ } | Type::VarArray { .. }
    )
}

// =============================================================================
// Internal utilities
// =============================================================================

/// Collect the base type names referenced in a definition's fields.
fn collect_field_types(def: &Definition, type_name_fn: &dyn Fn(&str) -> String) -> Vec<String> {
    match def {
        Definition::Struct(s) => s
            .members
            .iter()
            .filter_map(|m| base_type_name(&m.type_, type_name_fn))
            .collect(),
        Definition::Union(u) => u
            .arms
            .iter()
            .filter_map(|arm| {
                arm.type_
                    .as_ref()
                    .and_then(|t| base_type_name(t, type_name_fn))
            })
            .collect(),
        Definition::Typedef(t) => {
            base_type_name(&t.type_, type_name_fn)
                .into_iter()
                .collect()
        }
        Definition::Enum(_) | Definition::Const(_) => vec![],
    }
}

/// Get the base type name from a Type (for cyclic detection).
fn base_type_name(type_: &Type, type_name_fn: &dyn Fn(&str) -> String) -> Option<String> {
    match type_ {
        Type::Ident { ident: name } => Some(type_name_fn(name)),
        Type::Optional { element_type: inner } => base_type_name(inner, type_name_fn),
        Type::Array { element_type, .. } => base_type_name(element_type, type_name_fn),
        Type::VarArray { element_type, .. } => base_type_name(element_type, type_name_fn),
        _ => None,
    }
}
