#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionHistoryResultEntry is an XDR Struct defined as:
///
/// ```text
/// struct TransactionHistoryResultEntry
/// {
///     uint32 ledgerSeq;
///     TransactionResultSet txResultSet;
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
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
pub struct TransactionHistoryResultEntry {
    pub ledger_seq: u32,
    pub tx_result_set: TransactionResultSet,
    pub ext: TransactionHistoryResultEntryExt,
}

impl ReadXdr for TransactionHistoryResultEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_seq: u32::read_xdr(r)?,
                tx_result_set: TransactionResultSet::read_xdr(r)?,
                ext: TransactionHistoryResultEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionHistoryResultEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_seq.write_xdr(w)?;
            self.tx_result_set.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionHistoryResultEntryView is a borrowing equivalent of [`TransactionHistoryResultEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionHistoryResultEntryView<'a> {
    pub ledger_seq: u32,
    pub tx_result_set: TransactionResultSetView<'a>,
    pub ext: TransactionHistoryResultEntryExt,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionHistoryResultEntryView<'_> {
    type Owned = TransactionHistoryResultEntry;
    fn into_owned(&self) -> TransactionHistoryResultEntry {
        TransactionHistoryResultEntry {
            ledger_seq: IntoOwned::into_owned(&self.ledger_seq),
            tx_result_set: IntoOwned::into_owned(&self.tx_result_set),
            ext: IntoOwned::into_owned(&self.ext),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionHistoryResultEntryView<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: &TransactionHistoryResultEntryView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionHistoryResultEntryView<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: TransactionHistoryResultEntryView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for TransactionHistoryResultEntryView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_seq.write_xdr(w)?;
            self.tx_result_set.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
