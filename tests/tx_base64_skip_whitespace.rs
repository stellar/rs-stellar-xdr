#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "base64"))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use base64::Engine;
use std::assert_eq;
use std::io::Cursor;
use stellar_xdr::Error;
use stellar_xdr::{Limited, Limits, ReadXdr, WriteXdr};

#[test]
fn test_skip_whitespace() -> Result<(), Error> {
    let v_bytes = [1u32.to_xdr(Limits::none())?, 2u32.to_xdr(Limits::none())?].concat();

    let v_base64 = base64::engine::general_purpose::STANDARD.encode(&v_bytes);
    assert_eq!(v_base64.len(), 12);
    assert_eq!(v_base64, "AAAAAQAAAAI=");

    let v_base64 = " AA\nAAAQ A  AAAI= ";

    {
        assert_eq!(
            u32::read_xdr_base64_to_end(&mut Limited::new(Cursor::new(&v_base64), Limits::none())),
            Err(Error::Invalid)
        );
        assert_eq!(
            u64::read_xdr_base64_to_end(&mut Limited::new(Cursor::new(&v_base64), Limits::none())),
            Ok((1u64 << 32) | 2u64)
        );
    }

    {
        assert_eq!(
            u32::from_xdr_base64(&v_base64, Limits::none()),
            Err(Error::Invalid)
        );
        assert_eq!(
            u64::from_xdr_base64(&v_base64, Limits::none()),
            Ok((1u64 << 32) | 2u64)
        );
    }

    {
        let mut r = Limited::new(Cursor::new(&v_base64), Limits::none());
        let mut iter = u32::read_xdr_base64_iter(&mut r);
        assert_eq!(iter.next(), Some(Ok(1)),);
        assert_eq!(iter.next(), Some(Ok(2)),);
        assert_eq!(iter.next(), None,);
    }

    Ok(())
}
