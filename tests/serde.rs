#![cfg(all(feature = "std", feature = "serde"))]

use stellar_xdr::{BytesM, Hash, StringM, VecM};

use stellar_xdr::{AccountId, ContractEvent, ContractEventType, Int128Parts};

use std::str::FromStr;

#[test]
fn test_serde_ser() -> Result<(), Box<dyn std::error::Error>> {
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
    assert_eq!(
        serde_json::to_string(&<_ as Into<Hash>>::into(
            *b"01234567890123456789013456789012"
        ))?,
        "\"3031323334353637383930313233343536373839303133343536373839303132\""
    );
    assert_eq!(
        serde_json::to_string(&AccountId::from_str(
            "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"
        )?)?,
        "\"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF\""
    );
    Ok(())
}

#[test]
fn test_serde_der() -> Result<(), Box<dyn std::error::Error>> {
    let v: VecM<u8> = serde_json::from_str("[104,101,108,108,111,32,119,111,114,108,100]")?;
    assert_eq!(v, <_ as TryInto<VecM<u8>>>::try_into("hello world")?);

    let v: BytesM = serde_json::from_str("\"68656c6c6f20776f726c64\"")?;
    assert_eq!(v, <_ as TryInto<BytesM>>::try_into("hello world")?);

    let v: StringM = serde_json::from_str("\"hello world\"")?;
    assert_eq!(v, <_ as TryInto<StringM>>::try_into("hello world")?,);

    let v: Hash = serde_json::from_str(
        "\"3031323334353637383930313233343536373839303133343536373839303132\"",
    )?;
    assert_eq!(
        v,
        <_ as Into<Hash>>::into(*b"01234567890123456789013456789012"),
    );

    assert_eq!(
        AccountId::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")?,
        serde_json::from_str("\"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF\"")?,
    );

    Ok(())
}

#[test]
fn test_structs_that_ser_to_string_and_dual_der() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_string(&Int128Parts { hi: 1, lo: 2 })?,
        "\"18446744073709551618\"",
    );
    assert_eq!(
        serde_json::from_str::<Int128Parts>("\"18446744073709551618\"")?,
        Int128Parts { hi: 1, lo: 2 },
    );
    assert_eq!(
        serde_json::from_str::<Int128Parts>(r#"{"hi":1,"lo":2}"#)?,
        Int128Parts { hi: 1, lo: 2 },
    );
    Ok(())
}

#[test]
fn test_serde_type_field_uses_unescaped_json_key() -> Result<(), Box<dyn std::error::Error>> {
    let event = ContractEvent {
        type_: ContractEventType::Contract,
        ..Default::default()
    };

    // Serializes with the SEP-51 JSON key `type`, not the escaped Rust name `type_`.
    let json = serde_json::to_string(&event)?;
    assert!(json.contains(r#""type":"#), "expected `type` key in {json}");
    assert!(!json.contains("type_"), "unexpected `type_` key in {json}");

    // Deserializes from the correct SEP-51 key `type`.
    assert_eq!(serde_json::from_str::<ContractEvent>(&json)?, event);

    // Deserializes from the legacy escaped key `type_` (backward-compat alias).
    let legacy = json.replace(r#""type":"#, r#""type_":"#);
    assert_eq!(serde_json::from_str::<ContractEvent>(&legacy)?, event);

    Ok(())
}
