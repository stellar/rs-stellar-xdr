#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
// #[cfg(feature = "next")]
// use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{Int128Parts, UInt128Parts};

#[test]
fn round_trip_u128() {
    let u128_val = 0x1234567890abcdef1234567890abcdefu128;
    let xdr_val: UInt128Parts = u128_val.into();
    let u128_val2: u128 = xdr_val.into();
    assert_eq!(u128_val, u128_val2);
}

#[test]
fn round_trip_i128() {
    let i128_val = 0x1234567890abcdef1234567890abcdefi128;
    let xdr_val: Int128Parts = i128_val.into();
    let i128_val2: i128 = xdr_val.into();
    assert_eq!(i128_val, i128_val2);
}

#[test]
fn round_trip_u256() {
    let u256_val = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdefu128;
    let xdr_val: UInt128Parts = u256_val.into();
    let u256_val2: u128 = xdr_val.into();
    assert_eq!(u256_val, u256_val2);
}
