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
#[cfg_eval::cfg_eval]
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
