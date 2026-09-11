#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionSignaturePayloadTaggedTransaction is a borrowing equivalent of [`TransactionSignaturePayloadTaggedTransaction`](super::super::TransactionSignaturePayloadTaggedTransaction)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    Tx(Transaction),
    TxFeeBump(FeeBumpTransaction),
}

impl TransactionSignaturePayloadTaggedTransaction {
    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }
}

impl TransactionSignaturePayloadTaggedTransaction {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_signature_payload_tagged_transaction(self);
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
        w.write_type_transaction_signature_payload_tagged_transaction(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`TransactionSignaturePayloadTaggedTransaction`], mirroring `<TransactionSignaturePayloadTaggedTransaction as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_signature_payload_tagged_transaction(
        &mut self,
        v: &TransactionSignaturePayloadTaggedTransaction,
    ) {
        let d = v.discriminant();
        self.write_type_envelope_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            TransactionSignaturePayloadTaggedTransaction::Tx(value) => {
                self.write_type_transaction(value);
            }
            TransactionSignaturePayloadTaggedTransaction::TxFeeBump(value) => {
                self.write_type_fee_bump_transaction(value);
            }
        }
    }
}
