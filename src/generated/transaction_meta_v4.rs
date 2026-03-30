#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV4 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV4
/// {
///     ExtensionPoint ext;
/// 
///     LedgerEntryChanges txChangesBefore;  // tx level changes before operations
///                                          // are applied if any
///     OperationMetaV2 operations<>;        // meta for each operation
///     LedgerEntryChanges txChangesAfter;   // tx level changes after operations are
///                                          // applied if any
///     SorobanTransactionMetaV2* sorobanMeta; // Soroban-specific meta (only for
///                                            // Soroban transactions).
/// 
///     TransactionEvent events<>; // Used for transaction-level events (like fee payment)
///     DiagnosticEvent diagnosticEvents<>; // Used for all diagnostic information
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
pub struct TransactionMetaV4 {
    pub ext: ExtensionPoint,
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMetaV2>,
    pub tx_changes_after: LedgerEntryChanges,
    pub soroban_meta: Option<SorobanTransactionMetaV2>,
    pub events: VecM<TransactionEvent>,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl ReadXdr for TransactionMetaV4 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMetaV2>::read_xdr(r)?,
                tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
                soroban_meta: Option::<SorobanTransactionMetaV2>::read_xdr(r)?,
                events: VecM::<TransactionEvent>::read_xdr(r)?,
                diagnostic_events: VecM::<DiagnosticEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV4 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.tx_changes_before.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.tx_changes_after.write_xdr(w)?;
            self.soroban_meta.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}
