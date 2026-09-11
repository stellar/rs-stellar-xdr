#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StellarValueProposedValue is a borrowing equivalent of [`StellarValueProposedValue`](super::super::StellarValueProposedValue)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct StellarValueProposedValue {
    pub tx_set_hash: Hash,
    pub previous_ledger_hash: Hash,
    pub previous_ledger_version: u32,
    pub lc_value_signature: LedgerCloseValueSignature,
}

impl StellarValueProposedValue {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_stellar_value_proposed_value(self);
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
        w.write_type_stellar_value_proposed_value(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`StellarValueProposedValue`], mirroring `<StellarValueProposedValue as WriteXdr>::write_xdr`.
    pub const fn write_type_stellar_value_proposed_value(&mut self, v: &StellarValueProposedValue) {
        self.write_type_hash(&v.tx_set_hash);
        self.write_type_hash(&v.previous_ledger_hash);
        self.write_u32(v.previous_ledger_version);
        self.write_type_ledger_close_value_signature(&v.lc_value_signature);
    }
}
