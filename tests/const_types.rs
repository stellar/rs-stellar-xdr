//! Tests for the `const` module's borrowing types.

#![cfg(feature = "const")]

use stellar_xdr::r#const::{BytesM, StringM, VecM};
use stellar_xdr::ErrorLengthExceedsMax;

#[test]
fn consts_try_from_enforce_max_len() {
    assert!(VecM::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
    assert_eq!(
        VecM::<u32, 2>::try_from_slice(&[1, 2, 3]),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesM::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesM::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringM::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringM::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    assert!(StringM::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        StringM::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );
}

#[test]
fn consts_try_from_or_panic() {
    assert_eq!(
        VecM::<u32, 3>::try_from_slice_or_panic(&[1, 2, 3]),
        VecM::<u32, 3>::try_from_slice(&[1, 2, 3]).unwrap()
    );
    assert_eq!(
        BytesM::<3>::try_from_slice_or_panic(b"abc"),
        BytesM::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringM::<3>::try_from_str_or_panic("abc"),
        StringM::<3>::try_from_str("abc").unwrap()
    );
    assert_eq!(
        StringM::<3>::try_from_slice_or_panic(b"abc"),
        StringM::<3>::try_from_slice(b"abc").unwrap()
    );
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = VecM::<u32, 2>::try_from_slice_or_panic(&[1, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = BytesM::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_const_try_from_or_panic_panics_enforce_max_len() {
    let _ = StringM::<2>::try_from_str_or_panic("abc");
}

#[test]
fn consts_default() {
    assert_eq!(
        VecM::<u32, 3>::default(),
        VecM::try_from_slice(&[]).unwrap()
    );
    assert_eq!(BytesM::<3>::default(), BytesM::try_from_slice(&[]).unwrap());
    assert_eq!(StringM::<3>::default(), StringM::try_from_str("").unwrap());
}
