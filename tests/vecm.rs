#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{BytesM, Limits, ReadXdr, ScVal};

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
    let result = ScVal::from_xdr(data, Limits::none());
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
    let result = ScVal::from_xdr(data, Limits::none());
    assert!(result.is_err());
}

#[test]
fn valid_bytes_len() {
    let data = [
        0x00, 0x00, 0x00, 0x08, // length
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];
    let result: Result<BytesM, _> = BytesM::from_xdr(data, Limits::none());
    assert_eq!(
        result,
        Ok(BytesM::try_from([1, 2, 3, 4, 5, 6, 7, 8]).unwrap())
    );
}

#[test]
fn valid_bytes_len_greater_than_preallocated_bytes_limit() {
    let mut data = [0x01u8; 3000];
    data[0] = 0x00; // length
    data[1] = 0x00; // length
    data[2] = 0x0B; // length
    data[3] = 0xB4; // length
    let result: Result<BytesM, _> = BytesM::from_xdr(data, Limits::none());
    assert_eq!(result, Ok(BytesM::try_from([0x01u8; 2996]).unwrap()));
}

#[test]
fn invalid_bytes_len() {
    let data = [
        0xFF, 0xFF, 0xFF, 0xFF, // length
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];
    let result: Result<BytesM, _> = BytesM::from_xdr(data, Limits::none());
    assert!(result.is_err());
}
