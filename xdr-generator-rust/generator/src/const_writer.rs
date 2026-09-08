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
//! This module decides what each method does: which writer method serializes
//! each value, how the value is passed, and what wrappers are needed. It emits
//! that as data ([`ConstWriterMethodOutput`]) and the
//! `const_writer_impl.rs.jinja` template renders it as Rust. The rendered code
//! produces the same bytes as the trait-based `WriteXdr::write_xdr`
//! implementations of the owned types, expressed with only const-compatible
//! operations: `while` loops instead of `for`, and direct calls to each
//! concrete type's writer method instead of trait dispatch.

use std::collections::{BTreeMap, HashMap, HashSet};

use heck::ToSnakeCase;
use xdr_parser::ast::{Definition, Type, Union, UnionArm, XdrSpec};
use xdr_parser::types::{is_builtin_type, TypeInfo};

use crate::naming::{case_value, field_name, mod_name, type_name};
use crate::output::{
    ConstDocName, ConstEncode, ConstLoop, ConstPass, ConstSubject, ConstUnionArm, ConstWriterBody,
    ConstWriterMethodOutput, ConstWriterOutput,
};
use crate::types::{const_ref_base_type, const_ref_type, type_ref};

/// Build the `ConstWriter` methods for an entire spec.
///
/// `cfg_by_name` maps a Rust type name to the cfg it is gated by, cleared to
/// `None` where the same name appears in several `#ifdef` branches and so is
/// always present.
pub(crate) fn build(
    spec: &XdrSpec,
    type_info: &TypeInfo,
    ref_required: &HashSet<String>,
    cfg_by_name: &HashMap<String, Option<String>>,
) -> ConstWriterOutput {
    let mut c = Collector {
        type_info,
        ref_required,
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

struct Collector<'a> {
    type_info: &'a TypeInfo,
    ref_required: &'a HashSet<String>,
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
    fn in_loop<F: FnOnce(&mut Self) -> ConstEncode>(&mut self, f: F) -> ConstEncode {
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
            Type::Optional(inner)
            | Type::Array {
                element_type: inner,
                ..
            } => self.owner_module(inner),
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

    /// The call that serializes `acc`, a value of `type_`.
    ///
    /// `acc` is either a place expression (`is_ref == false`, e.g. `v.foo` or
    /// `s[i]`) or an identifier already bound to a reference (`is_ref == true`,
    /// e.g. the binding of a `match` arm). `parent` is the type being
    /// serialized, which decides whether a cyclic reference is wrapped.
    fn encode(
        &mut self,
        type_: &Type,
        acc: &str,
        is_ref: bool,
        parent: Option<&str>,
    ) -> ConstEncode {
        let call = |method: &str, pass: ConstPass| ConstEncode {
            loops: Vec::new(),
            acc: acc.to_string(),
            is_ref,
            method: method.to_string(),
            pass,
        };
        match type_ {
            Type::Int => call("write_i32", ConstPass::Value),
            Type::UnsignedInt => call("write_u32", ConstPass::Value),
            Type::Hyper => call("write_i64", ConstPass::Value),
            Type::UnsignedHyper => call("write_u64", ConstPass::Value),
            Type::Bool => call("write_bool", ConstPass::Value),
            Type::Float | Type::Double => {
                unimplemented!("const XDR serialization of float and double is not supported")
            }
            Type::OpaqueFixed(_) => call("write_fixed_opaque", ConstPass::Ref),
            // The variable-length byte cases only ever occur inside a `Ref`
            // type, where the value is a borrowing `BytesMRef`/`StringMRef`,
            // each of which exposes a const `as_slice`.
            Type::OpaqueVar(_) | Type::String(_) => call("write_var_opaque", ConstPass::Slice),
            Type::Ident(_) => {
                if let Some(builtin) = self.type_info.resolve_typedef_to_builtin(type_) {
                    let builtin = builtin.clone();
                    return self.encode(&builtin, acc, is_ref, parent);
                }
                // Where the ident is cyclic with `parent`, its `Ref` form is
                // already a reference to the value, so it is passed along
                // as-is rather than borrowed again; a `match` binding of one is
                // a double reference that auto-deref resolves at the call.
                let ref_ty = const_ref_type(type_, parent, self.type_info, self.ref_required);
                let pass = if ref_ty.starts_with('&') {
                    ConstPass::AsIs
                } else {
                    ConstPass::Ref
                };
                call(&format!("write_type_{}", self.suffix(type_)), pass)
            }
            Type::Optional(_) => {
                let (name, by_value) = self.need_option(type_, parent);
                let pass = if by_value {
                    ConstPass::Value
                } else {
                    ConstPass::Ref
                };
                call(&name, pass)
            }
            Type::VarArray { .. } => {
                let name = self.need_vec(type_);
                call(&name, ConstPass::Ref)
            }
            // A fixed array is a plain `[T; N]` in both the owned and `Ref`
            // forms, so it is walked inline rather than given a method: there
            // is no wrapper type to name one after.
            Type::Array { element_type, size } => {
                let index = self.index_name();
                let element_type = element_type.clone();
                let elem_acc = format!("{acc}[{index}]");
                // A container holds its elements by value, so an element is
                // never a cyclic reference back to the enclosing type.
                let mut elem = self.in_loop(|c| c.encode(&element_type, &elem_acc, false, None));
                elem.loops.insert(
                    0,
                    ConstLoop {
                        index,
                        len: self.type_info.size_to_literal(size),
                    },
                );
                elem
            }
        }
    }

    /// Register (if new) and name the method serializing an `Option` type.
    fn need_option(&mut self, type_: &Type, parent: Option<&str>) -> (String, bool) {
        let Type::Optional(inner) = type_ else {
            unreachable!("need_option called with a non-optional type")
        };
        let param = const_ref_type(type_, parent, self.type_info, self.ref_required);
        // Where the option is cyclic the `Ref` form holds a reference to the
        // inner value rather than the value itself, which is a different Rust
        // type and so needs a method of its own. Such an option is a pair of
        // `Copy` words, so it is taken by value rather than by reference.
        let by_value = param.starts_with("Option<&");
        let marker = if by_value { "option_ref_" } else { "option_" };
        let name = format!(
            "write_{}{marker}{}",
            self.type_marker(inner),
            self.suffix(inner)
        );

        // A wrapper over a builtin scalar is written by hand on `ConstWriter`,
        // beside the scalar serializer it calls, so it is named but not
        // generated.
        if self.owner_module(inner).is_none() {
            assert_hand_written(&name);
            return (name, by_value);
        }

        if !self.wrappers.contains_key(&name) {
            // Each wrapper is its own method, so its bindings start fresh.
            self.loop_depth = 0;
            let inner_encode = self.encode(inner, "v", true, parent);
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
                    cfg: self.wrapper_cfg(inner),
                    module: self.owner_module(inner),
                    subject: ConstSubject::Option {
                        inner: self.doc_name(inner),
                        owned: type_ref(type_, parent, self.type_info),
                    },
                    body: ConstWriterBody::Option(inner_encode),
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

        // As for options, a wrapper over a builtin scalar is hand-written.
        if self.owner_module(element_type).is_none() {
            assert_hand_written(&name);
            return name;
        }

        if !self.wrappers.contains_key(&name) {
            // Each wrapper is its own method, so its bindings start fresh.
            self.loop_depth = 0;
            let element_type = element_type.clone();
            // `VecMRef` borrows its elements as one slice, so each element is
            // held by value and is never a cyclic reference.
            let elem = self.in_loop(|c| c.encode(&element_type, "s[i]", false, None));
            // The max length is a const parameter rather than a fixed size, so
            // one method serves every `VecM` of this element type whatever its
            // declared maximum.
            let elem_ty = const_ref_base_type(&element_type, self.type_info, self.ref_required);
            self.wrappers.insert(
                name.clone(),
                ConstWriterMethodOutput {
                    name: name.clone(),
                    generics: "<const MAX: u32>".to_string(),
                    param_type: format!("&VecMRef<'_, {elem_ty}, MAX>"),
                    cfg: self.wrapper_cfg(&element_type),
                    module: self.owner_module(&element_type),
                    subject: ConstSubject::Vec {
                        inner: self.doc_name(&element_type),
                        elem: type_ref(&element_type, None, self.type_info),
                    },
                    body: ConstWriterBody::Vec(elem),
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
            Type::Optional(t)
            | Type::Array {
                element_type: t, ..
            } => self.wrapper_cfg(t),
            Type::VarArray { element_type, .. } => self.wrapper_cfg(element_type),
            _ => None,
        }
    }

    /// How a type is named in a generated doc comment: a link for a type from
    /// the `.x` files, plain code for a builtin.
    fn doc_name(&self, type_: &Type) -> ConstDocName {
        match type_ {
            Type::Ident(name) if self.type_info.resolve_typedef_to_builtin(type_).is_none() => {
                ConstDocName {
                    name: type_name(name),
                    link: true,
                }
            }
            _ => ConstDocName {
                name: type_ref(type_, None, self.type_info),
                link: false,
            },
        }
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
            Definition::Typedef(t) => {
                ConstWriterBody::Newtype(self.encode(&t.type_, "v.0", false, None))
            }
            Definition::Enum(_) => ConstWriterBody::Enum,
            Definition::Struct(s) => {
                let parent = name.clone();
                ConstWriterBody::Struct(
                    s.members
                        .iter()
                        .map(|m| {
                            let acc = format!("v.{}", field_name(&m.name));
                            self.encode(&m.type_, &acc, false, Some(&parent))
                        })
                        .collect(),
                )
            }
            Definition::Union(u) => self.union_body(u, &name),
        };

        // A type that owns heap data is serialized through its borrowing `Ref`
        // form, which is the only form const evaluation can hold; every other
        // type is serialized directly.
        let param_type = if self.ref_required.contains(&name) {
            format!("&{name}Ref<'_>")
        } else {
            format!("&{name}")
        };

        Some(ConstWriterMethodOutput {
            name: format!("write_type_{}", name.to_snake_case()),
            generics: String::new(),
            param_type,
            cfg,
            module: Some(mod_name(def.name())),
            subject: ConstSubject::Type(name),
            body,
        })
    }

    /// The body serializing a union: its discriminant, then the payload of the
    /// selected arm.
    fn union_body(&mut self, u: &Union, name: &str) -> ConstWriterBody {
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

        // Both the owned and `Ref` forms expose a const `discriminant`, so the
        // body mirrors the owned type's `write_xdr`.
        let discriminant = self.encode(&u.discriminant.type_, "d", false, None);

        // The match scrutinee is the `Ref` form where the union owns heap
        // data, since that is the type the parameter holds.
        let scrutinee = if self.ref_required.contains(name) {
            format!("{name}Ref")
        } else {
            name.to_string()
        };

        let arms = u
            .arms
            .iter()
            .flat_map(|arm| {
                self.union_arms(
                    arm,
                    name,
                    &discriminant_type,
                    discriminant_is_builtin,
                    &prefix,
                )
            })
            .collect();

        ConstWriterBody::Union {
            scrutinee,
            discriminant,
            arms,
        }
    }

    /// The match arms for one union arm, one per case value it covers.
    fn union_arms(
        &mut self,
        arm: &UnionArm,
        parent: &str,
        discriminant_type: &str,
        discriminant_is_builtin: bool,
        prefix: &str,
    ) -> Vec<ConstUnionArm> {
        arm.cases
            .iter()
            .map(|case| {
                let (case_name, _) = case_value(
                    discriminant_type,
                    discriminant_is_builtin,
                    &case.value,
                    prefix,
                );
                ConstUnionArm {
                    cfg: arm.cfg.as_ref().map(|c| c.render()),
                    case_name,
                    payload: arm
                        .type_
                        .as_ref()
                        .map(|t| self.encode(t, "value", true, Some(parent))),
                }
            })
            .collect()
    }
}

/// Check that a wrapper over a builtin scalar is one `ConstWriter` defines by
/// hand.
///
/// The hand-written set covers the scalars `ConstWriter` can serialize. A
/// wrapper over `opaque`, `string` or a fixed array would also have no file to
/// live in, and the XDR has never needed one; fail at generation time rather
/// than emit a call to a method that does not exist.
fn assert_hand_written(name: &str) {
    const HAND_WRITTEN: &[&str] = &[
        "write_option_i32",
        "write_option_u32",
        "write_option_i64",
        "write_option_u64",
        "write_option_bool",
        "write_vec_i32",
        "write_vec_u32",
        "write_vec_i64",
        "write_vec_u64",
        "write_vec_bool",
    ];
    assert!(
        HAND_WRITTEN.contains(&name),
        "`{name}` wraps a builtin scalar, so it has no generated file to live \
         in, but `ConstWriter` does not define it. Write it by hand in \
         xdr-generator-rust/generator/header.rs beside the other \
         `write_option_`/`write_vec_` methods."
    );
}
