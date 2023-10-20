#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{ReadXdr, ScVal};

#[test]
fn valid_len() {
    let data = [
        0x00, 0x00, 0x00, 0x11, // SCV_MAP
        0x00, 0x00, 0x00, 0x01, // Some
        0x00, 0x00, 0x00, 0x01, // map length
        0x00, 0x00, 0x00, 0x0f, // SCV_SYMBOL
        0x00, 0x00, 0x00, 0x03, // symbol length: 3
        0x63, 0x6e, 0x74, 0x00, // symbol value : "cnt"
        0x00, 0x00, 0x00, 0x03, // SCV_U32
        0x00, 0x00, 0x00, 0x2a, // 42
    ];
    let result = ScVal::from_xdr(data);
    assert!(result.is_ok());
}

#[test]
fn invalid_len() {
    let data = [
        0x00, 0x00, 0x00, 0x11, // SCV_MAP
        0x00, 0x00, 0x00, 0x01, // Some
        0xff, 0xff, 0xff, 0xff, // map length (first byte invalid)
        0x00, 0x00, 0x00, 0x0f, // SCV_SYMBOL
        0x00, 0x00, 0x00, 0x03, // symbol length: 3
        0x63, 0x6e, 0x74, 0x00, // symbol value : "cnt"
        0x00, 0x00, 0x00, 0x03, // SCV_U32
        0x00, 0x00, 0x00, 0x2a, // 42
    ];
    let result = ScVal::from_xdr(data);
    assert!(result.is_err());
}
