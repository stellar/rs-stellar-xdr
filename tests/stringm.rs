#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{Error, StringM};

use std::str::FromStr;

#[test]
fn stringm_from_str_at_max() {
    // Exactly at the limit should succeed.
    let result = StringM::<3>::from_str("abc");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"abc");
}

#[test]
fn stringm_from_str_within_max() {
    // Within the limit should succeed.
    let result = StringM::<3>::from_str("ab");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"ab");
}

#[test]
fn stringm_from_str_exceeding_max() {
    let result = StringM::<3>::from_str("abcd");
    assert_eq!(result, Err(Error::LengthExceedsMax));
}

#[test]
fn stringm_from_str_empty() {
    let result = StringM::<3>::from_str("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_vec(), b"");
}
