#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{AccountId, MuxedAccount, MuxedAccountMed25519, PublicKey, Uint256};

#[test]
fn from_account_id_to_muxed_account() {
    let account_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([1u8; 32])));
    let muxed_account: MuxedAccount = account_id.into();
    assert_eq!(muxed_account, MuxedAccount::Ed25519(Uint256([1u8; 32])));
}

#[test]
fn from_public_key_to_muxed_account() {
    let public_key = PublicKey::PublicKeyTypeEd25519(Uint256([1u8; 32]));
    let muxed_account: MuxedAccount = public_key.into();
    assert_eq!(muxed_account, MuxedAccount::Ed25519(Uint256([1u8; 32])));
}

#[test]
fn from_muxed_account_ed_to_account_id() {
    let muxed_account: MuxedAccount = MuxedAccount::Ed25519(Uint256([1u8; 32]));
    let account_id = muxed_account.account_id();
    assert_eq!(
        account_id,
        AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([1u8; 32])))
    );
}

#[test]
fn from_muxed_account_med_to_account_id() {
    let muxed_account: MuxedAccount = MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
        id: 2,
        ed25519: Uint256([1u8; 32]),
    });
    let account_id = muxed_account.account_id();
    assert_eq!(
        account_id,
        AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([1u8; 32])))
    );
}
