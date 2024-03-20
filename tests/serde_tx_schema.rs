#![cfg(feature = "wasm")]

use stellar_xdr::{curr::TypeVariant, schema::Schema};

#[allow(clippy::too_many_lines)]
#[test]
fn test_serde_tx_schema() -> Result<(), Box<dyn std::error::Error>> {
    let schema: Schema = TypeVariant::ScVal.try_into()?;
    let s = serde_json::to_string_pretty(&schema.0)?;
    println!("{s}");
    Ok(())
}
