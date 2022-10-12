#![cfg(all(feature = "std", feature = "serde"))]

use stellar_xdr::{BytesM, StringM, VecM};

#[test]
fn test_serde() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<VecM<u8>>>::try_into("hello world")?)?,
        "[104,101,108,108,111,32,119,111,114,108,100]"
    );
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<BytesM>>::try_into("hello world")?)?,
        "\"68656c6c6f20776f726c64\""
    );
    assert_eq!(
        serde_json::to_string(&<_ as TryInto<StringM>>::try_into("hello world")?)?,
        "\"hello world\""
    );

    Ok(())
}
