#![cfg(all(feature = "std", feature = "base64"))]

use stellar_xdr::Error;

#[test]
fn test_read_distance() -> Result<(), Error> {
    use std::io::Cursor;
    use stellar_xdr::{ReadXdr, WriteXdr};

    let v_bytes = [1u32.to_xdr()?, 2u32.to_xdr()?].concat();

    // read_xdr should support not consuming the buffer on the read, and being
    // able to do subsequent reads.
    {
        let mut cursor = Cursor::new(&v_bytes);
        assert_eq!(u32::read_xdr(&mut cursor), Ok(1u32));
        assert_eq!(u32::read_xdr(&mut cursor), Ok(2u32));
    }

    // read_xdr_to_end should require that the buffer completely fill into the
    // type being read.
    {
        assert_eq!(
            u32::read_xdr_to_end(&mut Cursor::new(&v_bytes)),
            Err(Error::Invalid)
        );
        assert_eq!(
            u64::read_xdr_to_end(&mut Cursor::new(&v_bytes)),
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
