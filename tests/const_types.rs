//! Tests for the borrowing `Const` types.

use stellar_xdr::{BytesMConst, ErrorLengthExceedsMax, StringMConst, VecMConst};

#[test]
fn consts_try_from_enforce_max_len() {
    assert!(VecMConst::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
    assert_eq!(
        VecMConst::<u32, 2>::try_from_slice(&[1, 2, 3]),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesMConst::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesMConst::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringMConst::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringMConst::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    assert!(StringMConst::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        StringMConst::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );
}

#[test]
fn consts_try_from_or_panic() {
    assert_eq!(
        VecMConst::<u32, 3>::try_from_slice_or_panic(&[1, 2, 3]),
        VecMConst::<u32, 3>::try_from_slice(&[1, 2, 3]).unwrap()
    );
    assert_eq!(
        BytesMConst::<3>::try_from_slice_or_panic(b"abc"),
        BytesMConst::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringMConst::<3>::try_from_str_or_panic("abc"),
        StringMConst::<3>::try_from_str("abc").unwrap()
    );
    assert_eq!(
        StringMConst::<3>::try_from_slice_or_panic(b"abc"),
        StringMConst::<3>::try_from_slice(b"abc").unwrap()
    );
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = VecMConst::<u32, 2>::try_from_slice_or_panic(&[1, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = BytesMConst::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = StringMConst::<2>::try_from_str_or_panic("abc");
}

#[test]
fn consts_default() {
    assert_eq!(
        VecMConst::<u32, 3>::default(),
        VecMConst::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        BytesMConst::<3>::default(),
        BytesMConst::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        StringMConst::<3>::default(),
        StringMConst::try_from_str("").unwrap()
    );
}
