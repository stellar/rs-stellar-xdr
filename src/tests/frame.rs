#![cfg(feature = "std")]
use crate::{Error, Frame, Limited, Limits, PublicKey, ReadXdr, Uint256, WriteXdr};
use std::io::Cursor;

// Frames wrap XDR messages, and have a four byte header that holds a length and
// the high bit indicates if there are more fragments for the current frame.

#[test]
fn frame_read_last_fragment() {
    let pk = PublicKey::PublicKeyTypeEd25519(Uint256([1; 32]));
    let msg = pk.to_xdr(Limits::none()).unwrap();

    let mut bytes = vec![];

    // Header: last-fragment bit set, followed by the length of the message.
    let header = 0x8000_0000 | u32::try_from(msg.len()).unwrap();
    bytes.extend_from_slice(&header.to_be_bytes());
    bytes.extend_from_slice(&msg);

    // Test decoding - expect successful decoding.
    let v = Frame::<PublicKey>::from_xdr(&bytes, Limits::none());
    assert_eq!(v, Ok(Frame(pk)));
}

#[test]
fn frame_read_non_last_fragment_unsupported() {
    let pk = PublicKey::PublicKeyTypeEd25519(Uint256([1; 32]));
    let msg = pk.to_xdr(Limits::none()).unwrap();

    let mut bytes = vec![];

    // Header: last-fragment bit not set, followed by the length of the message.
    let header = u32::try_from(msg.len()).unwrap();
    bytes.extend_from_slice(&header.to_be_bytes());
    bytes.extend_from_slice(&msg);

    // Test decoding - expect error because multi-fragment frames are unsupported.
    let v = Frame::<PublicKey>::from_xdr(&bytes, Limits::none());
    assert_eq!(v, Err(Error::Unsupported));
}

#[test]
fn frame_read_multi_frame() {
    let pk1 = PublicKey::PublicKeyTypeEd25519(Uint256([1; 32]));
    let msg1 = pk1.to_xdr(Limits::none()).unwrap();
    let pk2 = PublicKey::PublicKeyTypeEd25519(Uint256([2; 32]));
    let msg2 = pk2.to_xdr(Limits::none()).unwrap();
    let pk3 = PublicKey::PublicKeyTypeEd25519(Uint256([3; 32]));
    let msg3 = pk3.to_xdr(Limits::none()).unwrap();

    let mut bytes = vec![];

    // Multiple messages, each with their own header.
    let header1 = 0x8000_0000 | u32::try_from(msg1.len()).unwrap();
    bytes.extend_from_slice(&header1.to_be_bytes());
    bytes.extend_from_slice(&msg1);
    let header2 = 0x8000_0000 | u32::try_from(msg2.len()).unwrap();
    bytes.extend_from_slice(&header2.to_be_bytes());
    bytes.extend_from_slice(&msg2);
    let header3 = 0x8000_0000 | u32::try_from(msg3.len()).unwrap();
    bytes.extend_from_slice(&header3.to_be_bytes());
    bytes.extend_from_slice(&msg3);

    // Test decoding - expect successful decoding of all then EOF.
    let mut cursor = Cursor::new(bytes);
    let mut limited = Limited::new(&mut cursor, Limits::none());
    let v1 = Frame::<PublicKey>::read_xdr(&mut limited);
    assert_eq!(v1, Ok(Frame(pk1)));
    let v2 = Frame::<PublicKey>::read_xdr(&mut limited);
    assert_eq!(v2, Ok(Frame(pk2)));
    let v3 = Frame::<PublicKey>::read_xdr(&mut limited);
    assert_eq!(v3, Ok(Frame(pk3.clone())));
    let v4 = Frame::<PublicKey>::read_xdr(&mut limited);
    assert_eq!(v4, Err(Error::Io(std::io::ErrorKind::UnexpectedEof.into())));
}
