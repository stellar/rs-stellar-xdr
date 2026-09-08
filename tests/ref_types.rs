//! Tests for the borrowing `Ref` types.

use stellar_xdr::{BytesMRef, ErrorLengthExceedsMax, StringMRef, VecMRef};

#[cfg(feature = "alloc")]
mod common;
#[cfg(feature = "alloc")]
use common::{tx_env_owned, tx_env_ref};
#[cfg(feature = "alloc")]
use stellar_xdr::{TransactionEnvelope, TransactionEnvelopeRef};

#[cfg(feature = "std")]
use stellar_xdr::{Limits, WriteXdr};
#[test]
fn refs_try_from_enforce_max_len() {
    assert!(VecMRef::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
    assert_eq!(
        VecMRef::<u32, 2>::try_from_slice(&[1, 2, 3]),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesMRef::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesMRef::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringMRef::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringMRef::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    assert!(StringMRef::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        StringMRef::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );
}

#[test]
fn refs_try_from_or_panic() {
    assert_eq!(
        VecMRef::<u32, 3>::try_from_slice_or_panic(&[1, 2, 3]),
        VecMRef::<u32, 3>::try_from_slice(&[1, 2, 3]).unwrap()
    );
    assert_eq!(
        BytesMRef::<3>::try_from_slice_or_panic(b"abc"),
        BytesMRef::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringMRef::<3>::try_from_str_or_panic("abc"),
        StringMRef::<3>::try_from_str("abc").unwrap()
    );
    assert_eq!(
        StringMRef::<3>::try_from_slice_or_panic(b"abc"),
        StringMRef::<3>::try_from_slice(b"abc").unwrap()
    );
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = VecMRef::<u32, 2>::try_from_slice_or_panic(&[1, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = BytesMRef::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = StringMRef::<2>::try_from_str_or_panic("abc");
}

#[test]
fn refs_default() {
    assert_eq!(
        VecMRef::<u32, 3>::default(),
        VecMRef::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        BytesMRef::<3>::default(),
        BytesMRef::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        StringMRef::<3>::default(),
        StringMRef::try_from_str("").unwrap()
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ref_converts_to_owned() {
    // A ref and owned value that are identically defined.
    let r: TransactionEnvelopeRef = const { tx_env_ref() };
    let o: TransactionEnvelope = tx_env_owned();

    assert_eq!(TransactionEnvelope::from(&r), o);
}

#[cfg(feature = "std")]
#[test]
fn ref_and_owned_encode_same() {
    // A ref and owned value that are identically defined.
    let r: TransactionEnvelopeRef = const { tx_env_ref() };
    let o: TransactionEnvelope = tx_env_owned();

    // Ref and owned encode to the same XDR.
    let r_xdr = r.to_xdr(Limits::none()).unwrap();
    let o_xdr = o.to_xdr(Limits::none()).unwrap();
    assert_eq!(r_xdr, o_xdr);
}
