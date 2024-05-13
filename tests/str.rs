#![cfg(feature = "curr")]
#![cfg(feature = "std")]

use stellar_xdr::curr as stellar_xdr;

use stellar_xdr::{
    AccountId, AssetCode, AssetCode12, AssetCode4, ClaimableBalanceId, Error, Hash, MuxedAccount,
    MuxedAccountMed25519, NodeId, PublicKey, ScAddress, SignerKey, SignerKeyEd25519SignedPayload,
    Uint256,
};

use std::str::FromStr;

#[test]
fn public_key_from_str() {
    let v = PublicKey::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    assert_eq!(v, Ok(PublicKey::PublicKeyTypeEd25519(Uint256([0; 32]))));
}

#[test]
fn public_key_to_string() {
    let s = PublicKey::PublicKeyTypeEd25519(Uint256([0; 32])).to_string();
    assert_eq!(
        s,
        "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"
    );
}

#[test]
fn account_id_from_str() {
    let v = AccountId::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    assert_eq!(
        v,
        Ok(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0; 32]))))
    );
}

#[test]
fn account_id_to_string() {
    let s = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0; 32]))).to_string();
    assert_eq!(
        s,
        "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"
    );
}

#[test]
fn muxed_account_med_25519_to_string() {
    let s = MuxedAccountMed25519 {
        ed25519: Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),
        id: 9_223_372_036_854_775_808,
    }
    .to_string();
    assert_eq!(
        s,
        "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK"
    );
}

#[test]
fn muxed_account_med_25519_from_str() {
    let v = MuxedAccountMed25519::from_str(
        "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK",
    );
    assert_eq!(
        v,
        Ok(MuxedAccountMed25519 {
            ed25519: Uint256([
                0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
                0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
                0xfc, 0x7f, 0xe8, 0x9a,
            ]),
            id: 9_223_372_036_854_775_808,
        }),
    );
}

#[test]
fn muxed_account_to_string_with_ed25519() {
    let s = MuxedAccount::Ed25519(Uint256([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))
    .to_string();
    assert_eq!(
        s,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn muxed_account_from_str_with_ed25519() {
    let v = MuxedAccount::from_str("GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ");
    assert_eq!(
        v,
        Ok(MuxedAccount::Ed25519(Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]))),
    );
}

#[test]
fn muxed_account_to_string_with_muxed_ed25519() {
    let s = MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
        ed25519: Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),
        id: 9_223_372_036_854_775_808,
    })
    .to_string();
    assert_eq!(
        s,
        "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK"
    );
}

#[test]
fn muxed_account_from_str_with_muxed_ed25519() {
    let v = MuxedAccount::from_str(
        "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK",
    );
    assert_eq!(
        v,
        Ok(MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
            ed25519: Uint256([
                0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
                0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
                0xfc, 0x7f, 0xe8, 0x9a,
            ]),
            id: 9_223_372_036_854_775_808,
        })),
    );
}

#[test]
fn node_id_from_str() {
    let v = NodeId::from_str("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
    assert_eq!(
        v,
        Ok(NodeId(PublicKey::PublicKeyTypeEd25519(Uint256([0; 32]))))
    );
}

#[test]
fn node_id_to_string() {
    let s = NodeId(PublicKey::PublicKeyTypeEd25519(Uint256([0; 32]))).to_string();
    assert_eq!(
        s,
        "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"
    );
}

#[test]
fn signer_key_ed25519_signed_payload_to_string() {
    let s = SignerKeyEd25519SignedPayload {
        ed25519: Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),
        payload: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ]
        .try_into()
        .unwrap(),
    }
    .to_string();
    assert_eq!(
        s,
        "PA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAQACAQDAQCQMBYIBEFAWDANBYHRAEISCMKBKFQXDAMRUGY4DUPB6IBZGM"
    );
}

#[test]
fn signer_key_ed25519_signed_payload_from_str() {
    let v = SignerKeyEd25519SignedPayload::from_str(
        "PA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAQACAQDAQCQMBYIBEFAWDANBYHRAEISCMKBKFQXDAMRUGY4DUPB6IBZGM",
    );
    assert_eq!(
        v,
        Ok(SignerKeyEd25519SignedPayload {
            ed25519: Uint256([
                0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
                0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
                0xfc, 0x7f, 0xe8, 0x9a,
            ]),
            payload: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20
            ]
            .try_into()
            .unwrap(),
        }),
    );
}

#[test]
fn signer_key_to_string_with_ed25519() {
    let s = SignerKey::Ed25519(Uint256([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))
    .to_string();
    assert_eq!(
        s,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn signer_key_from_str_with_ed25519() {
    let v = SignerKey::from_str("GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ");
    assert_eq!(
        v,
        Ok(SignerKey::Ed25519(Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),)),
    );
}

#[test]
fn signer_key_to_string_with_pre_auth_tx() {
    let s = SignerKey::PreAuthTx(Uint256([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))
    .to_string();
    assert_eq!(
        s,
        "TA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUPUI"
    );
}

#[test]
fn signer_key_from_str_with_pre_auth_tx() {
    let v = SignerKey::from_str("TA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUPUI");
    assert_eq!(
        v,
        Ok(SignerKey::PreAuthTx(Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),)),
    );
}

#[test]
fn signer_key_to_string_with_hash_x() {
    let s = SignerKey::HashX(Uint256([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))
    .to_string();
    assert_eq!(
        s,
        "XA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVLRR"
    );
}

#[test]
fn signer_key_from_str_with_hash_x() {
    let v = SignerKey::from_str("XA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVLRR");
    assert_eq!(
        v,
        Ok(SignerKey::HashX(Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),)),
    );
}

#[test]
fn signer_key_to_string_with_signed_payload_ed25519() {
    let s = SignerKey::Ed25519SignedPayload(SignerKeyEd25519SignedPayload {
        ed25519: Uint256([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]),
        payload: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ]
        .try_into()
        .unwrap(),
    })
    .to_string();
    assert_eq!(
        s,
        "PA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAQACAQDAQCQMBYIBEFAWDANBYHRAEISCMKBKFQXDAMRUGY4DUPB6IBZGM"
    );
}

#[test]
fn signer_key_from_str_with_signed_payload_ed25519() {
    let v = SignerKey::from_str(
        "PA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAQACAQDAQCQMBYIBEFAWDANBYHRAEISCMKBKFQXDAMRUGY4DUPB6IBZGM",
    );
    assert_eq!(
        v,
        Ok(SignerKey::Ed25519SignedPayload(
            SignerKeyEd25519SignedPayload {
                ed25519: Uint256([
                    0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90,
                    0xf7, 0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d,
                    0x7a, 0x03, 0xfc, 0x7f, 0xe8, 0x9a,
                ]),
                payload: [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
                    0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
                ]
                .try_into()
                .unwrap(),
            }
        )),
    );
}

#[test]
fn sc_address_to_string_with_account() {
    let s = ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))))
    .to_string();
    assert_eq!(
        s,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn sc_address_from_str_with_account() {
    let v = ScAddress::from_str("GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ");
    assert_eq!(
        v,
        Ok(ScAddress::Account(AccountId(
            PublicKey::PublicKeyTypeEd25519(Uint256([
                0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
                0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
                0xfc, 0x7f, 0xe8, 0x9a,
            ]))
        ))),
    );
}

#[test]
fn sc_address_to_string_with_contract() {
    let s = ScAddress::Contract(Hash([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ]))
    .to_string();
    assert_eq!(
        s,
        "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA"
    );
}

#[test]
fn sc_address_from_str_with_contract() {
    let v = ScAddress::from_str("CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA");
    assert_eq!(
        v,
        Ok(ScAddress::Contract(Hash([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]))),
    );
}

#[test]
fn sc_address_from_str_with_invalid() {
    let v = ScAddress::from_str(
        "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK",
    );
    assert_eq!(v, Err(Error::Invalid));
}

#[test]
fn asset_code_4_from_str() {
    assert_eq!(AssetCode4::from_str(""), Ok(AssetCode4(*b"\0\0\0\0")));
    assert_eq!(AssetCode4::from_str("a"), Ok(AssetCode4(*b"a\0\0\0")));
    assert_eq!(AssetCode4::from_str("ab"), Ok(AssetCode4(*b"ab\0\0")));
    assert_eq!(AssetCode4::from_str("abc"), Ok(AssetCode4(*b"abc\0")));
    assert_eq!(AssetCode4::from_str("abcd"), Ok(AssetCode4(*b"abcd")));

    assert_eq!(AssetCode4::from_str("abcde"), Err(Error::Invalid));
}

#[test]
fn asset_code_4_to_string() {
    assert_eq!(AssetCode4(*b"\0\0\0\0").to_string(), "");
    assert_eq!(AssetCode4(*b"a\0\0\0").to_string(), "a");
    assert_eq!(AssetCode4(*b"ab\0\0").to_string(), "ab");
    assert_eq!(AssetCode4(*b"abc\0").to_string(), "abc");
    assert_eq!(AssetCode4(*b"abcd").to_string(), "abcd");

    // Preserve as much of the code as possible, even if it contains nul bytes.
    assert_eq!(AssetCode4(*b"a\0cd").to_string(), r"a\0cd");

    // Replace bytes that are not valid utf8 with the replacement character � and preserve length.
    assert_eq!(AssetCode4(*b"a\xc3\x28d").to_string(), r"a\xc3(d");
    assert_eq!(AssetCode4(*b"a\xc3\xc3\x28").to_string(), r"a\xc3\xc3(");
    assert_eq!(AssetCode4(*b"a\xc3\xc3\xc3").to_string(), r"a\xc3\xc3\xc3");
}

#[test]
#[rustfmt::skip]
fn asset_code_12_from_str() {
    assert_eq!(AssetCode12::from_str(""), Ok(AssetCode12(*b"\0\0\0\0\0\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("a"), Ok(AssetCode12(*b"a\0\0\0\0\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("ab"), Ok(AssetCode12(*b"ab\0\0\0\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abc"), Ok(AssetCode12(*b"abc\0\0\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcd"), Ok(AssetCode12(*b"abcd\0\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcde"), Ok(AssetCode12(*b"abcde\0\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcdef"), Ok(AssetCode12(*b"abcdef\0\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcdefg"), Ok(AssetCode12(*b"abcdefg\0\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcdefgh"), Ok(AssetCode12(*b"abcdefgh\0\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcdefghi"), Ok(AssetCode12(*b"abcdefghi\0\0\0")));
    assert_eq!(AssetCode12::from_str("abcdefghij"), Ok(AssetCode12(*b"abcdefghij\0\0")));
    assert_eq!(AssetCode12::from_str("abcdefghijk"), Ok(AssetCode12(*b"abcdefghijk\0")));
    assert_eq!(AssetCode12::from_str("abcdefghijkl"), Ok(AssetCode12(*b"abcdefghijkl")));

    assert_eq!(AssetCode12::from_str("abcdefghijklm"), Err(Error::Invalid));
}

#[test]
#[rustfmt::skip]
fn asset_code_12_to_string() {
    assert_eq!(AssetCode12(*b"\0\0\0\0\0\0\0\0\0\0\0\0").to_string(), r"\0\0\0\0\0");
    assert_eq!(AssetCode12(*b"a\0\0\0\0\0\0\0\0\0\0\0").to_string(), r"a\0\0\0\0");
    assert_eq!(AssetCode12(*b"ab\0\0\0\0\0\0\0\0\0\0").to_string(), r"ab\0\0\0");
    assert_eq!(AssetCode12(*b"abc\0\0\0\0\0\0\0\0\0").to_string(), r"abc\0\0");
    assert_eq!(AssetCode12(*b"abcd\0\0\0\0\0\0\0\0").to_string(), r"abcd\0");
    assert_eq!(AssetCode12(*b"abcde\0\0\0\0\0\0\0").to_string(), "abcde");
    assert_eq!(AssetCode12(*b"abcdef\0\0\0\0\0\0").to_string(), "abcdef");
    assert_eq!(AssetCode12(*b"abcdefg\0\0\0\0\0").to_string(), "abcdefg");
    assert_eq!(AssetCode12(*b"abcdefgh\0\0\0\0").to_string(), "abcdefgh");
    assert_eq!(AssetCode12(*b"abcdefghi\0\0\0").to_string(), "abcdefghi");
    assert_eq!(AssetCode12(*b"abcdefghij\0\0").to_string(), "abcdefghij");
    assert_eq!(AssetCode12(*b"abcdefghijk\0").to_string(), "abcdefghijk");
    assert_eq!(AssetCode12(*b"abcdefghijkl").to_string(), "abcdefghijkl");

    // Preserve as much of the code as possible, even if it contains nul bytes.
    assert_eq!(AssetCode12(*b"a\0cd\0\0\0\0\0\0\0\0").to_string(), r"a\0cd\0");

    // Replace bytes that are not valid utf8 with the replacement character � and preserve length.
    assert_eq!(AssetCode12(*b"a\xc3\x28d\0\0\0\0\0\0\0\0").to_string(), r"a\xc3(d\0");
    assert_eq!(AssetCode12(*b"a\xc3\xc3\x28d\0\0\0\0\0\0\0").to_string(), r"a\xc3\xc3(d");
}

#[test]
#[rustfmt::skip]
fn asset_code_from_str() {
    assert_eq!(AssetCode::from_str(""), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"\0\0\0\0"))));
    assert_eq!(AssetCode::from_str("a"), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"a\0\0\0"))));
    assert_eq!(AssetCode::from_str("ab"), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"ab\0\0"))));
    assert_eq!(AssetCode::from_str("abc"), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"abc\0"))));
    assert_eq!(AssetCode::from_str("abcd"), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"abcd"))));

    assert_eq!(AssetCode::from_str("abcde"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcde\0\0\0\0\0\0\0"))));
    assert_eq!(AssetCode::from_str("abcdef"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdef\0\0\0\0\0\0"))));
    assert_eq!(AssetCode::from_str("abcdefg"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefg\0\0\0\0\0"))));
    assert_eq!(AssetCode::from_str("abcdefgh"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefgh\0\0\0\0"))));
    assert_eq!(AssetCode::from_str("abcdefghi"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghi\0\0\0"))));
    assert_eq!(AssetCode::from_str("abcdefghij"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghij\0\0"))));
    assert_eq!(AssetCode::from_str("abcdefghijk"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghijk\0"))));
    assert_eq!(AssetCode::from_str("abcdefghijkl"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghijkl"))));

    assert_eq!(AssetCode::from_str("abcdefghijklm"), Err(Error::Invalid));
}

#[test]
#[rustfmt::skip]
fn asset_code_to_string() {
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"\0\0\0\0")).to_string(), "");
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"a\0\0\0")).to_string(), "a");
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"ab\0\0")).to_string(), "ab");
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"abc\0")).to_string(), "abc");
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"abcd")).to_string(), "abcd");

    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"\0\0\0\0\0\0\0\0\0\0\0\0")).to_string(), r"\0\0\0\0\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"a\0\0\0\0\0\0\0\0\0\0\0")).to_string(), r"a\0\0\0\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"ab\0\0\0\0\0\0\0\0\0\0")).to_string(), r"ab\0\0\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abc\0\0\0\0\0\0\0\0\0")).to_string(), r"abc\0\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcd\0\0\0\0\0\0\0\0")).to_string(), r"abcd\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcde\0\0\0\0\0\0\0")).to_string(), "abcde");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdef\0\0\0\0\0\0")).to_string(), "abcdef");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefg\0\0\0\0\0")).to_string(), "abcdefg");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefgh\0\0\0\0")).to_string(), "abcdefgh");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghi\0\0\0")).to_string(), "abcdefghi");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghij\0\0")).to_string(), "abcdefghij");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghijk\0")).to_string(), "abcdefghijk");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"abcdefghijkl")).to_string(), "abcdefghijkl");

    // Preserve as much of the code as possible, even if it contains nul bytes.
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"a\0cd")).to_string(), r"a\0cd");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"a\0cd\0\0\0\0\0\0\0\0")).to_string(), r"a\0cd\0");

    // Replace bytes that are not valid utf8 with the replacement character � and preserve length.
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"a\xc3\x28d")).to_string(), r"a\xc3(d");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"a\xc3\x28d\0\0\0\0\0\0\0\0")).to_string(), r"a\xc3(d\0");
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"a\xc3\xc3\x28d\0\0\0\0\0\0\0")).to_string(), r"a\xc3\xc3(d");
}

#[test]
#[rustfmt::skip]
fn asset_code_from_str_to_string_roundtrip_unicode() {
    // Round tripped to correct variant based on byte length, not code point length.
    assert_eq!(AssetCode::CreditAlphanum12(AssetCode12(*b"a\xd9\xaa\xd9\xaa\0\0\0\0\0\0\0")).to_string(), r"a\xd9\xaa\xd9\xaa");
    assert_eq!(AssetCode::from_str(r"a\xd9\xaa\xd9\xaa"), Ok(AssetCode::CreditAlphanum12(AssetCode12(*b"a\xd9\xaa\xd9\xaa\0\0\0\0\0\0\0"))));

    // Round tripped to correct variant based on byte length even when utf8
    // parsing error occurs. To preserve type consistency when round tripping
    // the data, the length when parsing errors occur must be consistent with
    // the input length, which is why a nul byte is expected instead of a
    // Unicode Replacement Character, which would be two bytes.
    assert_eq!(AssetCode::CreditAlphanum4(AssetCode4(*b"a\xc3\xc3d")).to_string(), r"a\xc3\xc3d");
    assert_eq!(AssetCode::from_str(r"a\xc3\xc3d"), Ok(AssetCode::CreditAlphanum4(AssetCode4(*b"a\xc3\xc3d"))));
}

#[test]
#[rustfmt::skip]
fn claimable_balance_id() {
    assert_eq!(
        ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([1u8; 32])).to_string(),
        "000000000101010101010101010101010101010101010101010101010101010101010101",
    );
    // Valid
    assert_eq!(ClaimableBalanceId::from_str("000000000101010101010101010101010101010101010101010101010101010101010101"), Ok(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([1u8; 32]))));
    // Half byte short.
    assert_eq!(ClaimableBalanceId::from_str("00000000010101010101010101010101010101010101010101010101010101010101010"), Err(Error::InvalidHex));
    // Full byte short.
    assert_eq!(ClaimableBalanceId::from_str("0000000001010101010101010101010101010101010101010101010101010101010101"), Err(Error::LengthMismatch));
    // Half byte too long.
    assert_eq!(ClaimableBalanceId::from_str("0000000001010101010101010101010101010101010101010101010101010101010101011"), Err(Error::InvalidHex));
    // Full byte too long.
    assert_eq!(ClaimableBalanceId::from_str("00000000010101010101010101010101010101010101010101010101010101010101010101"), Err(Error::LengthMismatch));
    // Unrecognized discriminant value.
    assert_eq!(ClaimableBalanceId::from_str("000000010101010101010101010101010101010101010101010101010101010101010101"), Err(Error::Invalid));
}
