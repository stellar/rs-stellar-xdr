#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV2
/// {
///     LedgerEntryChanges txChangesBefore; // tx level changes before operations
///                                         // are applied if any
///     OperationMeta operations<>;         // meta for each operation
///     LedgerEntryChanges txChangesAfter;  // tx level changes after operations are
///                                         // applied if any
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
pub struct TransactionMetaV2 {
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
    pub tx_changes_after: LedgerEntryChanges,
}

impl ReadXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMeta>::read_xdr(r)?,
                tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionMetaV2Ref is a borrowing equivalent of [`TransactionMetaV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV2Ref<'a> {
    pub tx_changes_before: LedgerEntryChangesRef<'a>,
    pub operations: VecMRef<'a, OperationMetaRef<'a>>,
    pub tx_changes_after: LedgerEntryChangesRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionMetaV2Ref<'_>> for TransactionMetaV2 {
    #[must_use]
    fn from(v: &TransactionMetaV2Ref<'_>) -> Self {
        Self {
            tx_changes_before: (&v.tx_changes_before).into(),
            operations: v.operations.to_vecm_from(),
            tx_changes_after: (&v.tx_changes_after).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionMetaV2Ref<'_>> for TransactionMetaV2 {
    #[must_use]
    fn from(v: TransactionMetaV2Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionMetaV2Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            Ok(())
        })
    }
}
