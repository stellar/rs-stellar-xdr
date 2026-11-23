#![cfg(all(feature = "std", feature = "serde"))]

use crate::{BytesM, Hash, StringM, VecM};

use crate::{
    AccountId, AuthenticatedMessage, AuthenticatedMessageV0, ContractEvent, ContractEventType,
    Curve25519Public, Curve25519Secret, HmacSha256Key, HmacSha256Mac, Int128Parts, PeerAddress,
    PeerAddressIp, ShortHashSeed, StellarMessage,
};

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
fn test_serde_type_field_uses_sep51_json_key() -> Result<(), Box<dyn std::error::Error>> {
    let event = ContractEvent {
        type_: ContractEventType::Contract,
        ..Default::default()
    };

    // The SEP-51 JSON uses the key `type`, not the previously escaped `type_`.
    let json = r#"{"ext":"v0","contract_id":null,"type":"contract","body":{"v0":{"topics":[],"data":{"bool":false}}}}"#;
    // The legacy JSON that older versions of this crate emitted, using `type_`.
    let legacy_json = r#"{"ext":"v0","contract_id":null,"type_":"contract","body":{"v0":{"topics":[],"data":{"bool":false}}}}"#;

    // Serializes with the SEP-51 key `type`.
    assert_eq!(serde_json::to_string(&event)?, json);

    // Deserializes from the SEP-51 key `type`.
    assert_eq!(serde_json::from_str::<ContractEvent>(json)?, event);

    // The legacy escaped key `type_` is still accepted via a serde alias.
    assert_eq!(serde_json::from_str::<ContractEvent>(legacy_json)?, event);

    Ok(())
}

#[test]
fn test_serde_der_vecm_enforces_max() {
    assert_eq!(
        serde_json::from_str::<VecM<u32, 2>>("[1,2]").unwrap(),
        VecM::<u32, 2>::try_from(vec![1, 2]).unwrap()
    );
    assert!(serde_json::from_str::<VecM<u32, 2>>("[1,2,3]").is_err());
}

#[test]
fn test_serde_inline_opaque_uses_sep51_hex() {
    let v = PeerAddressIp::IPv4([0xab, 0xcd, 0xef, 0x01]);
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(r#"{"i_pv4":"abcdef01"}"#.to_string())
    );
    assert_eq!(
        serde_json::from_str::<PeerAddressIp>(r#"{"i_pv4":"abcdef01"}"#).map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<PeerAddressIp>(r#"{"i_pv4":[171,205,239,1]}"#)
            .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = PeerAddressIp::IPv6([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(r#"{"i_pv6":"00000000000000000000000000000001"}"#.to_string())
    );
    assert_eq!(
        serde_json::from_str::<PeerAddressIp>(r#"{"i_pv6":"00000000000000000000000000000001"}"#)
            .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<PeerAddressIp>(r#"{"i_pv6":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]}"#)
            .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = ShortHashSeed { seed: [0xab; 16] };
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(r#"{"seed":"abababababababababababababababab"}"#.to_string())
    );
    assert_eq!(
        serde_json::from_str::<ShortHashSeed>(r#"{"seed":"abababababababababababababababab"}"#)
            .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<ShortHashSeed>(
            r#"{"seed":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = Curve25519Public { key: [0xab; 32] };
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
                .to_string()
        )
    );
    assert_eq!(
        serde_json::from_str::<Curve25519Public>(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<Curve25519Public>(
            r#"{"key":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = Curve25519Secret { key: [0xab; 32] };
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
                .to_string()
        )
    );
    assert_eq!(
        serde_json::from_str::<Curve25519Secret>(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<Curve25519Secret>(
            r#"{"key":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = HmacSha256Key { key: [0xab; 32] };
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
                .to_string()
        )
    );
    assert_eq!(
        serde_json::from_str::<HmacSha256Key>(
            r#"{"key":"abababababababababababababababababababababababababababababababab"}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<HmacSha256Key>(
            r#"{"key":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    let v = HmacSha256Mac { mac: [0xab; 32] };
    assert_eq!(
        serde_json::to_string(&v).map_err(|e| e.to_string()),
        Ok(
            r#"{"mac":"abababababababababababababababababababababababababababababababab"}"#
                .to_string()
        )
    );
    assert_eq!(
        serde_json::from_str::<HmacSha256Mac>(
            r#"{"mac":"abababababababababababababababababababababababababababababababab"}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
    assert_eq!(
        serde_json::from_str::<HmacSha256Mac>(
            r#"{"mac":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#
        )
        .map_err(|e| e.to_string()),
        Ok(v.clone())
    );
}

#[test]
fn test_serde_der_inline_opaque_accepts_any_case_hex() {
    for json in [r#"{"i_pv4":"ABCDEF01"}"#, r#"{"i_pv4":"AbCdEf01"}"#] {
        assert_eq!(
            serde_json::from_str::<PeerAddressIp>(json).map_err(|e| e.to_string()),
            Ok(PeerAddressIp::IPv4([0xab, 0xcd, 0xef, 0x01])),
            "{json}"
        );
    }
}

#[test]
fn test_serde_inline_opaque_nested() {
    let pa = PeerAddress {
        ip: PeerAddressIp::IPv6([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
        port: 11625,
        num_failures: 2,
    };
    let json =
        r#"{"ip":{"i_pv6":"00000000000000000000000000000001"},"port":11625,"num_failures":2}"#;
    assert_eq!(
        serde_json::to_string(&pa).map_err(|e| e.to_string()),
        Ok(json.to_string())
    );
    assert_eq!(
        serde_json::from_str::<PeerAddress>(json).map_err(|e| e.to_string()),
        Ok(pa.clone())
    );

    let msg = AuthenticatedMessage::V0(AuthenticatedMessageV0 {
        sequence: 1,
        message: StellarMessage::Peers(vec![pa].try_into().unwrap()),
        mac: HmacSha256Mac { mac: [0xab; 32] },
    });
    let json = r#"{"v0":{"sequence":"1","message":{"peers":[{"ip":{"i_pv6":"00000000000000000000000000000001"},"port":11625,"num_failures":2}]},"mac":{"mac":"abababababababababababababababababababababababababababababababab"}}}"#;
    assert_eq!(
        serde_json::to_string(&msg).map_err(|e| e.to_string()),
        Ok(json.to_string())
    );
    assert_eq!(
        serde_json::from_str::<AuthenticatedMessage>(json).map_err(|e| e.to_string()),
        Ok(msg.clone())
    );
}

#[test]
fn test_serde_der_inline_opaque_rejects_invalid() {
    for (json, err) in [
        (
            r#"{"i_pv4":"010203"}"#,
            r#"invalid value: string "010203", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 17"#,
        ),
        (
            r#"{"i_pv4":"0102030405"}"#,
            r#"invalid value: string "0102030405", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 21"#,
        ),
        (
            r#"{"i_pv4":"0102030"}"#,
            r#"invalid value: string "0102030", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 18"#,
        ),
        (
            r#"{"i_pv4":"0102030g"}"#,
            r#"invalid value: string "0102030g", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 19"#,
        ),
        (
            r#"{"i_pv4":""}"#,
            r#"invalid value: string "", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 11"#,
        ),
        (
            r#"{"i_pv4":"0x01020304"}"#,
            r#"invalid value: string "0x01020304", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 21"#,
        ),
        (
            r#"{"i_pv4":" 01020304"}"#,
            r#"invalid value: string " 01020304", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 20"#,
        ),
        (
            r#"{"i_pv4":"01020304 "}"#,
            r#"invalid value: string "01020304 ", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 20"#,
        ),
        (
            r#"{"i_pv4":"000102030405060708090a0b0c0d0e0f"}"#,
            r#"invalid value: string "000102030405060708090a0b0c0d0e0f", expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 43"#,
        ),
        (
            r#"{"i_pv4":[1,2,3]}"#,
            r"invalid length 3, expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 16",
        ),
        (
            r#"{"i_pv4":[1,2,3,4,5]}"#,
            r"invalid length 5, expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 20",
        ),
        (
            r#"{"i_pv4":[1,2,3,256]}"#,
            r"invalid value: integer `256`, expected u8 at line 1 column 19",
        ),
        (
            r#"{"i_pv4":1}"#,
            r"invalid type: integer `1`, expected a hex string encoding 4 bytes or an array of 4 integers at line 1 column 10",
        ),
        (
            r#"{"i_pv6":"01020304"}"#,
            r#"invalid value: string "01020304", expected a hex string encoding 16 bytes or an array of 16 integers at line 1 column 19"#,
        ),
        (
            r#"{"i_pv6":[1,2,3,4]}"#,
            r"invalid length 4, expected a hex string encoding 16 bytes or an array of 16 integers at line 1 column 18",
        ),
    ] {
        assert_eq!(
            serde_json::from_str::<PeerAddressIp>(json).map_err(|e| e.to_string()),
            Err(err.to_string()),
            "{json}"
        );
    }
}

#[test]
fn test_serde_enum_member_starting_with_digit() -> Result<(), Box<dyn std::error::Error>> {
    use crate::BinaryFuseFilterType;
    for (v, s) in [
        (BinaryFuseFilterType::B8Bit, "\"8_bit\""),
        (BinaryFuseFilterType::B16Bit, "\"16_bit\""),
        (BinaryFuseFilterType::B32Bit, "\"32_bit\""),
    ] {
        assert_eq!(serde_json::to_string(&v)?, s);
        assert_eq!(serde_json::from_str::<BinaryFuseFilterType>(s)?, v);
    }
    // The names used before the rename are still accepted.
    for (v, s) in [
        (BinaryFuseFilterType::B8Bit, "\"b8_bit\""),
        (BinaryFuseFilterType::B16Bit, "\"b16_bit\""),
        (BinaryFuseFilterType::B32Bit, "\"b32_bit\""),
    ] {
        assert_eq!(serde_json::from_str::<BinaryFuseFilterType>(s)?, v);
    }
    Ok(())
}
