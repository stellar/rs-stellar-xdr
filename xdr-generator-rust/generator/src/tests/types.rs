use crate::types::base_type_ref;
use xdr_parser::ast::{Size, Type};

#[test]
fn test_base_type_ref() {
    assert_eq!(base_type_ref(&Type::Int, None), "i32");
    assert_eq!(base_type_ref(&Type::UnsignedHyper, None), "u64");
    assert_eq!(
        base_type_ref(&Type::OpaqueFixed(Size::Literal(32)), None),
        "[u8; 32]"
    );
}
