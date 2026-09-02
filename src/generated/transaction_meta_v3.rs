#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV3 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV3
/// {
///     ExtensionPoint ext;
///
///     LedgerEntryChanges txChangesBefore;  // tx level changes before operations
///                                          // are applied if any
///     OperationMeta operations<>;          // meta for each operation
///     LedgerEntryChanges txChangesAfter;   // tx level changes after operations are
///                                          // applied if any
///     SorobanTransactionMeta* sorobanMeta; // Soroban-specific meta (only for
///                                          // Soroban transactions).
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TransactionMetaV3 {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
    pub tx_changes_after: LedgerEntryChanges,
    pub soroban_meta: Option<SorobanTransactionMeta>,
}

impl ReadXdr for TransactionMetaV3 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMeta>::read_xdr(r)?,
                tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
                soroban_meta: Option::<SorobanTransactionMeta>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV3 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            self.soroban_meta.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionMetaV3Ref is a borrowing equivalent of [`TransactionMetaV3`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV3Ref<'a> {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChangesRef<'a>,
    pub operations: VecMRef<'a, OperationMetaRef<'a>>,
    pub tx_changes_after: LedgerEntryChangesRef<'a>,
    pub soroban_meta: Option<SorobanTransactionMetaRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionMetaV3Ref<'_> {
    type Owned = TransactionMetaV3;
    fn into_owned(self) -> TransactionMetaV3 {
        TransactionMetaV3 {
            ext: self.ext.into_owned(),
            tx_changes_before: self.tx_changes_before.into_owned(),
            operations: self.operations.into_owned(),
            tx_changes_after: self.tx_changes_after.into_owned(),
            soroban_meta: self.soroban_meta.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionMetaV3Ref<'_>> for TransactionMetaV3 {
    #[must_use]
    fn from(v: &TransactionMetaV3Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionMetaV3Ref<'_>> for TransactionMetaV3 {
    #[must_use]
    fn from(v: TransactionMetaV3Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionMetaV3Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            self.soroban_meta.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionMetaV3View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_meta_v3(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
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
        w.write_type_transaction_meta_v3(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionMetaV3`], mirroring `<TransactionMetaV3 as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_meta_v3(&mut self, v: &TransactionMetaV3View<'_>) {
        self.write_type_extension_point(&v.ext);
        self.write_type_ledger_entry_changes(&v.tx_changes_before);
        self.write_type_vec_operation_meta(&v.operations);
        self.write_type_ledger_entry_changes(&v.tx_changes_after);
        self.write_type_option_soroban_transaction_meta(&v.soroban_meta);
    }
}
