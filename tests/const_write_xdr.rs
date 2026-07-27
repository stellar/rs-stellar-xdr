#![cfg(all(feature = "const", feature = "std"))]

//! Tests for the const XDR serializers: `const_write_xdr`, `const_to_xdr`, and
//! `const_xdr_len`. For types that own heap data these live on the borrowing
//! `Ref` form; for types that own none they live on the type itself. Output is
//! compared against the streaming `write_xdr` of the owned type (needs `std`).

use std::io::Cursor;
use stellar_xdr::{
    ConstError, ConstWriter, Hash, Limited, Limits, Memo, MemoRef, MemoType, ReadXdr, StringMRef,
    TimeBounds, TimePoint, TxDemandVector, TxDemandVectorRef, Uint256, VecMRef, WriteXdr,
};

/// Serialize via the streaming `write_xdr` path, for comparison.
fn streamed<T: WriteXdr>(v: &T) -> Vec<u8> {
    let mut w = Limited::new(Cursor::new(Vec::new()), Limits::none());
    v.write_xdr(&mut w).unwrap();
    w.inner.into_inner()
}

#[test]
fn const_on_owned_non_heap_types() {
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
fn const_on_ref_union_matches_write_xdr() {
    // Union arms exercised through the borrowing MemoRef: void, length-prefixed
    // and padded string, scalar, and a fixed-opaque newtype.
    let cases: [MemoRef; 4] = [
        MemoRef::None,
        MemoRef::Text(StringMRef::new_str("hello")),
        MemoRef::Id(42),
        MemoRef::Hash(Hash([9u8; 32])),
    ];
    for m in cases {
        let owned: Memo = (&m).into();
        let n = m.const_xdr_len();
        let mut buf = vec![0u8; n];
        let mut w = ConstWriter::new(&mut buf, &Limits::none());
        m.const_write_xdr(&mut w);
        assert_eq!(w.error(), None);
        assert_eq!(buf, streamed(&owned), "{owned:?}");
    }
}

#[test]
fn const_on_ref_var_array() {
    // A newtype over `VecM<Hash>`, serialized through its borrowing Ref form,
    // built from a slice of fixed-size arrays.
    let hashes = [Hash([1u8; 32]), Hash([2u8; 32]), Hash([3u8; 32])];
    let v = TxDemandVectorRef(VecMRef::new(&hashes));
    let owned: TxDemandVector = (&v).into();
    assert_eq!(v.const_xdr_len(), streamed(&owned).len());
    // 4-byte length prefix + 3 * 32 bytes = 100.
    assert_eq!(
        v.const_to_xdr::<100>().as_slice(),
        streamed(&owned).as_slice()
    );
}

#[test]
fn const_round_trips() {
    let m = MemoRef::Text(StringMRef::new_str("round trip"));
    let owned: Memo = (&m).into();
    // 4 (discriminant) + 4 (len) + 10 ("round trip") + 2 (pad) = 20.
    let bytes = m.const_to_xdr::<20>();
    assert_eq!(Memo::from_xdr(bytes, Limits::none()).unwrap(), owned);
}

#[test]
fn const_respects_limits() {
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
// Both an owned non-heap type and a borrowing Ref type are exercised entirely
// in const contexts.
const TB: TimeBounds = TimeBounds {
    min_time: TimePoint(1),
    max_time: TimePoint(0x0102_0304_0506_0708),
};
const TB_LEN: usize = TB.const_xdr_len();
const TB_XDR: [u8; TB_LEN] = TB.const_to_xdr::<TB_LEN>();

const MEMO: MemoRef = MemoRef::Text(StringMRef::new_str("hi"));
const MEMO_LEN: usize = MEMO.const_xdr_len();
const MEMO_XDR: [u8; MEMO_LEN] = MEMO.const_to_xdr::<MEMO_LEN>();

#[test]
fn const_context() {
    assert_eq!(TB_LEN, 16);
    assert_eq!(TB_XDR.to_vec(), streamed(&TB));

    // 4 (discriminant) + 4 (len) + 2 ("hi") + 2 (pad) = 12.
    assert_eq!(MEMO_LEN, 12);
    let owned: Memo = (&MEMO).into();
    assert_eq!(MEMO_XDR.to_vec(), streamed(&owned));
}
