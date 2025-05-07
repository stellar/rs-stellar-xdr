#![cfg(feature = "curr")]
#![cfg(all(feature = "std", feature = "serde"))]

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{ConfigSettingEntry, ScNonceKey, ScVal, SequenceNumber};

#[test]
fn test_serde_typedef_64bit() -> Result<(), Box<dyn std::error::Error>> {
    let x = SequenceNumber(123);
    let s = serde_json::to_string(&x)?;
    println!("{s}");
    assert_eq!(s, r#""123""#,);
    let x_rt = serde_json::from_str::<SequenceNumber>(&s)?;
    assert_eq!(x_rt, x);
    Ok(())
}

#[test]
fn test_serde_union_64bit() -> Result<(), Box<dyn std::error::Error>> {
    let x = ScVal::I64(-123);
    let s = serde_json::to_string(&x)?;
    println!("{s}");
    assert_eq!(s, r#"{"i64":"-123"}"#,);
    let x_rt = serde_json::from_str::<ScVal>(&s)?;
    assert_eq!(x_rt, x);

    let x = ScVal::U64(123);
    let s = serde_json::to_string(&x)?;
    println!("{s}");
    assert_eq!(s, r#"{"u64":"123"}"#,);
    let x_rt = serde_json::from_str::<ScVal>(&s)?;
    assert_eq!(x_rt, x);
    Ok(())
}

#[test]
fn test_serde_struct_field_64bit() -> Result<(), Box<dyn std::error::Error>> {
    let x = ScNonceKey { nonce: 123 };
    let s = serde_json::to_string(&x)?;
    println!("{s}");
    assert_eq!(s, r#"{"nonce":"123"}"#,);
    let x_rt = serde_json::from_str::<ScNonceKey>(&s)?;
    assert_eq!(x_rt, x);
    Ok(())
}

#[test]
fn test_serde_vecm_containing_64bit_int() -> Result<(), Box<dyn std::error::Error>> {
    let x = ConfigSettingEntry::LiveSorobanStateSizeWindow([1, 2, 3, u64::MAX].try_into()?);
    let s = serde_json::to_string_pretty(&x)?;
    println!("{s}");
    assert_eq!(
        s,
        r#"{
  "live_soroban_state_size_window": [
    "1",
    "2",
    "3",
    "18446744073709551615"
  ]
}"#,
    );
    let x_rt = serde_json::from_str::<ConfigSettingEntry>(&s)?;
    assert_eq!(x_rt, x);
    Ok(())
}
