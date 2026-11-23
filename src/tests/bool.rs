#![cfg(feature = "std")]

use crate::{Error, Limits, ReadXdr, ScVal, WriteXdr};

#[test]
fn bool_decodes_zero_as_false() {
    let bytes = [0u8, 0, 0, 0];
    assert_eq!(bool::from_xdr(bytes, Limits::none()), Ok(false));
    assert_eq!(false.to_xdr(Limits::none()), Ok(bytes.to_vec()));
}

#[test]
fn bool_decodes_one_as_true() {
    let bytes = [0u8, 0, 0, 1];
    assert_eq!(bool::from_xdr(bytes, Limits::none()), Ok(true));
    assert_eq!(true.to_xdr(Limits::none()), Ok(bytes.to_vec()));
}

#[test]
fn bool_rejects_two() {
    let res = bool::from_xdr([0u8, 0, 0, 2], Limits::none());
    assert_eq!(res, Err(Error::Invalid));
}

#[test]
fn bool_rejects_max() {
    let res = bool::from_xdr([0xffu8, 0xff, 0xff, 0xff], Limits::none());
    assert_eq!(res, Err(Error::Invalid));
}

#[test]
fn scval_bool_rejects_two() {
    // [0, 0, 0, 0]: ScVal discriminant SCV_BOOL (0)
    // [0, 0, 0, 2]: bool value 2, which is neither false (0) nor true (1)
    let res = ScVal::from_xdr([0u8, 0, 0, 0, 0, 0, 0, 2], Limits::none());
    assert_eq!(res, Err(Error::Invalid));
}

#[test]
fn scval_bool_rejects_max() {
    // [0, 0, 0, 0]: ScVal discriminant SCV_BOOL (0)
    // [0xff, 0xff, 0xff, 0xff]: bool value 0xffffffff, which is neither false (0) nor true (1)
    let res = ScVal::from_xdr([0u8, 0, 0, 0, 0xff, 0xff, 0xff, 0xff], Limits::none());
    assert_eq!(res, Err(Error::Invalid));
}
