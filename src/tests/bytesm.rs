#![cfg(feature = "std")]

use crate::{BytesM, Error, Limits, ReadXdr, SignerKeyEd25519SignedPayload, WriteXdr};

use std::str::FromStr;

#[test]
fn bytesm_try_from_below_min() {
    let result = BytesM::<3, 2>::try_from(vec![1u8]);
    assert_eq!(result, Err(Error::LengthBelowMin));
}

#[test]
fn bytesm_try_from_at_min() {
    let result = BytesM::<3, 2>::try_from(vec![1u8, 2]);
    assert_eq!(result.unwrap().as_vec(), &[1, 2]);
}

#[test]
fn bytesm_try_from_exceeding_max_with_min() {
    let result = BytesM::<3, 2>::try_from(vec![1u8, 2, 3, 4]);
    assert_eq!(result, Err(Error::LengthExceedsMax));
}

#[test]
fn bytesm_from_str_below_min() {
    let result = BytesM::<3, 1>::from_str("");
    assert_eq!(result, Err(Error::LengthBelowMin));
}

#[test]
fn bytesm_default_is_min_len_zeros() {
    assert_eq!(BytesM::<3, 2>::default().as_vec(), &[0, 0]);
    assert!(BytesM::<3>::default().is_empty());
}

#[test]
fn bytesm_read_xdr_below_min() {
    let xdr = BytesM::<3>::default().to_xdr(Limits::none()).unwrap();
    let result = BytesM::<3, 1>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::LengthBelowMin));
}

#[test]
fn signer_key_ed25519_signed_payload_read_xdr_empty_payload() {
    // An ed25519 key of zeros followed by a zero payload length.
    let xdr = [0u8; 36];
    let result = SignerKeyEd25519SignedPayload::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::LengthBelowMin));
}

#[test]
fn signer_key_ed25519_signed_payload_default_displays() {
    let v = SignerKeyEd25519SignedPayload::default();
    assert_eq!(v.payload.as_vec(), &[0]);
    let s = v.to_string();
    assert_eq!(SignerKeyEd25519SignedPayload::from_str(&s), Ok(v));
}

#[cfg(feature = "serde")]
#[test]
fn signer_key_ed25519_signed_payload_json_empty_payload() {
    let result = serde_json::from_str::<SignerKeyEd25519SignedPayload>(
        r#"{"ed25519":"0000000000000000000000000000000000000000000000000000000000000000","payload":""}"#,
    );
    assert!(result.is_err());
}
