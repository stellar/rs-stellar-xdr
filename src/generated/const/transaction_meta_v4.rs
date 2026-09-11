#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV4 is a borrowing equivalent of [`TransactionMetaV4`](super::super::TransactionMetaV4)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct TransactionMetaV4 {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMetaV2>,
    pub tx_changes_after: LedgerEntryChanges,
    pub soroban_meta: Option<SorobanTransactionMetaV2>,
    pub events: VecM<TransactionEvent>,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl TransactionMetaV4 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_meta_v4(self);
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
        w.write_type_transaction_meta_v4(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`TransactionMetaV4`], mirroring `<TransactionMetaV4 as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_meta_v4(&mut self, v: &TransactionMetaV4) {
        self.write_type_extension_point(&v.ext);
        self.write_type_ledger_entry_changes(&v.tx_changes_before);
        self.write_type_vec_operation_meta_v2(&v.operations);
        self.write_type_ledger_entry_changes(&v.tx_changes_after);
        self.write_type_option_soroban_transaction_meta_v2(&v.soroban_meta);
        self.write_type_vec_transaction_event(&v.events);
        self.write_type_vec_diagnostic_event(&v.diagnostic_events);
    }
}
