//! Shared harness for the differential fuzz targets.

/// Builds a value in both the owned and the const form from the same input
/// bytes, and asserts the two encoders agree.
///
/// The const form of a type mirrors the owned form field for field, and the
/// `Arbitrary` impls of the types the const form substitutes — `VecM` for
/// `Vec`, `BytesM` and `StringM` for `Vec<u8>`, a `&'static` reference for a
/// `Box` — consume input bytes exactly as the owned ones do. Driving both
/// derives from the same input therefore yields the same value in both forms,
/// with no conversion between them, and the two encoders must agree.
///
/// `$write` is the [`ConstWriter`](stellar_xdr::r#const::ConstWriter) method
/// for `$type`, which is what a const context reaches for; going through it
/// rather than `const_to_xdr` keeps the buffer sizeable at runtime.
#[macro_export]
macro_rules! assert_same_encoding {
    ($type:ident, $write:ident, $data:expr) => {{
        use arbitrary::{Arbitrary, Unstructured};
        use stellar_xdr::{r#const, Limits, WriteXdr};

        let data: &[u8] = $data;
        let owned = stellar_xdr::$type::arbitrary(&mut Unstructured::new(data));
        let konst = r#const::$type::arbitrary(&mut Unstructured::new(data));

        // Reading the same input in lockstep means the two forms agree on
        // whether the input describes a value at all.
        assert_eq!(
            owned.is_ok(),
            konst.is_ok(),
            "{} built in one form but not the other",
            stringify!($type),
        );

        if let (Ok(owned), Ok(konst)) = (owned, konst) {
            // Encode the const value the way a caller in a const context does
            // at compile time: measure, then write into a buffer of that size.
            let mut buf = vec![0u8; konst.const_xdr_len()];
            let mut w = r#const::ConstWriter::new(&mut buf);
            w.$write(&konst);
            assert_eq!(
                w.len(),
                buf.len(),
                "{} const_xdr_len disagrees with what the writer wrote",
                stringify!($type),
            );
            assert_eq!(
                buf,
                owned.to_xdr(Limits::none()).unwrap(),
                "{} encodings differ",
                stringify!($type),
            );
        }
    }};
}
