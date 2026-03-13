use crate::naming::{field_name, type_name};
use crate::types::base_type_ref;
use xdr_parser::ast::{Size, Type};

#[test]
fn test_type_name() {
    assert_eq!(type_name("public_key"), "PublicKey");
    assert_eq!(type_name("PUBLIC_KEY_TYPE_ED25519"), "PublicKeyTypeEd25519");
}

#[test]
fn test_field_name() {
    assert_eq!(field_name("publicKey"), "public_key");
    assert_eq!(field_name("type"), "type_");
}

#[test]
fn test_base_type_ref() {
    assert_eq!(base_type_ref(&Type::Int, None), "i32");
    assert_eq!(base_type_ref(&Type::UnsignedHyper, None), "u64");
    assert_eq!(
        base_type_ref(&Type::OpaqueFixed(Size::Literal(32)), None),
        "[u8; 32]"
    );
}
