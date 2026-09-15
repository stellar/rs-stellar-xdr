#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StellarValueExt is a borrowing equivalent of [`StellarValueExt`](super::super::StellarValueExt)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum StellarValueExt {
    Basic,
    Signed(LedgerCloseValueSignature),
    EmptyTxSet(StellarValueProposedValue),
}

impl StellarValueExt {
    #[must_use]
    pub const fn discriminant(&self) -> StellarValueType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Basic => StellarValueType::Basic,
            Self::Signed(_) => StellarValueType::Signed,
            Self::EmptyTxSet(_) => StellarValueType::EmptyTxSet,
        }
    }
}

impl StellarValueExt {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_stellar_value_ext(self);
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
        w.write_type_stellar_value_ext(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`StellarValueExt`], mirroring `<StellarValueExt as WriteXdr>::write_xdr`.
    pub const fn write_type_stellar_value_ext(&mut self, v: &StellarValueExt) {
        let d = v.discriminant();
        self.write_type_stellar_value_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            StellarValueExt::Basic => {}
            StellarValueExt::Signed(value) => {
                self.write_type_ledger_close_value_signature(value);
            }
            StellarValueExt::EmptyTxSet(value) => {
                self.write_type_stellar_value_proposed_value(value);
            }
        }
    }
}
