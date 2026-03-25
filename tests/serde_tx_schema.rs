#![cfg(all(feature = "schemars", feature = "serde", feature = "alloc"))]

#[allow(clippy::too_many_lines)]
#[test]
fn test_serde_tx_schema() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schemars::schema_for!(stellar_xdr::ScVal);
    let s = serde_json::to_string_pretty(&schema)?;
    println!("{s}");
    Ok(())
}
