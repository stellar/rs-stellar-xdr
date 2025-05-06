#![cfg(feature = "curr")]
#![cfg(feature = "std")]

use stellar_xdr::curr as stellar_xdr;

use stellar_xdr::{
    AccountId, AssetCode, AssetCode12, AssetCode4, ClaimableBalanceId, ContractId, Error, Hash,
    Int128Parts, Int256Parts, MuxedAccount, MuxedAccountMed25519, MuxedEd25519Account, NodeId,
    PoolId, PublicKey, ScAddress, SignerKey, SignerKeyEd25519SignedPayload, UInt128Parts,
    UInt256Parts, Uint256,
};

use std::{fmt::Debug, str::FromStr};

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
fn contract_id_from_str() {
    let v = ContractId::from_str("CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA");
    assert_eq!(
        v,
        Ok(ContractId(Hash([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]))),
    );
}

#[test]
fn contract_id_to_string() {
    let s = ContractId(Hash([
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
    let s = ScAddress::Contract(ContractId(Hash([
        0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7, 0x05,
        0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03, 0xfc, 0x7f,
        0xe8, 0x9a,
    ])))
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
        Ok(ScAddress::Contract(ContractId(Hash([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ])))),
    );
}

#[test]
fn sc_address_to_string_with_muxed_account() {
    let s = ScAddress::MuxedAccount(MuxedEd25519Account {
        id: 123_456,
        ed25519: Uint256([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ]),
    })
    .to_string();
    assert_eq!(
        s,
        "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU"
    );
}

#[test]
fn sc_address_from_str_with_muxed_account() {
    let v = ScAddress::from_str(
        "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU",
    );
    assert_eq!(
        v,
        Ok(ScAddress::MuxedAccount(MuxedEd25519Account {
            id: 123_456,
            ed25519: Uint256([
                0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
                0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
                0x05, 0xc7, 0xb1, 0x03,
            ]),
        })),
    );
}

#[test]
fn sc_address_to_string_with_liquidity_pool() {
    let s = ScAddress::LiquidityPool(PoolId(Hash([
        0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79, 0xe4,
        0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d, 0x05, 0xc7,
        0xb1, 0x03,
    ])))
    .to_string();
    assert_eq!(
        s,
        "LA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGZ5J"
    );
}

#[test]
fn sc_address_from_str_with_liquidity_pool() {
    let v = ScAddress::from_str("LA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUPJN");
    assert_eq!(
        v,
        Ok(ScAddress::LiquidityPool(PoolId(Hash([
            0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
            0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
            0xfc, 0x7f, 0xe8, 0x9a,
        ]))))
    );
}

#[test]
fn sc_address_to_string_with_claimable_balance() {
    let s = ScAddress::ClaimableBalance(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([
        0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79, 0xe4,
        0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d, 0x05, 0xc7,
        0xb1, 0x03,
    ])))
    .to_string();
    assert_eq!(
        s,
        "BAADMPVKHBTYIH522D2O3CGHPHSP4ZXFNISHBXEYYDWJYBZ5AXD3CA3GDE"
    );
}

#[test]
fn sc_address_from_str_with_claimable_balance() {
    let v = ScAddress::from_str("BAAD6DBUX6J22DMZOHIEZTEQ64CVCHEDRKWZONFEUL5Q26QD7R76RGR4TU");
    assert_eq!(
        v,
        Ok(ScAddress::ClaimableBalance(
            ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([
                0x3f, 0x0c, 0x34, 0xbf, 0x93, 0xad, 0x0d, 0x99, 0x71, 0xd0, 0x4c, 0xcc, 0x90, 0xf7,
                0x05, 0x51, 0x1c, 0x83, 0x8a, 0xad, 0x97, 0x34, 0xa4, 0xa2, 0xfb, 0x0d, 0x7a, 0x03,
                0xfc, 0x7f, 0xe8, 0x9a,
            ]))
        ))
    );
}

#[test]
fn muxed_ed25519_account_to_string_with_muxed_account() {
    let s = MuxedEd25519Account {
        id: 123_456,
        ed25519: Uint256([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ]),
    }
    .to_string();
    assert_eq!(
        s,
        "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU"
    );
}

#[test]
fn muxed_ed25519_account_from_str_with_muxed_account() {
    let v = MuxedEd25519Account::from_str(
        "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU",
    );
    assert_eq!(
        v,
        Ok(MuxedEd25519Account {
            id: 123_456,
            ed25519: Uint256([
                0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
                0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
                0x05, 0xc7, 0xb1, 0x03,
            ])
        }),
    );
}

#[test]
fn sc_address_from_str_with_invalid() {
    let v = ScAddress::from_str("XBU2RRGLXH3E5CQHTD3ODLDF2BWDCYUSSBLLZ5GNW7JXHDIYKXZWGTOG");
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
fn claimable_balance_id() {
    // To string
    assert_eq!(
        ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ]))
        .to_string(),
        "BAADMPVKHBTYIH522D2O3CGHPHSP4ZXFNISHBXEYYDWJYBZ5AXD3CA3GDE",
    );
    // From string - valid
    assert_eq!(
        ClaimableBalanceId::from_str("BAADMPVKHBTYIH522D2O3CGHPHSP4ZXFNISHBXEYYDWJYBZ5AXD3CA3GDE"),
        Ok(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ])))
    );
    // From string - invalid
    assert_eq!(
        ClaimableBalanceId::from_str("BAADMPVKHBTYIH522D2O3CGHPHSP4ZXFNISHBXEYYDWJYBZ5AXD3CA3GDEA"),
        Err(Error::Invalid)
    );
}

#[test]
fn liquidity_pool_id() {
    // To string
    assert_eq!(
        PoolId(Hash([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ]))
        .to_string(),
        "LA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGZ5J",
    );
    // From string - valid
    assert_eq!(
        PoolId::from_str("LA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGZ5J"),
        Ok(PoolId(Hash([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ])))
    );
    // From string - invalid
    assert_eq!(
        ClaimableBalanceId::from_str("LA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAGPZA"),
        Err(Error::Invalid)
    );
}

#[test]
#[rustfmt::skip]
fn i128() {
    assert_roundtrip(Int128Parts { hi: 0, lo: 0 } .to_string(), "0");
    assert_roundtrip(Int128Parts { hi: 0, lo: 1 } .to_string(), "1");
    assert_roundtrip(Int128Parts { hi: -1,lo: 0xffffffffffffffff } .to_string(), "-1");
    assert_roundtrip(Int128Parts { hi: 0x7fffffffffffffff, lo: 0xffffffffffffffff } .to_string(), "170141183460469231731687303715884105727");
    assert_roundtrip(Int128Parts { hi: -0x8000000000000000,lo: 0x0000000000000000 } .to_string(), "-170141183460469231731687303715884105728");
    assert_eq!(Int128Parts::from_str("x"), Err(Error::Invalid));
}

#[test]
#[rustfmt::skip]
fn u128() {
    assert_roundtrip(UInt128Parts { hi: 0, lo: 0 } .to_string(), "0");
    assert_roundtrip(UInt128Parts { hi: 0, lo: 1 } .to_string(), "1");
    assert_roundtrip(UInt128Parts { hi: 0xffffffffffffffff, lo: 0xffffffffffffffff } .to_string(), "340282366920938463463374607431768211455");
    assert_eq!(UInt128Parts::from_str("x"), Err(Error::Invalid));
}

#[test]
#[rustfmt::skip]
fn i256() {
    assert_roundtrip(Int256Parts { hi_hi: 0, hi_lo: 0, lo_hi: 0, lo_lo: 0 } .to_string(), "0");
    assert_roundtrip(Int256Parts { hi_hi: 0, hi_lo: 0, lo_hi: 0, lo_lo: 1 } .to_string(), "1");
    assert_roundtrip(Int256Parts { hi_hi: -1, hi_lo: 0xffffffffffffffff, lo_hi: 0xffffffffffffffff, lo_lo: 0xffffffffffffffff } .to_string(), "-1");
    assert_roundtrip(Int256Parts { hi_hi: 0x7fffffffffffffff, hi_lo: 0xffffffffffffffff, lo_hi: 0xffffffffffffffff, lo_lo: 0xffffffffffffffff } .to_string(), "57896044618658097711785492504343953926634992332820282019728792003956564819967");
    assert_roundtrip(Int256Parts { hi_hi: -0x8000000000000000, hi_lo: 0x0000000000000000, lo_hi: 0x0000000000000000, lo_lo: 0x0000000000000000 } .to_string(), "-57896044618658097711785492504343953926634992332820282019728792003956564819968");
    assert_eq!(Int256Parts::from_str("x"), Err(Error::Invalid));
}

#[test]
#[rustfmt::skip]
fn u256() {
    assert_roundtrip(UInt256Parts { hi_hi: 0, hi_lo: 0, lo_hi: 0, lo_lo: 0 } .to_string(), "0");
    assert_roundtrip(UInt256Parts { hi_hi: 0, hi_lo: 0, lo_hi: 0, lo_lo: 1 } .to_string(), "1");
    assert_roundtrip(UInt256Parts { hi_hi: 0xffffffffffffffff, hi_lo: 0xffffffffffffffff, lo_hi: 0xffffffffffffffff, lo_lo: 0xffffffffffffffff } .to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(UInt256Parts::from_str("x"), Err(Error::Invalid));
}

fn assert_roundtrip<V>(v: V, s: &str)
where
    V: FromStr + ToString + PartialEq + Debug,
    V::Err: PartialEq + Debug,
{
    assert_eq!(v.to_string(), s);
    assert_eq!(V::from_str(s), Ok(v));
}
