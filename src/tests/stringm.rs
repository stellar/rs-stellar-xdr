#![cfg(feature = "std")]

use crate::{Error, StringM};

use std::str::FromStr;

#[test]
fn stringm_from_str_at_max() {
    // Exactly at the limit should succeed.
    let result = StringM::<3>::from_str("abc");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"abc");
}

#[test]
fn stringm_from_str_within_max() {
    // Within the limit should succeed.
    let result = StringM::<3>::from_str("ab");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"ab");
}

#[test]
fn stringm_from_str_exceeding_max() {
    let result = StringM::<3>::from_str("abcd");
    assert_eq!(result, Err(Error::LengthExceedsMax));
}

#[test]
fn stringm_from_str_empty() {
    let result = StringM::<3>::from_str("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"");
}

// The JSON form escapes bytes outside printable ASCII as \xNN, so a valid
// StringM can serialize to more than MAX characters. The schema must not
// impose a maxLength the crate's own output can exceed.
// https://github.com/stellar/rs-stellar-xdr/issues/621
#[cfg(feature = "schemars")]
#[test]
fn stringm_json_schema_has_no_max_length() {
    let s = StringM::<28>::try_from(b"pago alquiler mayo se\xc3\xb1orabc".to_vec()).unwrap();
    let json = serde_json::to_value(&s).unwrap();
    assert!(json.as_str().unwrap().chars().count() > 28);

    let schema = serde_json::to_value(schemars::schema_for!(StringM<28>)).unwrap();
    assert_eq!(schema.get("maxLength"), None);
    assert!(schema["description"]
        .as_str()
        .unwrap()
        .contains("at most 28 bytes"));
}
