#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "serde"))]

use std::str::FromStr;

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    AccountId, AlphaNum4, AssetCode4, BytesM, ChangeTrustAsset, ChangeTrustOp, Hash, Memo,
    MuxedAccount, Operation, OperationBody, Preconditions, SequenceNumber, StringM, Transaction,
    TransactionEnvelope, TransactionExt, TransactionV1Envelope, Uint256, VecM,
};

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
        serde_json::to_string(&<_ as TryInto<Hash>>::try_into(
            *b"01234567890123456789013456789012"
        )?)?,
        "\"3031323334353637383930313233343536373839303133343536373839303132\""
    );
    #[cfg(feature = "curr")]
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
        <_ as TryInto<Hash>>::try_into(*b"01234567890123456789013456789012")?,
    );

    let v: AccountId =
        serde_json::from_str("\"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF\"")?;
    assert_eq!(
        v,
        AccountId::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")?
    );

    Ok(())
}

#[cfg(feature = "curr")]
#[test]
fn test_serde_tx() -> Result<(), Box<dyn std::error::Error>> {
    let te = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([
                0x3c, 0xb3, 0x61, 0xab, 0x62, 0x4b, 0x10, 0x70, 0x4c, 0x6c, 0xcf, 0x4f, 0xdb, 0x1e,
                0x40, 0x79, 0xd2, 0x3d, 0x68, 0xec, 0x2c, 0xd3, 0x22, 0xc2, 0x28, 0x34, 0xc4, 0x1a,
                0xe1, 0xe6, 0x4b, 0xd3,
            ])),
            fee: 0,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::Text("Stellar".as_bytes().try_into()?),
            operations: [Operation {
                source_account: Some(MuxedAccount::Ed25519(Uint256([
                    0x9b, 0x9f, 0xfa, 0xba, 0xcf, 0x46, 0x65, 0xb3, 0x57, 0x29, 0x76, 0xfb, 0x85,
                    0x09, 0x79, 0xcb, 0xc7, 0x6b, 0x9d, 0x67, 0x9c, 0x6b, 0xca, 0xeb, 0xd5, 0x9b,
                    0xbf, 0xb3, 0x43, 0xe8, 0xe9, 0x46,
                ]))),
                body: OperationBody::ChangeTrust(ChangeTrustOp {
                    line: ChangeTrustAsset::CreditAlphanum4(AlphaNum4 {
                        asset_code: AssetCode4(*b"ABCD"),
                        issuer: AccountId(stellar_xdr::PublicKey::PublicKeyTypeEd25519(Uint256([
                            0x43, 0xd0, 0x9f, 0x49, 0x2a, 0x2a, 0xe3, 0xaa, 0x0a, 0xed, 0x8e, 0xce,
                            0xdc, 0xb2, 0x26, 0xa4, 0xf7, 0x50, 0xa9, 0x0e, 0xcb, 0x4e, 0x09, 0xf9,
                            0xac, 0x76, 0x4a, 0x55, 0x37, 0xca, 0xd8, 0x77,
                        ]))),
                    }),
                    limit: i64::MAX,
                }),
            }]
            .to_vec()
            .try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: [].try_into()?,
    });
    let s = serde_json::to_string_pretty(&te)?;
    println!("{s}");
    assert_eq!(
        s,
        r#"{
  "tx": {
    "tx": {
      "source_account": "GA6LGYNLMJFRA4CMNTHU7WY6IB45EPLI5QWNGIWCFA2MIGXB4ZF5GQGY",
      "fee": 0,
      "seq_num": 1,
      "cond": "none",
      "memo": {
        "text": "Stellar"
      },
      "operations": [
        {
          "source_account": "GCNZ76V2Z5DGLM2XFF3PXBIJPHF4O245M6OGXSXL2WN37M2D5DUUNSOO",
          "body": {
            "change_trust": {
              "line": {
                "credit_alphanum4": {
                  "asset_code": "41424344",
                  "issuer": "GBB5BH2JFIVOHKQK5WHM5XFSE2SPOUFJB3FU4CPZVR3EUVJXZLMHOLOM"
                }
              },
              "limit": 9223372036854775807
            }
          }
        }
      ],
      "ext": "v0"
    },
    "signatures": []
  }
}"#,
    );

    Ok(())
}
