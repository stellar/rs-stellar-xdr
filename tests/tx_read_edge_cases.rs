#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "base64"))]

use std::io::{self, Cursor};
use stellar_xdr::Error;
use stellar_xdr::{DepthLimitedRead, ReadXdr, WriteXdr, DEFAULT_MAX_DEPTH_LIMIT};

#[test]
fn test_read_interrupts_and_residuals() -> Result<(), Error> {
    let v_bytes = [1u32.to_xdr()?, 2u32.to_xdr()?].concat();

    // read_xdr should support not consuming the buffer on the read, being able
    // to do subsequent reads, and continuing on interrupts.
    {
        let mut cursor = DepthLimitedRead::new(
            Interrupted::new(Cursor::new(&v_bytes)),
            DEFAULT_MAX_DEPTH_LIMIT,
        );
        assert_eq!(u32::read_xdr(&mut cursor), Ok(1u32));
        assert_eq!(u32::read_xdr(&mut cursor), Ok(2u32));
    }

    // read_xdr_to_end should require that the buffer completely fill into the
    // type being read, and continue on interrupts.
    {
        assert_eq!(
            u32::read_xdr_to_end(&mut DepthLimitedRead::new(
                Interrupted::new(Cursor::new(&v_bytes)),
                DEFAULT_MAX_DEPTH_LIMIT,
            )),
            Err(Error::Invalid)
        );
        assert_eq!(
            u64::read_xdr_to_end(&mut DepthLimitedRead::new(
                Interrupted::new(Cursor::new(&v_bytes)),
                DEFAULT_MAX_DEPTH_LIMIT,
            )),
            Ok(1u64 << 32 | 2u64)
        );
    }

    // from_xdr should require that the buffer completely fill into the type
    // being read.
    {
        assert_eq!(u32::from_xdr(&v_bytes), Err(Error::Invalid));
        assert_eq!(u64::from_xdr(&v_bytes), Ok(1u64 << 32 | 2u64));
    }

    // from_xdr_base64 should require that the buffer completely fill into the type
    // being read.
    {
        let v_base64 = base64::encode(v_bytes);
        assert_eq!(u32::from_xdr_base64(&v_base64), Err(Error::Invalid));
        assert_eq!(u64::from_xdr_base64(&v_base64), Ok(1u64 << 32 | 2u64));
    }

    Ok(())
}

struct Interrupted<R: io::Read> {
    next_read_interrupts: bool,
    r: R,
}

impl<R: io::Read> Interrupted<R> {
    pub fn new(r: R) -> Self {
        Self {
            next_read_interrupts: true,
            r,
        }
    }
}

impl<R: io::Read> io::Read for Interrupted<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.next_read_interrupts {
            self.next_read_interrupts = false;
            Err(io::Error::new(io::ErrorKind::Interrupted, "interrupted"))
        } else {
            self.r.read(buf)
        }
    }
}
