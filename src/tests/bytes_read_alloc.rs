#![cfg(feature = "std")]

use crate::{BytesM, Error, Limited, Limits, ReadXdr, StringM, VecM, WriteXdr};
use std::io::{self, Cursor, Read};

/// A reader that records the largest buffer it was asked to fill. Reads into a
/// `Vec` are made into its spare capacity, so this bounds how much the decoder
/// had allocated ahead of the bytes it actually received.
struct MaxBufReader<R> {
    inner: R,
    max_buf: usize,
}

impl<R: Read> Read for MaxBufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.max_buf = self.max_buf.max(buf.len());
        self.inner.read(buf)
    }
}

fn huge_len_input() -> Vec<u8> {
    // Declares a length of 0xFFFFFFF0 bytes, but only a few bytes follow.
    [0xFF, 0xFF, 0xFF, 0xF0, 0x01, 0x02, 0x03, 0x04].to_vec()
}

fn assert_huge_len_does_not_allocate<T: ReadXdr + std::fmt::Debug>() {
    let mut r = Limited::new(
        MaxBufReader {
            inner: Cursor::new(huge_len_input()),
            max_buf: 0,
        },
        Limits::none(),
    );
    let result = T::read_xdr(&mut r);
    assert_eq!(
        result.unwrap_err(),
        Error::Io(io::ErrorKind::UnexpectedEof.into())
    );
    assert!(
        r.inner.max_buf <= 1024,
        "reader was given a {} byte buffer for a 4 byte body",
        r.inner.max_buf
    );
}

#[test]
fn huge_len_does_not_allocate_declared_len() {
    assert_huge_len_does_not_allocate::<VecM<u8>>();
    assert_huge_len_does_not_allocate::<BytesM>();
    assert_huge_len_does_not_allocate::<StringM>();
}

fn assert_roundtrip<T>(make: impl Fn(Vec<u8>) -> T, as_vec: impl Fn(&T) -> &Vec<u8>)
where
    T: ReadXdr + WriteXdr + PartialEq + std::fmt::Debug,
{
    for len in [
        0usize, 1, 2, 3, 4, 5, 7, 8, 9, 31, 32, 33, 1000, 4096, 70_000,
    ] {
        let bytes: Vec<u8> = (0..len).map(|i| u8::try_from(i % 251).unwrap()).collect();
        let v = make(bytes.clone());
        let xdr = v.to_xdr(Limits::none()).unwrap();
        let decoded = T::from_xdr(&xdr, Limits::none()).unwrap();
        assert_eq!(decoded, v);
        assert_eq!(as_vec(&decoded), &bytes);
        assert_eq!(
            as_vec(&decoded).capacity(),
            len,
            "len {len}: capacity should match the decoded length"
        );

        // A body shorter than the declared length fails.
        if len > 0 {
            let short = &xdr[..4 + len - 1];
            assert_eq!(
                T::from_xdr(short, Limits::none()),
                Err(Error::Io(io::ErrorKind::UnexpectedEof.into()))
            );
        }

        // Non-zero padding fails.
        if len % 4 != 0 {
            let mut bad_pad = xdr.clone();
            *bad_pad.last_mut().unwrap() = 1;
            assert_eq!(
                T::from_xdr(&bad_pad, Limits::none()),
                Err(Error::NonZeroPadding)
            );
        }
    }
}

#[test]
fn roundtrip_vecm_u8() {
    assert_roundtrip(|b| VecM::<u8>::try_from(b).unwrap(), |v| v.as_vec());
}

#[test]
fn roundtrip_bytesm() {
    assert_roundtrip(
        |b| BytesM::<{ u32::MAX }>::try_from(b).unwrap(),
        |v| v.as_vec(),
    );
}

#[test]
fn roundtrip_stringm() {
    assert_roundtrip(
        |b| StringM::<{ u32::MAX }>::try_from(b).unwrap(),
        |v| v.as_vec(),
    );
}
