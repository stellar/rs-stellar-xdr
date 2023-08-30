#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "serde"))]

use stellar_xdr::{ScError, ScVal};

#[test]
fn test_serde_ser() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_string(&ScVal::Error(ScError::Contract(4)))?,
        "{\"error\":{\"contract\":4}}"
    );

    Ok(())
}
