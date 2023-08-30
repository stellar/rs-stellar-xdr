#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "serde"))]

use stellar_xdr::{ScError, ScVal, VecM};

#[test]
fn test_serde_ser() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_string(&ScVal::Error(ScError::Contract(4)))?,
        "{\"error\":{\"contract\":4}}"
    );
    assert_eq!(
        serde_json::to_string(&VecM::<ScVal, 1>::try_from([ScVal::Error(ScError::Contract(4))])?)?,
        "[{\"error\":{\"contract\":4}}]"
    );

    Ok(())
}
