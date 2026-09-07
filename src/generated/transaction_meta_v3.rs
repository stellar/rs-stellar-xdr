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

/// TransactionMetaV3View is a borrowing equivalent of [`TransactionMetaV3`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV3View<'a> {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChangesView<'a>,
    pub operations: VecMView<'a, OperationMetaView<'a>>,
    pub tx_changes_after: LedgerEntryChangesView<'a>,
    pub soroban_meta: Option<SorobanTransactionMetaView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionMetaV3View<'_> {
    type Owned = TransactionMetaV3;
    fn into_owned(&self) -> TransactionMetaV3 {
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
impl From<&TransactionMetaV3View<'_>> for TransactionMetaV3 {
    #[must_use]
    fn from(v: &TransactionMetaV3View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionMetaV3View<'_>> for TransactionMetaV3 {
    #[must_use]
    fn from(v: TransactionMetaV3View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionMetaV3View<'_> {
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
