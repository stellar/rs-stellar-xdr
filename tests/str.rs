#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

use std::str::FromStr;

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{PublicKey, Uint256};

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
