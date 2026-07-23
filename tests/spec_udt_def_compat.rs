#![cfg(feature = "std")]

// Tests for reusing the contract-spec `lib` field as a var-array of
// `SCSpecUDTDef`, and for the hash-carrying `SCSpecTypeUDTV2` reference type.
//
// The key backwards-compatibility property: an empty XDR `string` and an empty
// XDR var-array both encode as a single zero-length prefix (`00 00 00 00`), so
// a spec entry that did not set a `lib` is byte-for-byte identical under the
// old (`string lib<80>`) and new (`SCSpecUDTDef lib<>`) schemas.

use stellar_xdr::{
    Limits, ReadXdr, ScSpecType, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtDef, ScSpecUdtStructV0,
    WriteXdr,
};

/// Bytes exactly as the *old* `string lib<80>` schema would encode
/// `SCSpecUDTStructV0 { doc: "", lib: "", name: "Foo", fields: [] }`.
fn old_empty_lib_struct_bytes() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u32.to_be_bytes()); // doc: string len 0
    b.extend_from_slice(&0u32.to_be_bytes()); // lib: string len 0
    b.extend_from_slice(&3u32.to_be_bytes()); // name: string len 3
    b.extend_from_slice(b"Foo\0"); // name bytes, padded to 4
    b.extend_from_slice(&0u32.to_be_bytes()); // fields: vec count 0
    b
}

#[test]
fn old_empty_lib_struct_decodes_under_new_schema() {
    let bytes = old_empty_lib_struct_bytes();
    let decoded = ScSpecUdtStructV0::from_xdr(&bytes, Limits::none()).unwrap();
    assert!(decoded.lib.is_empty());
    assert_eq!(decoded.name.to_utf8_string_lossy(), "Foo");
    assert!(decoded.fields.is_empty());
    // Re-encoding produces the identical original bytes.
    assert_eq!(decoded.to_xdr(Limits::none()).unwrap(), bytes);
}

#[test]
fn new_empty_lib_struct_encodes_identically_to_old() {
    let s = ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: Vec::new().try_into().unwrap(),
        name: "Foo".try_into().unwrap(),
        fields: Vec::new().try_into().unwrap(),
    };
    assert_eq!(s.to_xdr(Limits::none()).unwrap(), old_empty_lib_struct_bytes());
}

#[test]
fn lib_and_hash_entries_round_trip() {
    let s = ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: vec![
            ScSpecUdtDef::Lib("mylib".try_into().unwrap()),
            ScSpecUdtDef::Hash([1, 2, 3, 4, 5, 6, 7, 8]),
        ]
        .try_into()
        .unwrap(),
        name: "Foo".try_into().unwrap(),
        fields: Vec::new().try_into().unwrap(),
    };
    let bytes = s.to_xdr(Limits::none()).unwrap();
    assert_eq!(ScSpecUdtStructV0::from_xdr(&bytes, Limits::none()).unwrap(), s);
}

#[test]
fn udt_v2_reference_round_trips_with_new_discriminant() {
    assert_eq!(ScSpecType::UdtV2 as i32, 2001);
    let def = ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 {
        hash: [9, 8, 7, 6, 5, 4, 3, 2],
        name: "Bar".try_into().unwrap(),
    });
    let bytes = def.to_xdr(Limits::none()).unwrap();
    // Discriminant is the first 4 bytes, big-endian.
    assert_eq!(&bytes[..4], &2001u32.to_be_bytes());
    assert_eq!(ScSpecTypeDef::from_xdr(&bytes, Limits::none()).unwrap(), def);
}
