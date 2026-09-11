#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMetaV2 is a borrowing equivalent of [`SorobanTransactionMetaV2`](super::super::SorobanTransactionMetaV2)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionMetaV2 {
    pub ext: SorobanTransactionMetaExt,
    pub return_value: Option<ScVal>,
}

impl SorobanTransactionMetaV2 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_transaction_meta_v2(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_soroban_transaction_meta_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SorobanTransactionMetaV2`], mirroring `<SorobanTransactionMetaV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_transaction_meta_v2(&mut self, v: &SorobanTransactionMetaV2) {
        self.write_type_soroban_transaction_meta_ext(&v.ext);
        self.write_type_option_sc_val(&v.return_value);
    }

    /// Serializes an optional [`SorobanTransactionMetaV2`], mirroring `<Option<SorobanTransactionMetaV2> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_soroban_transaction_meta_v2(
        &mut self,
        v: &Option<SorobanTransactionMetaV2>,
    ) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_soroban_transaction_meta_v2(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
