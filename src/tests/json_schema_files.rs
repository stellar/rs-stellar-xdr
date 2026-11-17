#![cfg(feature = "std")]

// Every JSON schema published in xdr-json/ must load in a strict draft 2019-09
// validator.
#[test]
fn xdr_json_schemas_load_in_strict_validator() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("xdr-json");
    let mut count = 0;
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let schema: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        if let Err(e) = jsonschema::draft201909::new(&schema) {
            failures.push(format!("{}: {e}", path.display()));
        }
        count += 1;
    }
    assert!(count > 0, "no schema files found in {}", dir.display());
    assert!(failures.is_empty(), "{failures:#?}");
}

// JSON the crate emits for a valid StringM must validate against the crate's
// own published JSON schema, even when the string contains escaped bytes.
#[cfg(all(feature = "serde_json", feature = "base64"))]
#[test]
fn stringm_json_validates_against_xdr_json_schema() {
    use crate::{Limits, Memo, ReadXdr};

    // MEMO_TEXT at its maximum length of 28 bytes, where ñ takes 2 bytes, so
    // the escaped JSON string is longer than the maximum.
    let memo = Memo::from_xdr_base64(
        "AAAAAQAAABxwYWdvIGFscXVpbGVyIG1heW8gc2XDsW9yYWJj",
        Limits::none(),
    )
    .unwrap();
    let json = serde_json::to_value(&memo).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"text": "pago alquiler mayo se\\xc3\\xb1orabc"})
    );

    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../../xdr-json/Memo.json")).unwrap();
    let validator = jsonschema::draft201909::new(&schema).unwrap();
    let errors: Vec<String> = validator
        .iter_errors(&json)
        .map(|e| e.to_string())
        .collect();
    assert_eq!(errors, Vec::<String>::new());
}
