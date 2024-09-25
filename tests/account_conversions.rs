#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

use ::stellar_xdr::curr::Uint256;
#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{AccountId, MuxedAccount, PublicKey};

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
