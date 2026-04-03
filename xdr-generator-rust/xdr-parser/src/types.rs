//! Language-agnostic type resolution and classification utilities.

use crate::ast::{Definition, Enum, Size, Type, XdrSpec};
use std::collections::{HashMap, HashSet};

/// Type information collected from the XDR spec.
pub struct TypeInfo {
    /// Map from type name (in target language form) to its definition
    pub definitions: HashMap<String, Definition>,
    /// Map from type name to the types it references in its fields
    pub type_field_types: HashMap<String, Vec<String>>,
    /// Map from const name to its value
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
            Size::Literal(n) => n.to_string(),
            Size::Named(name) => {
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
    ///
    /// `type_name_fn` is used to convert the ident name to the target language's
    /// type name for lookup in the definitions map.
    pub fn resolve_typedef_to_builtin_with<'a>(
        &'a self,
        type_: &Type,
        type_name_fn: &dyn Fn(&str) -> String,
    ) -> Option<&'a Type> {
        if let Type::Ident(name) = type_ {
            let target_name = type_name_fn(name);
            if let Some(Definition::Typedef(t)) = self.definitions.get(&target_name) {
                if is_builtin_type(&t.type_) {
                    return Some(&t.type_);
                }
            }
        }
        None
    }

    /// Convenience: resolve typedef to builtin using a provided naming function.
    /// This is the backward-compatible version that uses rust_type_name internally.
    pub fn resolve_typedef_to_builtin<'a>(&'a self, type_: &Type) -> Option<&'a Type> {
        // For backward compatibility, try lookup with the raw ident name first,
        // then fall back. The definitions map is keyed by the target-language name.
        if let Type::Ident(name) = type_ {
            // Try all definitions to find a match - the map is keyed by transformed name
            for (_, def) in &self.definitions {
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
        if let Type::Ident(name) = discriminant_type {
            // Search by original XDR name since the map is keyed by target-language name
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
    matches!(type_, Type::OpaqueFixed(_))
}

/// Check if a type is a fixed-length array (including fixed opaque).
pub fn is_fixed_array(type_: &Type) -> bool {
    matches!(type_, Type::OpaqueFixed(_) | Type::Array { .. })
}

/// Check if a type is a variable-length array.
pub fn is_var_array(type_: &Type) -> bool {
    matches!(
        type_,
        Type::OpaqueVar(_) | Type::String(_) | Type::VarArray { .. }
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
        Definition::Typedef(_) | Definition::Enum(_) | Definition::Const(_) => vec![],
    }
}

/// Get the base type name from a Type (for cyclic detection).
fn base_type_name(type_: &Type, type_name_fn: &dyn Fn(&str) -> String) -> Option<String> {
    match type_ {
        Type::Ident(name) => Some(type_name_fn(name)),
        Type::Optional(inner) => base_type_name(inner, type_name_fn),
        Type::Array { element_type, .. } => base_type_name(element_type, type_name_fn),
        Type::VarArray { element_type, .. } => base_type_name(element_type, type_name_fn),
        _ => None,
    }
}
