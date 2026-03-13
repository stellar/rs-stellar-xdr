use crate::naming::{field_name, type_name};

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
