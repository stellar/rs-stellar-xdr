#![cfg(all(feature = "std", feature = "base64"))]

use base64::Engine;
use std::assert_eq;
use std::io::Cursor;
use stellar_xdr::Error;
use stellar_xdr::{Limited, Limits, ReadXdr, WriteXdr};

#[test]
fn test_skip_whitespace_long_run() -> Result<(), Error> {
    // A long contiguous whitespace run (2048 bytes here, large enough to fill
    // at least one of the base64 decoder's internal read buffers) must not be
    // treated as end-of-input. Before the fix, SkipWhitespace::read returned
    // Ok(0) when a whole delegate read was whitespace, which the decoder
    // interprets as EOF -> silent truncation.
    let v_bytes = [1u32.to_xdr(Limits::none())?, 2u32.to_xdr(Limits::none())?].concat();
    let core = base64::engine::general_purpose::STANDARD.encode(&v_bytes);
    assert_eq!(core, "AAAAAQAAAAI=");
    let ws = " ".repeat(2048);

    // Leading long whitespace run.
    let leading = format!("{ws}{core}");
    assert_eq!(
        u64::from_xdr_base64(&leading, Limits::none()),
        Ok((1u64 << 32) | 2u64)
    );

    // Interior long whitespace run (split the base64 mid-string).
    let (a, b) = core.split_at(4);
    let interior = format!("{a}{ws}{b}");
    assert_eq!(
        u64::from_xdr_base64(&interior, Limits::none()),
        Ok((1u64 << 32) | 2u64)
    );

    Ok(())
}

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
            u32::from_xdr_base64(v_base64, Limits::none()),
            Err(Error::Invalid)
        );
        assert_eq!(
            u64::from_xdr_base64(v_base64, Limits::none()),
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
