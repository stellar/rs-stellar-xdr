//! Emission of const XDR serialization as inherent methods on `ConstWriter`.
//!
//! Every type defined in the `.x` files gets one
//! `ConstWriter::write_type_{type}` method that serializes a value of that
//! type, instead of each type gaining a `const_write_xdr` of its own. A type
//! that appears inside an `Option` or a `VecM` gets a
//! `write_type_option_{type}` or `write_type_vec_{type}` method alongside,
//! since each wrapped form is a distinct Rust type and const evaluation offers
//! no generic dispatch to share one encoder between them.
//!
//! The `type_` marks a type from the `.x` files, distinguishing these from the
//! primitive writers (`write_u32`, `write_var_opaque`, and so on) that
//! `ConstWriter` defines by hand. A wrapper over a primitive is likewise not a
//! defined type, so it is named `write_option_u32` rather than
//! `write_type_option_u32`.
//!
//! Keeping the encoders on the writer leaves each type with only the thin
//! `const_xdr_len`/`const_to_xdr` pair that wraps its writer method. Each
//! method is still emitted into the file of the type it serializes, so a type
//! and its encoder stay together; the only exception is a wrapper over a
//! builtin scalar, which has no type file and so goes into the `const_writer`
//! module.
//!
//! The emitted code produces the same bytes as the trait-based
//! `WriteXdr::write_xdr` implementations of the owned types, expressed with
//! only const-compatible operations: `while` loops instead of `for`, and direct
//! calls to each concrete type's writer method instead of trait dispatch.

use std::collections::{BTreeMap, HashMap, HashSet};

use heck::ToSnakeCase;
use xdr_parser::ast::{Definition, Type, Union, UnionArm, XdrSpec};
use xdr_parser::types::{is_builtin_type, TypeInfo};

use crate::naming::{case_value, field_name, mod_name, type_name};
use crate::output::{ConstWriterMethodOutput, ConstWriterOutput};
use crate::types::{const_view_base_type, const_view_type, type_ref};

/// Build the `ConstWriter` methods for an entire spec.
///
/// `cfg_by_name` maps a Rust type name to the cfg it is gated by, cleared to
/// `None` where the same name appears in several `#ifdef` branches and so is
/// always present.
pub(crate) fn build(
    spec: &XdrSpec,
    type_info: &TypeInfo,
    view_required: &HashSet<String>,
    cfg_by_name: &HashMap<String, Option<String>>,
) -> ConstWriterOutput {
    let mut c = Collector {
        type_info,
        view_required,
        cfg_by_name,
        wrappers: BTreeMap::new(),
        loop_depth: 0,
    };

    // One method per definition rather than per name: where the same type name
    // appears in several `#ifdef` branches each branch gets its own method,
    // carrying that branch's cfg, so only one is ever compiled.
    let mut methods: Vec<ConstWriterMethodOutput> = Vec::new();
    for def in spec.all_definitions() {
        if let Some(m) = c.definition_method(def) {
            methods.push(m);
        }
    }

    // Wrapper methods are collected while walking the definitions above, so
    // they are appended after. Sorted by name for a stable output.
    methods.extend(c.wrappers.into_values());

    ConstWriterOutput { methods }
}

/// Placeholder standing in for the union's Rust type in generated match arms,
/// substituted once the owned-versus-`View` choice is known.
const SCRUTINEE: &str = "__SCRUTINEE__";

struct Collector<'a> {
    type_info: &'a TypeInfo,
    view_required: &'a HashSet<String>,
    cfg_by_name: &'a HashMap<String, Option<String>>,
    /// Wrapper (`Option`/`VecM`) methods needed, keyed by method name.
    wrappers: BTreeMap<String, ConstWriterMethodOutput>,
    /// How many loops enclose the code being emitted. Each generated method is
    /// its own scope, so this resets per method and is only non-zero where a
    /// fixed array nests inside another loop.
    loop_depth: u32,
}

impl Collector<'_> {
    /// The loop index name for the current depth. Unsuffixed at the outermost
    /// loop, which is all this spec needs; a nested loop is suffixed so its
    /// index cannot shadow the one it sits inside.
    fn index_name(&self) -> String {
        if self.loop_depth == 0 {
            "i".to_string()
        } else {
            format!("i{}", self.loop_depth)
        }
    }

    /// Emit `f` with the loop depth increased, for code inside a loop body.
    fn in_loop<F: FnOnce(&mut Self) -> String>(&mut self, f: F) -> String {
        self.loop_depth += 1;
        let out = f(self);
        self.loop_depth -= 1;
        out
    }

    /// The cfg gating a Rust type name, if any.
    fn cfg_of(&self, name: &str) -> Option<String> {
        self.cfg_by_name.get(name).cloned().flatten()
    }

    /// The `type_` name marker for a wrapped type: present where the type is
    /// defined in the `.x` files, absent where it is a primitive, so a wrapper
    /// over a primitive reads like the primitive writers it sits beside.
    fn type_marker(&self, type_: &Type) -> &'static str {
        if self.owner_module(type_).is_some() {
            "type_"
        } else {
            ""
        }
    }

    /// The module a wrapper method belongs in: the one holding the type it
    /// wraps, found by digging to the innermost named type.
    ///
    /// `None` for a wrapper over a builtin scalar, which has no type of its own
    /// and so no file to live in.
    fn owner_module(&self, type_: &Type) -> Option<String> {
        match type_ {
            Type::Ident(name) => {
                if self.type_info.resolve_typedef_to_builtin(type_).is_some() {
                    None
                } else {
                    Some(mod_name(name))
                }
            }
            Type::Optional(inner) | Type::Array { element_type: inner, .. } => {
                self.owner_module(inner)
            }
            Type::VarArray { element_type, .. } => self.owner_module(element_type),
            _ => None,
        }
    }

    /// The method-name suffix for an XDR type, e.g. `transaction_envelope`,
    /// `option_account_id`, `vec_operation`.
    fn suffix(&self, type_: &Type) -> String {
        match type_ {
            Type::Int => "i32".to_string(),
            Type::UnsignedInt => "u32".to_string(),
            Type::Hyper => "i64".to_string(),
            Type::UnsignedHyper => "u64".to_string(),
            Type::Bool => "bool".to_string(),
            // Stellar XDR uses no float or double types, so `ConstWriter` has
            // no serializer for them (the streaming `WriteXdr` for f32/f64 is
            // likewise unimplemented). Fail at generation time rather than
            // emit a call to a method that does not exist.
            Type::Float | Type::Double => {
                unimplemented!("const XDR serialization of float and double is not supported")
            }
            Type::OpaqueFixed(_) => "opaque_fixed".to_string(),
            Type::OpaqueVar(_) => "opaque_var".to_string(),
            Type::String(_) => "string".to_string(),
            Type::Ident(name) => {
                // A typedef of a builtin scalar is a transparent Rust alias, so
                // it shares the underlying scalar's name rather than getting a
                // duplicate method of its own.
                if let Some(builtin) = self.type_info.resolve_typedef_to_builtin(type_) {
                    self.suffix(builtin)
                } else {
                    type_name(name).to_snake_case()
                }
            }
            Type::Optional(inner) => format!("option_{}", self.suffix(inner)),
            Type::Array { element_type, .. } => format!("array_{}", self.suffix(element_type)),
            Type::VarArray { element_type, .. } => format!("vec_{}", self.suffix(element_type)),
        }
    }

    /// The statements that serialize `acc`, a value of `type_`.
    ///
    /// `acc` is either a place expression (`is_ref == false`, e.g. `v.foo` or
    /// `s[i]`) or an identifier already bound to a reference (`is_ref == true`,
    /// e.g. the binding of a `match` arm). `parent` is the type being
    /// serialized, which decides whether a cyclic reference is wrapped.
    fn encode(&mut self, type_: &Type, acc: &str, is_ref: bool, parent: Option<&str>) -> String {
        match type_ {
            Type::Int => format!("self.write_i32({});", value(acc, is_ref)),
            Type::UnsignedInt => format!("self.write_u32({});", value(acc, is_ref)),
            Type::Hyper => format!("self.write_i64({});", value(acc, is_ref)),
            Type::UnsignedHyper => format!("self.write_u64({});", value(acc, is_ref)),
            Type::Bool => format!("self.write_bool({});", value(acc, is_ref)),
            Type::Float | Type::Double => {
                unimplemented!("const XDR serialization of float and double is not supported")
            }
            Type::OpaqueFixed(_) => {
                format!("self.write_fixed_opaque({});", reference(acc, is_ref))
            }
            // The variable-length byte cases only ever occur inside a `View`
            // type, where the value is a borrowing `BytesMView`/`StringMView`,
            // each of which exposes a const `as_slice`.
            Type::OpaqueVar(_) | Type::String(_) => {
                format!("self.write_var_opaque({acc}.as_slice());")
            }
            Type::Ident(_) => {
                if let Some(builtin) = self.type_info.resolve_typedef_to_builtin(type_) {
                    let builtin = builtin.clone();
                    return self.encode(&builtin, acc, is_ref, parent);
                }
                // Where the ident is cyclic with `parent`, its `View` form is
                // already a reference to the value, so it is passed along
                // as-is rather than borrowed again; a `match` binding of one is
                // a double reference that auto-deref resolves at the call.
                let view = const_view_type(type_, parent, self.type_info, self.view_required);
                let arg = if view.starts_with('&') {
                    acc.to_string()
                } else {
                    reference(acc, is_ref)
                };
                format!("self.write_type_{}({arg});", self.suffix(type_))
            }
            Type::Optional(_) => {
                let (name, by_value) = self.need_option(type_, parent);
                let arg = if by_value {
                    value(acc, is_ref)
                } else {
                    reference(acc, is_ref)
                };
                format!("self.{name}({arg});")
            }
            Type::VarArray { .. } => {
                let name = self.need_vec(type_);
                format!("self.{name}({});", reference(acc, is_ref))
            }
            // A fixed array is a plain `[T; N]` in both the owned and `View`
            // forms, so it is walked inline rather than given a method: there
            // is no wrapper type to name one after.
            Type::Array { element_type, size } => {
                let index = self.index_name();
                let element_type = element_type.clone();
                let acc = acc.to_string();
                let idx = index.clone();
                // A container holds its elements by value, so an element is
                // never a cyclic reference back to the enclosing type.
                let elem = self.in_loop(|c| {
                    c.encode(&element_type, &format!("{acc}[{idx}]"), false, None)
                });
                let len = self.type_info.size_to_literal(size);
                format!(
                    "{{ let mut {index} = 0usize; while {index} < {len} {{ {elem} {index} += 1; }} }}"
                )
            }
        }
    }

    /// Register (if new) and name the method serializing an `Option` type.
    fn need_option(&mut self, type_: &Type, parent: Option<&str>) -> (String, bool) {
        let Type::Optional(inner) = type_ else {
            unreachable!("need_option called with a non-optional type")
        };
        let param = const_view_type(type_, parent, self.type_info, self.view_required);
        // Where the option is cyclic the `View` form holds a reference to the
        // inner value rather than the value itself, which is a different Rust
        // type and so needs a method of its own. Such an option is a pair of
        // `Copy` words, so it is taken by value rather than by reference.
        let by_value = param.starts_with("Option<&");
        let marker = if by_value { "option_ref_" } else { "option_" };
        let name = format!("write_{}{marker}{}", self.type_marker(inner), self.suffix(inner));

        if !self.wrappers.contains_key(&name) {
            // Each wrapper is its own method, so its bindings start fresh.
            self.loop_depth = 0;
            let inner_encode = self.encode(inner, "v", true, parent);
            let body = format!(
                "match v {{ Some(v) => {{ self.write_u32(1); {inner_encode} }} None => {{ self.write_u32(0); }} }}"
            );
            let doc = format!(
                "Serializes an optional {}, mirroring `<{} as WriteXdr>::write_xdr`.",
                self.doc_name(inner),
                self.owned_doc_type(type_, parent),
            );
            self.wrappers.insert(
                name.clone(),
                ConstWriterMethodOutput {
                    name: name.clone(),
                    generics: String::new(),
                    param_type: if by_value {
                        param.clone()
                    } else {
                        format!("&{param}")
                    },
                    body,
                    cfg: self.wrapper_cfg(inner),
                    doc,
                    module: self.owner_module(inner),
                },
            );
        }
        (name, by_value)
    }

    /// Register (if new) and name the method serializing a `VecM` type.
    fn need_vec(&mut self, type_: &Type) -> String {
        let Type::VarArray { element_type, .. } = type_ else {
            unreachable!("need_vec called with a non-var-array type")
        };
        let name = format!(
            "write_{}vec_{}",
            self.type_marker(element_type),
            self.suffix(element_type)
        );

        if !self.wrappers.contains_key(&name) {
            // Each wrapper is its own method, so its bindings start fresh.
            self.loop_depth = 0;
            let element_type = element_type.clone();
            // `VecMView` borrows its elements as one slice, so each element is
            // held by value and is never a cyclic reference.
            let elem = self.in_loop(|c| c.encode(&element_type, "s[i]", false, None));
            let body = format!(
                "let s = v.as_slice(); let len = s.len(); self.write_len(len); \
                 let mut i = 0usize; while i < len {{ {elem} i += 1; }}"
            );
            // The max length is a const parameter rather than a fixed size, so
            // one method serves every `VecM` of this element type whatever its
            // declared maximum.
            let elem_ty =
                const_view_base_type(&element_type, self.type_info, self.view_required);
            let doc = format!(
                "Serializes a variable-length array of {}, mirroring `<VecM<{}, MAX> as WriteXdr>::write_xdr`.",
                self.doc_name(&element_type),
                type_ref(&element_type, None, self.type_info),
            );
            self.wrappers.insert(
                name.clone(),
                ConstWriterMethodOutput {
                    name: name.clone(),
                    generics: "<const MAX: u32>".to_string(),
                    param_type: format!("&VecMView<'_, {elem_ty}, MAX>"),
                    body,
                    cfg: self.wrapper_cfg(&element_type),
                    doc,
                    module: self.owner_module(&element_type),
                },
            );
        }
        name
    }

    /// The cfg a wrapper method needs: the one gating the type it names in its
    /// signature, so the method is present exactly where that type is.
    fn wrapper_cfg(&self, inner: &Type) -> Option<String> {
        match inner {
            Type::Ident(name) => {
                if self.type_info.resolve_typedef_to_builtin(inner).is_some() {
                    None
                } else {
                    self.cfg_of(&type_name(name))
                }
            }
            Type::Optional(t) | Type::Array { element_type: t, .. } => self.wrapper_cfg(t),
            Type::VarArray { element_type, .. } => self.wrapper_cfg(element_type),
            _ => None,
        }
    }

    /// A prose name for a type, for the generated doc comment.
    fn doc_name(&self, type_: &Type) -> String {
        match type_ {
            Type::Ident(name) if self.type_info.resolve_typedef_to_builtin(type_).is_none() => {
                format!("[`{}`]", type_name(name))
            }
            _ => format!("`{}`", type_ref(type_, None, self.type_info)),
        }
    }

    /// The owned Rust type a method's `View` parameter corresponds to, for the
    /// generated doc comment.
    fn owned_doc_type(&self, type_: &Type, parent: Option<&str>) -> String {
        type_ref(type_, parent, self.type_info)
    }

    /// The method serializing one definition, or `None` for definitions that
    /// are not written (consts, and typedefs of builtin scalars, which are
    /// transparent Rust aliases served by the underlying scalar's method).
    fn definition_method(&mut self, def: &Definition) -> Option<ConstWriterMethodOutput> {
        let name = type_name(def.name());
        let cfg = def.cfg().map(|c| c.render());
        // Each method is its own scope, so its bindings start fresh.
        self.loop_depth = 0;

        let body = match def {
            Definition::Const(_) => return None,
            Definition::Typedef(t) if is_builtin_type(&t.type_) => return None,
            Definition::Typedef(t) => self.encode(&t.type_, "v.0", false, None),
            // An enum is encoded as its discriminant value, an XDR int.
            Definition::Enum(_) => "self.write_i32(*v as i32);".to_string(),
            Definition::Struct(s) => {
                let parent = name.clone();
                s.members
                    .iter()
                    .map(|m| {
                        let acc = format!("v.{}", field_name(&m.name));
                        self.encode(&m.type_, &acc, false, Some(&parent))
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            Definition::Union(u) => self.union_body(u, &name),
        };

        // A type that owns heap data is serialized through its borrowing `View`
        // form, which is the only form const evaluation can hold; every other
        // type is serialized directly.
        let param_type = if self.view_required.contains(&name) {
            format!("&{name}View<'_>")
        } else {
            format!("&{name}")
        };

        let doc = format!(
            "Serializes a [`{name}`], mirroring `<{name} as WriteXdr>::write_xdr`."
        );

        Some(ConstWriterMethodOutput {
            name: format!("write_type_{}", name.to_snake_case()),
            generics: String::new(),
            param_type,
            body,
            cfg,
            doc,
            module: Some(mod_name(def.name())),
        })
    }

    /// The body serializing a union: its discriminant, then the payload of the
    /// selected arm.
    fn union_body(&mut self, u: &Union, name: &str) -> String {
        let discriminant_type = type_ref(&u.discriminant.type_, None, self.type_info);
        let discriminant_is_builtin = is_builtin_type(&u.discriminant.type_)
            || matches!(&u.discriminant.type_, Type::Ident(n) if {
                self.type_info.definitions.get(&type_name(n))
                    .map(|d| matches!(d, Definition::Typedef(t) if is_builtin_type(&t.type_)))
                    .unwrap_or(false)
            });
        let prefix = if discriminant_is_builtin {
            String::new()
        } else {
            self.type_info
                .discriminant_enum(&u.discriminant.type_)
                .map(|e| e.member_prefix.clone())
                .unwrap_or_default()
        };

        // Both the owned and `View` forms expose a const `discriminant`, so the
        // body mirrors the owned type's `write_xdr`.
        let d = self.encode(&u.discriminant.type_, "d", false, None);

        // The match scrutinee is the `View` form where the union owns heap
        // data, since that is the type the parameter holds.
        let scrutinee_type = if self.view_required.contains(name) {
            format!("{name}View")
        } else {
            name.to_string()
        };

        let arms = u
            .arms
            .iter()
            .flat_map(|arm| {
                self.union_arms(arm, name, &discriminant_type, discriminant_is_builtin, &prefix)
            })
            .collect::<Vec<_>>()
            .join(" ");

        let body = format!(
            "let d = v.discriminant(); {d} \
             #[allow(clippy::match_same_arms)] \
             match v {{ {arms} }}"
        );
        body.replace(SCRUTINEE, &scrutinee_type)
    }

    /// The match arms for one union arm, one per case value it covers.
    fn union_arms(
        &mut self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        prefix: &str,
    ) -> Vec<String> {
        arm.cases
            .iter()
            .map(|case| {
                let (case_name, _) = case_value(
                    discriminant_type,
                    discriminant_is_builtin,
                    &case.value,
                    prefix,
                );
                let cfg = arm
                    .cfg
                    .as_ref()
                    .map(|c| format!("#[cfg({})] ", c.render()))
                    .unwrap_or_default();
                match &arm.type_ {
                    Some(t) => {
                        let inner = self.encode(t, "value", true, Some(parent));
                        format!("{cfg}{SCRUTINEE}::{case_name}(value) => {{ {inner} }}")
                    }
                    None => format!("{cfg}{SCRUTINEE}::{case_name} => {{}}"),
                }
            })
            .collect()
    }
}

/// `acc` as a by-value expression: a `match` binding is a reference, so it is
/// dereferenced; a place expression is used as-is and copied.
fn value(acc: &str, is_ref: bool) -> String {
    if is_ref {
        format!("*{acc}")
    } else {
        acc.to_string()
    }
}

/// `acc` as a by-reference expression: a `match` binding already is one; a
/// place expression is borrowed.
fn reference(acc: &str, is_ref: bool) -> String {
    if is_ref {
        acc.to_string()
    } else {
        format!("&{acc}")
    }
}
