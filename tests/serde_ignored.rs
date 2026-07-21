#![cfg(all(
    feature = "std",
    feature = "serde",
    feature = "serde_json",
    feature = "serde_ignored",
    feature = "type_enum"
))]

use stellar_xdr::{Type, TypeVariant};

// Unknown fields nested inside a type with a custom Deserialize impl (the
// 128/256-bit parts and muxed account types) must still be reported by
// from_json_collecting_ignored_fields, even though those types deserialize a
// value-or-string form that previously relied on an untagged enum.
#[test]
fn test_ignored_fields_in_custom_deserialize_type() {
    let json = br#"{"i128":{"hi":1,"lo":2,"bogus":3}}"#;
    let (_, ignored) =
        Type::from_json_collecting_ignored_fields(TypeVariant::ScVal, &json[..]).unwrap();
    assert_eq!(ignored.len(), 1, "expected one ignored field: {ignored:?}");
    assert!(
        ignored[0].ends_with("bogus"),
        "expected the ignored field path to point at the unknown field: {ignored:?}"
    );
}

#[test]
fn test_no_ignored_fields_in_custom_deserialize_type() {
    let json = br#"{"i128":{"hi":1,"lo":2}}"#;
    let (_, ignored) =
        Type::from_json_collecting_ignored_fields(TypeVariant::ScVal, &json[..]).unwrap();
    assert!(ignored.is_empty(), "unexpected ignored fields: {ignored:?}");
}

// The string form of these types must continue to deserialize.
#[test]
fn test_custom_deserialize_string_form_still_works() {
    let json = br#"{"i128":"1"}"#;
    let (_, ignored) =
        Type::from_json_collecting_ignored_fields(TypeVariant::ScVal, &json[..]).unwrap();
    assert!(ignored.is_empty(), "unexpected ignored fields: {ignored:?}");
}
