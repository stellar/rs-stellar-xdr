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
