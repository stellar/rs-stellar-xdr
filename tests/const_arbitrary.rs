//! Differential test of the const XDR encoder against the owned one, over
//! values built by the `arbitrary` crate.
//!
//! The const form of a type mirrors the owned form field for field, and the
//! `Arbitrary` impls of the types the const form substitutes — `VecM` for
//! `Vec`, `BytesM` and `StringM` for `Vec<u8>`, a `&'static` reference for a
//! `Box` — consume input bytes exactly as the owned ones do. Driving both
//! derives from the same input therefore yields the same value in both forms,
//! with no conversion between them, and the two encoders must agree.

#![cfg(all(feature = "const", feature = "arbitrary", feature = "std"))]

use arbitrary::{Arbitrary, Unstructured};
use stellar_xdr::{r#const, Limits, WriteXdr};

/// A deterministic source of input bytes, so a failure names a seed that
/// reproduces it.
struct Rng(u64);

impl Rng {
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n)
            .map(|_| {
                self.0 ^= self.0 << 13;
                self.0 ^= self.0 >> 7;
                self.0 ^= self.0 << 17;
                #[allow(clippy::cast_possible_truncation)]
                {
                    self.0 as u8
                }
            })
            .collect()
    }
}

/// Encodes a const value at runtime, the way a caller in a const context does
/// at compile time: measure with an empty buffer, then write into a buffer of
/// that size.
macro_rules! const_encode {
    ($write:ident, $v:expr) => {{
        let mut empty: [u8; 0] = [];
        let mut w = r#const::ConstWriter::new(&mut empty);
        w.$write($v);
        let mut buf = vec![0u8; w.len()];
        let mut w = r#const::ConstWriter::new(&mut buf);
        w.$write($v);
        buf
    }};
}

/// Asserts that a value built from `data` encodes identically in both forms.
macro_rules! assert_same_encoding {
    ($type:ident, $write:ident, $data:expr) => {{
        let owned = stellar_xdr::$type::arbitrary(&mut Unstructured::new($data));
        let konst = r#const::$type::arbitrary(&mut Unstructured::new($data));
        // Reading the same input in lockstep means the two forms agree on
        // whether the input describes a value at all.
        assert_eq!(
            owned.is_ok(),
            konst.is_ok(),
            "{} built in one form but not the other",
            stringify!($type),
        );
        if let (Ok(owned), Ok(konst)) = (owned, konst) {
            assert_eq!(
                const_encode!($write, &konst),
                owned.to_xdr(Limits::none()).unwrap(),
                "{} encodings differ",
                stringify!($type),
            );
            true
        } else {
            false
        }
    }};
}

#[test]
fn const_and_owned_encode_same_for_arbitrary_values() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let mut built = 0;
    for _ in 0..256 {
        let data = rng.bytes(512);
        let d = &data[..];
        // A spread of shapes: a deep composite, both recursive types (one
        // recursing through a VecM, one through a &'static reference), a
        // union over heap data, and a fixed-size type that the const module
        // re-exports rather than redefines.
        built += usize::from(assert_same_encoding!(
            TransactionEnvelope,
            write_type_transaction_envelope,
            d
        ));
        built += usize::from(assert_same_encoding!(ScVal, write_type_sc_val, d));
        built += usize::from(assert_same_encoding!(
            ClaimPredicate,
            write_type_claim_predicate,
            d
        ));
        built += usize::from(assert_same_encoding!(
            ScSpecTypeDef,
            write_type_sc_spec_type_def,
            d
        ));
        built += usize::from(assert_same_encoding!(Memo, write_type_memo, d));
        built += usize::from(assert_same_encoding!(
            LedgerEntry,
            write_type_ledger_entry,
            d
        ));
        built += usize::from(assert_same_encoding!(AccountId, write_type_account_id, d));
    }
    // Guard against the assertions never running because every value failed to
    // build.
    assert!(built > 512, "too few values built: {built}");
}
