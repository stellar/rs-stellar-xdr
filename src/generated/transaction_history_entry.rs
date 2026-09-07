#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionHistoryEntry is an XDR Struct defined as:
///
/// ```text
/// struct TransactionHistoryEntry
/// {
///     uint32 ledgerSeq;
///     TransactionSet txSet;
///
///     // when v != 0, txSet must be empty
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         GeneralizedTransactionSet generalizedTxSet;
///     }
///     ext;
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
pub struct TransactionHistoryEntry {
    pub ledger_seq: u32,
    pub tx_set: TransactionSet,
    pub ext: TransactionHistoryEntryExt,
}

impl ReadXdr for TransactionHistoryEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_seq: u32::read_xdr(r)?,
                tx_set: TransactionSet::read_xdr(r)?,
                ext: TransactionHistoryEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionHistoryEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_seq.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionHistoryEntryView is a borrowing equivalent of [`TransactionHistoryEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionHistoryEntryView<'a> {
    pub ledger_seq: u32,
    pub tx_set: TransactionSetView<'a>,
    pub ext: TransactionHistoryEntryExtView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionHistoryEntryView<'_> {
    type Owned = TransactionHistoryEntry;
    fn into_owned(self) -> TransactionHistoryEntry {
        TransactionHistoryEntry {
            ledger_seq: self.ledger_seq.into_owned(),
            tx_set: self.tx_set.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionHistoryEntryView<'_>> for TransactionHistoryEntry {
    #[must_use]
    fn from(v: &TransactionHistoryEntryView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionHistoryEntryView<'_>> for TransactionHistoryEntry {
    #[must_use]
    fn from(v: TransactionHistoryEntryView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionHistoryEntryView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_seq.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
