//! Emission of const XDR serialization code.
//!
//! For each generated type an inherent `const fn const_write_xdr` is emitted
//! that serializes the value through a `ConstWriter` (defined in `header.rs`).
//! It is emitted on the borrowing `View` form of a type when the type owns heap
//! data (and therefore has a `View` variant), and on the type itself otherwise;
//! either way the value is `Copy`/borrow-only, which const evaluation requires.
//! This module drives the recursion over each field's type; the actual Rust
//! code it emits lives in the jinja templates (`const_encode_*.rs.jinja`) and,
//! for the surrounding function, in the per-type templates.
//!
//! The emitted code mirrors, statement for statement, the trait-based
//! `WriteXdr::write_xdr` implementations of the owned types, including their
//! depth and length limit accounting, but expressed with only const-compatible
//! operations: `while` loops instead of `for`, direct calls to each concrete
//! field type's own `const_write_xdr` instead of trait dispatch, and a
//! `ConstWriter` that records errors internally (see `header.rs`) instead of
//! returning a `Result`, which cannot be dropped in a const function.

use askama::Template;
use xdr_parser::ast::{Size, Type};
use xdr_parser::types::TypeInfo;

// Every emitted code snippet, down to a single scalar write, is produced from a
// template in `templates/const_encode_*.rs.jinja` paired with one of the types
// below, so all generated Rust text lives in the templates rather than inline
// string literals here.

/// `w.write_i32(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_i32.rs.jinja", escape = "none")]
struct I32Encode {
    arg: String,
}

/// `w.write_u32(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_u32.rs.jinja", escape = "none")]
struct U32Encode {
    arg: String,
}

/// `w.write_i64(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_i64.rs.jinja", escape = "none")]
struct I64Encode {
    arg: String,
}

/// `w.write_u64(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_u64.rs.jinja", escape = "none")]
struct U64Encode {
    arg: String,
}

/// `w.write_bool(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_bool.rs.jinja", escape = "none")]
struct BoolEncode {
    arg: String,
}

/// `w.write_fixed_opaque(<arg>);`
#[derive(Template)]
#[template(path = "const_encode_opaque_fixed.rs.jinja", escape = "none")]
struct OpaqueFixedEncode {
    arg: String,
}

/// `w.write_len_prefixed(<acc>.as_slice());`
#[derive(Template)]
#[template(path = "const_encode_len_prefixed.rs.jinja", escape = "none")]
struct LenPrefixedEncode {
    acc: String,
}

/// `<acc>.const_write_xdr(w);`
#[derive(Template)]
#[template(path = "const_encode_ident.rs.jinja", escape = "none")]
struct IdentEncode {
    acc: String,
}

/// `{ w.enter_depth(); <acc>.const_write_xdr(w); w.leave_depth(); }`
#[derive(Template)]
#[template(path = "const_encode_boxed_ident.rs.jinja", escape = "none")]
struct BoxedIdentEncode {
    acc: String,
}

#[derive(Template)]
#[template(path = "const_encode_optional.rs.jinja", escape = "none")]
struct OptionalEncode {
    scrutinee: String,
    var: String,
    inner: String,
}

#[derive(Template)]
#[template(path = "const_encode_array.rs.jinja", escape = "none")]
struct ArrayEncode {
    index: String,
    size: String,
    elem: String,
}

#[derive(Template)]
#[template(path = "const_encode_var_array.rs.jinja", escape = "none")]
struct VarArrayEncode {
    acc: String,
    slice: String,
    index: String,
    len: String,
    elem: String,
}

/// Render a const-encode snippet template into the Rust code it emits.
fn render<T: Template>(snippet: &T) -> String {
    snippet.render().expect("render const encode template")
}

/// Allocate the next unique suffix for temporary variable names, so that nested
/// containers do not shadow each other's loop variables.
fn next(counter: &mut u32) -> u32 {
    let n = *counter;
    *counter += 1;
    n
}

/// The value form of an accessor, for passing a `Copy` scalar by value.
/// A reference accessor (`is_ref`) is dereferenced; a place accessor is used
/// as-is.
fn value(acc: &str, is_ref: bool) -> String {
    if is_ref {
        format!("*{acc}")
    } else {
        acc.to_string()
    }
}

/// The reference form of an accessor. A reference accessor is used as-is; a
/// place accessor is borrowed.
fn reference(acc: &str, is_ref: bool) -> String {
    if is_ref {
        acc.to_string()
    } else {
        format!("&{acc}")
    }
}

/// Emit const-encoding statements that serialize the value denoted by `acc`
/// into the `ConstWriter` bound to the local `w`.
///
/// `acc` is either a place expression (`is_ref == false`, e.g. `self.foo` or
/// `s[i]`) or an identifier already bound to a reference (`is_ref == true`,
/// e.g. the `v` of an `Option` or union arm match). Method calls and field
/// accesses work uniformly on both because Rust auto-references and
/// auto-dereferences the receiver; only by-value and by-reference uses differ.
///
/// The variable-length cases (`OpaqueVar`/`String`/`VarArray`) only ever occur
/// inside a `View` type, where the value is a borrowing `BytesMView`/
/// `StringMView`/`VecMView`; each exposes a const `as_slice`.
///
/// `parent` is the type being generated (for cyclic detection) and `boxed`
/// marks positions where a cyclic ident is `Box`-wrapped by the owned
/// `type_ref` (direct fields and `Option` contents, but not array/vec
/// elements). At such a position a `Box` consumes its own depth level in
/// `WriteXdr`, which is mirrored here even though the `View` form borrows
/// instead of boxing.
fn encode_type(
    type_: &Type,
    type_info: &TypeInfo,
    acc: &str,
    is_ref: bool,
    parent: Option<&str>,
    boxed: bool,
    counter: &mut u32,
) -> String {
    match type_ {
        Type::Int => render(&I32Encode {
            arg: value(acc, is_ref),
        }),
        Type::UnsignedInt => render(&U32Encode {
            arg: value(acc, is_ref),
        }),
        Type::Hyper => render(&I64Encode {
            arg: value(acc, is_ref),
        }),
        Type::UnsignedHyper => render(&U64Encode {
            arg: value(acc, is_ref),
        }),
        Type::Bool => render(&BoolEncode {
            arg: value(acc, is_ref),
        }),
        // Stellar XDR uses no float or double types, so `ConstWriter` has no
        // serializer for them (the streaming `WriteXdr` for f32/f64 is likewise
        // unimplemented). Fail at generation time rather than emit a call to a
        // method that does not exist, should one ever be introduced.
        Type::Float | Type::Double => {
            unimplemented!("const XDR serialization of float and double is not supported")
        }
        Type::OpaqueFixed(_) => render(&OpaqueFixedEncode {
            arg: reference(acc, is_ref),
        }),
        Type::OpaqueVar(_) | Type::String(_) => render(&LenPrefixedEncode {
            acc: acc.to_string(),
        }),
        Type::Ident(_) => {
            // A typedef of a builtin scalar is emitted as a transparent Rust
            // type alias, so encode it as the underlying scalar. Every other
            // ident is a generated type with its own `const_write_xdr` (the
            // owned type when it owns no heap, otherwise its `View` form).
            if let Some(builtin) = type_info.resolve_typedef_to_builtin(type_) {
                encode_type(builtin, type_info, acc, is_ref, parent, boxed, counter)
            } else if boxed && crate::types::is_cyclic(type_, parent, type_info) {
                // A cyclic ident here is `Box`-wrapped in the owned type, and
                // `<Box<_> as WriteXdr>::write_xdr` consumes its own depth
                // level. Mirror it around the (auto-dereferenced) call; the
                // `View` form borrows instead of boxing but the depth
                // accounting must match.
                render(&BoxedIdentEncode {
                    acc: acc.to_string(),
                })
            } else {
                render(&IdentEncode {
                    acc: acc.to_string(),
                })
            }
        }
        Type::Optional(inner) => {
            let n = next(counter);
            let var = format!("__v{n}");
            let inner_encode = encode_type(inner, type_info, &var, true, parent, true, counter);
            render(&OptionalEncode {
                scrutinee: reference(acc, is_ref),
                var,
                inner: inner_encode,
            })
        }
        Type::Array { element_type, size } => {
            let n = next(counter);
            let index = format!("__i{n}");
            let elem = encode_type(
                element_type,
                type_info,
                &format!("{acc}[{index}]"),
                false,
                parent,
                false,
                counter,
            );
            render(&ArrayEncode {
                index,
                size: size_literal(size, type_info),
                elem,
            })
        }
        Type::VarArray { element_type, .. } => {
            let n = next(counter);
            let slice = format!("__s{n}");
            let index = format!("__i{n}");
            let len = format!("__len{n}");
            let elem = encode_type(
                element_type,
                type_info,
                &format!("{slice}[{index}]"),
                false,
                parent,
                false,
                counter,
            );
            render(&VarArrayEncode {
                acc: acc.to_string(),
                slice,
                index,
                len,
                elem,
            })
        }
    }
}

/// Resolve an array `Size` to a literal usize expression.
fn size_literal(size: &Size, type_info: &TypeInfo) -> String {
    type_info.size_to_literal(size)
}

/// Emit the const-encoding statements for a single struct member. `parent` is
/// the struct's type name, matching the `resolve_type` call that decided any
/// `Box` wrapping.
pub(crate) fn member_body(
    type_: &Type,
    type_info: &TypeInfo,
    parent: &str,
    member_name: &str,
) -> String {
    let mut counter = 0;
    encode_type(
        type_,
        type_info,
        &format!("self.{member_name}"),
        false,
        Some(parent),
        true,
        &mut counter,
    )
}

/// Emit the const-encoding statements for a union's discriminant. The
/// discriminant value is bound to the local `d` before this code runs. A
/// discriminant is never `Box`-wrapped.
pub(crate) fn discriminant_body(type_: &Type, type_info: &TypeInfo) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "d", false, None, false, &mut counter)
}

/// Emit the const-encoding statements for a non-void union arm. The arm value
/// is bound to the reference `v` before this code runs. `parent` is the
/// union's type name, matching the `resolve_type` call that decided any `Box`
/// wrapping.
pub(crate) fn union_arm_body(type_: &Type, type_info: &TypeInfo, parent: &str) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "v", true, Some(parent), true, &mut counter)
}

/// Emit the const-encoding statements for a typedef newtype's inner value. A
/// newtype's inner type is resolved without a parent, so it is never
/// `Box`-wrapped.
pub(crate) fn newtype_body(type_: &Type, type_info: &TypeInfo) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "self.0", false, None, false, &mut counter)
}
