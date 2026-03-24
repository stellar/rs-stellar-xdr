use crate::types::{base_type_ref, size_to_string};
use xdr_parser::ast::{Size, Type};

#[test]
fn test_base_type_ref_int() {
    assert_eq!(base_type_ref(&Type::Int, None), "i32");
}

#[test]
fn test_base_type_ref_unsigned_int() {
    assert_eq!(base_type_ref(&Type::UnsignedInt, None), "u32");
}

#[test]
fn test_base_type_ref_hyper() {
    assert_eq!(base_type_ref(&Type::Hyper, None), "i64");
}

#[test]
fn test_base_type_ref_unsigned_hyper() {
    assert_eq!(base_type_ref(&Type::UnsignedHyper, None), "u64");
}

#[test]
fn test_base_type_ref_float() {
    assert_eq!(base_type_ref(&Type::Float, None), "f32");
}

#[test]
fn test_base_type_ref_double() {
    assert_eq!(base_type_ref(&Type::Double, None), "f64");
}

#[test]
fn test_base_type_ref_bool() {
    assert_eq!(base_type_ref(&Type::Bool, None), "bool");
}

#[test]
fn test_base_type_ref_opaque_fixed() {
    assert_eq!(
        base_type_ref(&Type::OpaqueFixed { size: Size::Literal { literal: 32 } }, None),
        "[u8; 32]"
    );
}

#[test]
fn test_base_type_ref_opaque_fixed_named_size() {
    assert_eq!(
        base_type_ref(
            &Type::OpaqueFixed { size: Size::Named { named: "KEY_SIZE".to_string() } },
            None
        ),
        "[u8; KeySize]"
    );
}

#[test]
fn test_base_type_ref_opaque_var_with_max() {
    assert_eq!(
        base_type_ref(&Type::OpaqueVar { max_size: Some(Size::Literal { literal: 64 }) }, None),
        "BytesM::<64>"
    );
}

#[test]
fn test_base_type_ref_opaque_var_unbounded() {
    assert_eq!(base_type_ref(&Type::OpaqueVar { max_size: None }, None), "BytesM");
}

#[test]
fn test_base_type_ref_string_with_max() {
    assert_eq!(
        base_type_ref(&Type::String { max_size: Some(Size::Literal { literal: 100 }) }, None),
        "StringM::<100>"
    );
}

#[test]
fn test_base_type_ref_string_unbounded() {
    assert_eq!(base_type_ref(&Type::String { max_size: None }, None), "StringM");
}

#[test]
fn test_base_type_ref_ident() {
    assert_eq!(
        base_type_ref(&Type::Ident { ident: "public_key".to_string() }, None),
        "PublicKey"
    );
}

#[test]
fn test_base_type_ref_optional() {
    assert_eq!(
        base_type_ref(&Type::Optional { element_type: Box::new(Type::Int) }, None),
        "Option<i32>"
    );
}

#[test]
fn test_base_type_ref_array() {
    assert_eq!(
        base_type_ref(
            &Type::Array {
                element_type: Box::new(Type::UnsignedInt),
                size: Size::Literal { literal: 4 },
            },
            None
        ),
        "[u32; 4]"
    );
}

#[test]
fn test_base_type_ref_var_array_with_max() {
    assert_eq!(
        base_type_ref(
            &Type::VarArray {
                element_type: Box::new(Type::Int),
                max_size: Some(Size::Literal { literal: 10 }),
            },
            None
        ),
        "VecM<i32, 10>"
    );
}

#[test]
fn test_base_type_ref_var_array_unbounded() {
    assert_eq!(
        base_type_ref(
            &Type::VarArray {
                element_type: Box::new(Type::Int),
                max_size: None,
            },
            None
        ),
        "VecM<i32>"
    );
}

#[test]
fn test_base_type_ref_nested_optional_array() {
    assert_eq!(
        base_type_ref(
            &Type::Optional { element_type: Box::new(Type::Array {
                element_type: Box::new(Type::UnsignedHyper),
                size: Size::Literal { literal: 2 },
            }) },
            None
        ),
        "Option<[u64; 2]>"
    );
}

#[test]
fn test_size_to_string_literal() {
    assert_eq!(size_to_string(&Size::Literal { literal: 32 }), "32");
}

#[test]
fn test_size_to_string_named() {
    assert_eq!(
        size_to_string(&Size::Named { named: "MAX_SIZE".to_string() }),
        "MaxSize"
    );
}
