#![cfg(feature = "std")]

//! Tests for the const XDR serializer (`const_to_xdr`) that underlies `to_xdr`.

use std::io::Cursor;
use stellar_xdr::{
    ConstWriter, Hash, Limited, Limits, Memo, MemoType, ReadXdr, TimeBounds, TimePoint, Uint256,
    WriteXdr,
};

/// Serialize via the streaming `write_xdr` path, independently of `to_xdr`
/// (which now goes through the const path), so the two can be compared.
fn streamed<T: WriteXdr>(v: &T) -> Vec<u8> {
    let mut w = Limited::new(Cursor::new(Vec::new()), Limits::none());
    v.write_xdr(&mut w).unwrap();
    w.inner.into_inner()
}

#[test]
fn const_to_xdr_matches_write_xdr() {
    // Newtype over a scalar.
    let tp = TimePoint(0x0102_0304_0506_0708);
    assert_eq!(tp.to_xdr(Limits::none()).unwrap(), streamed(&tp));

    // Struct of newtypes.
    let tb = TimeBounds {
        min_time: TimePoint(1),
        max_time: TimePoint(u64::MAX),
    };
    assert_eq!(tb.to_xdr(Limits::none()).unwrap(), streamed(&tb));

    // Fixed-length opaque newtype.
    let u = Uint256([7u8; 32]);
    assert_eq!(u.to_xdr(Limits::none()).unwrap(), streamed(&u));

    // Enum.
    assert_eq!(
        MemoType::Id.to_xdr(Limits::none()).unwrap(),
        streamed(&MemoType::Id)
    );

    // Union: void, length-prefixed-and-padded string, scalar, and newtype arms.
    let memos = [
        Memo::None,
        Memo::Text("hello".as_bytes().to_vec().try_into().unwrap()),
        Memo::Id(42),
        Memo::Hash(Hash([9u8; 32])),
    ];
    for m in memos {
        assert_eq!(m.to_xdr(Limits::none()).unwrap(), streamed(&m), "{m:?}");
    }
}

#[test]
fn const_to_xdr_round_trips() {
    let tb = TimeBounds {
        min_time: TimePoint(1),
        max_time: TimePoint(2),
    };
    let bytes = tb.to_xdr(Limits::none()).unwrap();
    assert_eq!(TimeBounds::from_xdr(&bytes, Limits::none()).unwrap(), tb);
}

#[test]
fn const_to_xdr_respects_limits() {
    // A 32-byte fixed opaque needs 32 bytes; a smaller length budget must error,
    // matching write_xdr's limit behavior.
    let u = Uint256([0u8; 32]);
    assert_eq!(u.to_xdr(Limits::len(16)), Err(stellar_xdr::Error::LengthLimitExceeded));
    assert!(u.to_xdr(Limits::len(32)).is_ok());

    // A depth limit of zero fails immediately, as it does for write_xdr.
    assert_eq!(u.to_xdr(Limits::depth(0)), Err(stellar_xdr::Error::DepthLimitExceeded));
}

/// The const serializer is usable in a `const` context.
const MEMOTYPE_ID_XDR: [u8; 4] = {
    let mut buf = [0u8; 4];
    // The writer borrows `buf`; scope it so the borrow ends before `buf` is
    // returned.
    let _len = {
        let mut w = ConstWriter::new(
            &mut buf,
            &Limits {
                depth: u32::MAX,
                len: usize::MAX,
            },
        );
        MemoType::Id.const_to_xdr(&mut w);
        w.position()
    };
    buf
};

#[test]
fn const_to_xdr_in_const_context() {
    assert_eq!(MEMOTYPE_ID_XDR, [0, 0, 0, 2]);
    assert_eq!(
        MEMOTYPE_ID_XDR.to_vec(),
        MemoType::Id.to_xdr(Limits::none()).unwrap()
    );
}
