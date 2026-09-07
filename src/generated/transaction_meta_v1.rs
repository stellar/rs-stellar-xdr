#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV1 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV1
/// {
///     LedgerEntryChanges txChanges; // tx level changes if any
///     OperationMeta operations<>;   // meta for each operation
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
pub struct TransactionMetaV1 {
    pub tx_changes: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
}

impl ReadXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_changes: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMeta>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionMetaV1View is a borrowing equivalent of [`TransactionMetaV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV1View<'a> {
    pub tx_changes: LedgerEntryChangesView<'a>,
    pub operations: VecMView<'a, OperationMetaView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionMetaV1View<'_> {
    type Owned = TransactionMetaV1;
    fn into_owned(self) -> TransactionMetaV1 {
        TransactionMetaV1 {
            tx_changes: self.tx_changes.into_owned(),
            operations: self.operations.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionMetaV1View<'_>> for TransactionMetaV1 {
    #[must_use]
    fn from(v: &TransactionMetaV1View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionMetaV1View<'_>> for TransactionMetaV1 {
    #[must_use]
    fn from(v: TransactionMetaV1View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionMetaV1View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            Ok(())
        })
    }
}
