#![cfg(all(feature = "const", feature = "std"))]

//! Tests for the const XDR serializers: `const_write_xdr`, `const_to_xdr`, and
//! `const_xdr_len`. Compared against the streaming `write_xdr` (needs `std`).

use std::io::Cursor;
use stellar_xdr::{
    ConstError, ConstWriter, Hash, Limited, Limits, Memo, MemoType, ReadXdr, TimeBounds, TimePoint,
    Uint256, WriteXdr,
};

/// Serialize via the streaming `write_xdr` path, for comparison.
fn streamed<T: WriteXdr>(v: &T) -> Vec<u8> {
    let mut w = Limited::new(Cursor::new(Vec::new()), Limits::none());
    v.write_xdr(&mut w).unwrap();
    w.inner.into_inner()
}

#[test]
fn const_len_and_array_match_write_xdr() {
    // Fixed-length opaque newtype: 32 bytes, no length prefix.
    let u = Uint256([7u8; 32]);
    assert_eq!(u.const_xdr_len(), streamed(&u).len());
    assert_eq!(u.const_to_xdr::<32>().as_slice(), streamed(&u).as_slice());

    // Struct of newtypes over scalars: 2 * u64 = 16 bytes.
    let tb = TimeBounds {
        min_time: TimePoint(1),
        max_time: TimePoint(u64::MAX),
    };
    assert_eq!(tb.const_xdr_len(), streamed(&tb).len());
    assert_eq!(tb.const_to_xdr::<16>().as_slice(), streamed(&tb).as_slice());

    // Enum: 4 bytes.
    assert_eq!(
        MemoType::Id.const_to_xdr::<4>().as_slice(),
        streamed(&MemoType::Id).as_slice()
    );
}

#[test]
fn const_write_xdr_matches_write_xdr() {
    // Union arms: void, length-prefixed-and-padded string, scalar, newtype.
    let memos = [
        Memo::None,
        Memo::Text("hello".as_bytes().to_vec().try_into().unwrap()),
        Memo::Id(42),
        Memo::Hash(Hash([9u8; 32])),
    ];
    for m in memos {
        let n = m.const_xdr_len();
        let mut buf = vec![0u8; n];
        let mut w = ConstWriter::new(&mut buf, &Limits::none());
        m.const_write_xdr(&mut w);
        assert_eq!(w.error(), None);
        assert_eq!(buf, streamed(&m), "{m:?}");
    }
}

#[test]
fn const_round_trips() {
    let tb = TimeBounds {
        min_time: TimePoint(1),
        max_time: TimePoint(2),
    };
    let bytes = tb.const_to_xdr::<16>();
    assert_eq!(TimeBounds::from_xdr(bytes, Limits::none()).unwrap(), tb);
}

#[test]
fn const_write_xdr_respects_limits() {
    let u = Uint256([0u8; 32]);

    let mut buf = [0u8; 32];
    let mut w = ConstWriter::new(&mut buf, &Limits::depth(0));
    u.const_write_xdr(&mut w);
    assert_eq!(w.error(), Some(ConstError::DepthLimitExceeded));

    // A 32-byte value needs 32 bytes; a 16-byte budget must fail.
    let mut buf2 = [0u8; 32];
    let mut w2 = ConstWriter::new(&mut buf2, &Limits::len(16));
    u.const_write_xdr(&mut w2);
    assert_eq!(w2.error(), Some(ConstError::LengthLimitExceeded));
}

// Compile-time serialization to a fixed array, the way a proc-macro would emit
// it: size the array with `const_xdr_len`, then fill it with `const_to_xdr`.
const TB: TimeBounds = TimeBounds {
    min_time: TimePoint(1),
    max_time: TimePoint(0x0102_0304_0506_0708),
};
const TB_LEN: usize = TB.const_xdr_len();
const TB_XDR: [u8; TB_LEN] = TB.const_to_xdr::<TB_LEN>();

#[test]
fn const_context() {
    assert_eq!(TB_LEN, 16);
    assert_eq!(TB_XDR.to_vec(), streamed(&TB));
}
