#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationMetaV2 is a borrowing equivalent of [`OperationMetaV2`](super::super::OperationMetaV2)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct OperationMetaV2 {
    pub ext: ExtensionPoint,
    pub changes: LedgerEntryChanges,
    pub events: VecM<ContractEvent>,
}

impl OperationMetaV2 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_operation_meta_v2(self);
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
        w.write_type_operation_meta_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`OperationMetaV2`], mirroring `<OperationMetaV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_operation_meta_v2(&mut self, v: &OperationMetaV2) {
        self.write_type_extension_point(&v.ext);
        self.write_type_ledger_entry_changes(&v.changes);
        self.write_type_vec_contract_event(&v.events);
    }

    /// Serializes a variable-length array of [`OperationMetaV2`], mirroring `<VecM<OperationMetaV2, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_operation_meta_v2<const MAX: u32>(
        &mut self,
        v: &VecM<OperationMetaV2, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_operation_meta_v2(&s[i]);
            i += 1;
        }
    }
}
