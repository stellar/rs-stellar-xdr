//! Emission of const XDR serialization code.
//!
//! For each generated type an inherent `const fn const_write_xdr` is emitted that
//! serializes the value through a `ConstWriter` (defined in `header.rs`). This
//! module drives the recursion over each field's type; the actual Rust code it
//! emits lives in the jinja templates (`const_encode_*.rs.jinja`) and, for the
//! surrounding function, in the per-type templates.
//!
//! The emitted code mirrors, statement for statement, the trait-based
//! `WriteXdr::write_xdr` implementations, including their depth and length
//! limit accounting, but expressed with only const-compatible operations:
//! `while` loops instead of `for`, direct calls to each concrete field type's
//! own `const_write_xdr` instead of trait dispatch, and a `ConstWriter` that
//! records errors internally (see `header.rs`) instead of returning a
//! `Result`, which cannot be dropped in a const function.

use askama::Template;
use xdr_parser::ast::{Size, Type};
use xdr_parser::types::TypeInfo;

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
fn encode_type(
    type_: &Type,
    type_info: &TypeInfo,
    acc: &str,
    is_ref: bool,
    counter: &mut u32,
) -> String {
    match type_ {
        Type::Int => format!("w.write_i32({});", value(acc, is_ref)),
        Type::UnsignedInt => format!("w.write_u32({});", value(acc, is_ref)),
        Type::Hyper => format!("w.write_i64({});", value(acc, is_ref)),
        Type::UnsignedHyper => format!("w.write_u64({});", value(acc, is_ref)),
        Type::Bool => format!("w.write_bool({});", value(acc, is_ref)),
        Type::Float => format!("w.write_f32({});", value(acc, is_ref)),
        Type::Double => format!("w.write_f64({});", value(acc, is_ref)),
        Type::OpaqueFixed(_) => format!("w.write_fixed_opaque({});", reference(acc, is_ref)),
        Type::OpaqueVar(_) | Type::String(_) => {
            format!("w.write_len_prefixed({acc}.0.as_slice());")
        }
        Type::Ident(_) => {
            // A typedef of a builtin scalar is emitted as a transparent Rust
            // type alias, so encode it as the underlying scalar. Every other
            // ident is a generated type with its own `const_write_xdr` (method
            // resolution transparently dereferences a `Box` for cyclic types).
            if let Some(builtin) = type_info.resolve_typedef_to_builtin(type_) {
                encode_type(builtin, type_info, acc, is_ref, counter)
            } else {
                format!("{acc}.const_write_xdr(w);")
            }
        }
        Type::Optional(inner) => {
            let n = next(counter);
            let var = format!("__v{n}");
            let inner_encode = encode_type(inner, type_info, &var, true, counter);
            OptionalEncode {
                scrutinee: reference(acc, is_ref),
                var,
                inner: inner_encode,
            }
            .render()
            .expect("render const_encode_optional")
        }
        Type::Array { element_type, size } => {
            let n = next(counter);
            let index = format!("__i{n}");
            let elem =
                encode_type(element_type, type_info, &format!("{acc}[{index}]"), false, counter);
            ArrayEncode {
                index,
                size: size_literal(size, type_info),
                elem,
            }
            .render()
            .expect("render const_encode_array")
        }
        Type::VarArray { element_type, .. } => {
            let n = next(counter);
            let slice = format!("__s{n}");
            let index = format!("__i{n}");
            let len = format!("__len{n}");
            let elem =
                encode_type(element_type, type_info, &format!("{slice}[{index}]"), false, counter);
            VarArrayEncode {
                acc: acc.to_string(),
                slice,
                index,
                len,
                elem,
            }
            .render()
            .expect("render const_encode_var_array")
        }
    }
}

/// Resolve an array `Size` to a literal usize expression.
fn size_literal(size: &Size, type_info: &TypeInfo) -> String {
    type_info.size_to_literal(size)
}

/// Emit the const-encoding statements for a single struct member.
pub(crate) fn member_body(type_: &Type, type_info: &TypeInfo, member_name: &str) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, &format!("self.{member_name}"), false, &mut counter)
}

/// Emit the const-encoding statements for a union's discriminant. The
/// discriminant value is bound to the local `d` before this code runs.
pub(crate) fn discriminant_body(type_: &Type, type_info: &TypeInfo) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "d", false, &mut counter)
}

/// Emit the const-encoding statements for a non-void union arm. The arm value
/// is bound to the reference `v` before this code runs.
pub(crate) fn union_arm_body(type_: &Type, type_info: &TypeInfo) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "v", true, &mut counter)
}

/// Emit the const-encoding statements for a typedef newtype's inner value.
pub(crate) fn newtype_body(type_: &Type, type_info: &TypeInfo) -> String {
    let mut counter = 0;
    encode_type(type_, type_info, "self.0", false, &mut counter)
}
