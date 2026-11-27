#![cfg(feature = "std")]

use crate::{BytesM, Error, Limited, Limits, ReadXdr, StringM, VecM};
use std::io::{self, Cursor, Read};

/// A reader that tracks how much memory the decoder allocated ahead of the data
/// it had received.
///
/// The decoder can only ask a reader to fill a buffer it has already
/// allocated, so the largest buffer passed to `read` is a lower bound on how
/// much the decoder allocated before seeing the bytes to put in it. A decoder
/// that allocates the declared length up front passes a buffer of that length.
/// A decoder that grows its buffer as bytes arrive passes only small buffers.
struct MaxBufReader {
    inner: Cursor<Vec<u8>>,
    /// The largest buffer, in bytes, that the decoder has asked to be filled,
    /// i.e. how much it had allocated ahead of the data it received.
    max_buf: usize,
}

impl Read for MaxBufReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.max_buf = self.max_buf.max(buf.len());
        self.inner.read(buf)
    }
}

#[test]
fn vecm_u8_huge_len_with_short_input_errors_without_allocating_len() {
    let mut r = Limited::new(
        MaxBufReader {
            // Declares 0xFFFFFFF0 bytes, but only 4 follow.
            inner: Cursor::new(vec![0xFF, 0xFF, 0xFF, 0xF0, 1, 2, 3, 4]),
            max_buf: 0,
        },
        Limits::none(),
    );
    let result = VecM::<u8>::read_xdr(&mut r);
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
    assert!(r.inner.max_buf <= 1024, "max_buf {}", r.inner.max_buf);
}

#[test]
fn bytesm_huge_len_with_short_input_errors_without_allocating_len() {
    let mut r = Limited::new(
        MaxBufReader {
            // Declares 0xFFFFFFF0 bytes, but only 4 follow.
            inner: Cursor::new(vec![0xFF, 0xFF, 0xFF, 0xF0, 1, 2, 3, 4]),
            max_buf: 0,
        },
        Limits::none(),
    );
    let result = <BytesM>::read_xdr(&mut r);
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
    assert!(r.inner.max_buf <= 1024, "max_buf {}", r.inner.max_buf);
}

#[test]
fn stringm_huge_len_with_short_input_errors_without_allocating_len() {
    let mut r = Limited::new(
        MaxBufReader {
            // Declares 0xFFFFFFF0 bytes, but only 4 follow.
            inner: Cursor::new(vec![0xFF, 0xFF, 0xFF, 0xF0, 1, 2, 3, 4]),
            max_buf: 0,
        },
        Limits::none(),
    );
    let result = <StringM>::read_xdr(&mut r);
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
    assert!(r.inner.max_buf <= 1024, "max_buf {}", r.inner.max_buf);
}

#[test]
fn vecm_u8_decodes_bytes_and_padding() {
    let xdr = [0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0];
    let v = VecM::<u8>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.as_slice(), [1, 2, 3, 4, 5]);
}

#[test]
fn bytesm_decodes_bytes_and_padding() {
    let xdr = [0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 0];
    let v = <BytesM>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.as_slice(), [1, 2, 3, 4, 5]);
}

#[test]
fn stringm_decodes_bytes_and_padding() {
    let xdr = [0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o', 0, 0, 0];
    let v = <StringM>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.as_slice(), b"hello");
}

#[test]
fn vecm_u8_decoded_capacity_equals_len() {
    let mut xdr = 70_000u32.to_be_bytes().to_vec();
    xdr.extend(vec![7u8; 70_000]);
    let v = VecM::<u8>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.len(), 70_000);
    assert_eq!(v.as_vec().capacity(), 70_000);
}

#[test]
fn bytesm_decoded_capacity_equals_len() {
    let mut xdr = 70_000u32.to_be_bytes().to_vec();
    xdr.extend(vec![7u8; 70_000]);
    let v = <BytesM>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.len(), 70_000);
    assert_eq!(v.as_vec().capacity(), 70_000);
}

#[test]
fn stringm_decoded_capacity_equals_len() {
    let mut xdr = 70_000u32.to_be_bytes().to_vec();
    xdr.extend(vec![b'a'; 70_000]);
    let v = <StringM>::from_xdr(xdr, Limits::none()).unwrap();
    assert_eq!(v.len(), 70_000);
    assert_eq!(v.as_vec().capacity(), 70_000);
}

#[test]
fn vecm_u8_short_body_errors() {
    let xdr = [0, 0, 0, 5, 1, 2, 3];
    let result = VecM::<u8>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
}

#[test]
fn bytesm_short_body_errors() {
    let xdr = [0, 0, 0, 5, 1, 2, 3];
    let result = <BytesM>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
}

#[test]
fn stringm_short_body_errors() {
    let xdr = [0, 0, 0, 5, b'h', b'e', b'l'];
    let result = <StringM>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::Io(io::ErrorKind::UnexpectedEof.into())));
}

#[test]
fn vecm_u8_nonzero_padding_errors() {
    let xdr = [0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 1];
    let result = VecM::<u8>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::NonZeroPadding));
}

#[test]
fn bytesm_nonzero_padding_errors() {
    let xdr = [0, 0, 0, 5, 1, 2, 3, 4, 5, 0, 0, 1];
    let result = <BytesM>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::NonZeroPadding));
}

#[test]
fn stringm_nonzero_padding_errors() {
    let xdr = [0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o', 0, 0, 1];
    let result = <StringM>::from_xdr(xdr, Limits::none());
    assert_eq!(result, Err(Error::NonZeroPadding));
}
