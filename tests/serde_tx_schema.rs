#![cfg(feature = "curr")]
#![cfg(all(feature = "std", feature = "serde", feature = "cli"))]

use stellar_xdr::curr as xdr;
use xdr::TransactionEnvelope;

#[cfg(feature = "curr")]
#[allow(clippy::too_many_lines)]
#[test]
fn test_serde_tx_schema() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schemars::schema_for!(TransactionEnvelope);
    let s = serde_json::to_string_pretty(&schema)?;
    let json: serde_json::Value = serde_json::from_str(&s)?;
    insta::assert_json_snapshot!(json);
    
    Ok(())
}
