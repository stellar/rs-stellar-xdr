#![cfg(feature = "std")]

use crate::{Error, Limits, ReadXdr, ScVal};

#[test]
fn bool_valid() {
    assert_eq!(bool::from_xdr([0, 0, 0, 0], Limits::none()), Ok(false));
    assert_eq!(bool::from_xdr([0, 0, 0, 1], Limits::none()), Ok(true));
}

#[test]
fn bool_invalid() {
    assert_eq!(
        bool::from_xdr([0, 0, 0, 2], Limits::none()),
        Err(Error::Invalid)
    );
    assert_eq!(
        bool::from_xdr([0xff, 0xff, 0xff, 0xff], Limits::none()),
        Err(Error::Invalid)
    );
}

#[test]
fn scval_bool_invalid() {
    let data = [
        0x00, 0x00, 0x00, 0x00, // SCV_BOOL
        0x00, 0x00, 0x00, 0x07, // not 0 or 1
    ];
    assert_eq!(ScVal::from_xdr(data, Limits::none()), Err(Error::Invalid));
}
