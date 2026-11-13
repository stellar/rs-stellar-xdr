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

#[cfg(feature = "schemars")]
#[test]
fn stringm_json_schema_has_no_max_length() {
    // 28 bytes, where `ñ` takes 2 bytes. The JSON form escapes each byte
    // outside printable ASCII as `\xNN`, so the JSON string is longer than 28
    // characters, and the schema must not bound it with maxLength.
    let s = StringM::<28>::try_from("pago alquiler mayo se\u{f1}orabc".as_bytes()).unwrap();
    assert_eq!(s.len(), 28);
    let json = serde_json::to_value(&s).unwrap();
    assert_eq!(json, r"pago alquiler mayo se\xc3\xb1orabc");
    assert_eq!(json.as_str().unwrap().len(), 34);

    let schema = serde_json::to_value(schemars::schema_for!(StringM<28>)).unwrap();
    assert_eq!(schema["type"], "string");
    assert!(schema.get("maxLength").is_none());
    assert_eq!(
        schema["description"],
        r"XDR string of at most 28 bytes. Backslashes and bytes outside printable ASCII are escaped (for example as \xNN), so the JSON string can be longer than 28 characters."
    );
}
