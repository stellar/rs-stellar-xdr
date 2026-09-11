#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionEnvelope is a borrowing equivalent of [`TransactionEnvelope`](super::super::TransactionEnvelope)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum TransactionEnvelope {
    TxV0(TransactionV0Envelope),
    Tx(TransactionV1Envelope),
    TxFeeBump(FeeBumpTransactionEnvelope),
}

impl TransactionEnvelope {
    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => EnvelopeType::TxV0,
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }
}

impl TransactionEnvelope {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_envelope(self);
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
        w.write_type_transaction_envelope(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`TransactionEnvelope`], mirroring `<TransactionEnvelope as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_envelope(&mut self, v: &TransactionEnvelope) {
        let d = v.discriminant();
        self.write_type_envelope_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            TransactionEnvelope::TxV0(value) => {
                self.write_type_transaction_v0_envelope(value);
            }
            TransactionEnvelope::Tx(value) => {
                self.write_type_transaction_v1_envelope(value);
            }
            TransactionEnvelope::TxFeeBump(value) => {
                self.write_type_fee_bump_transaction_envelope(value);
            }
        }
    }

    /// Serializes a variable-length array of [`TransactionEnvelope`], mirroring `<VecM<TransactionEnvelope, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_transaction_envelope<const MAX: u32>(
        &mut self,
        v: &VecM<TransactionEnvelope, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_transaction_envelope(&s[i]);
            i += 1;
        }
    }
}
