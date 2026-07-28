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

/// TransactionHistoryResultEntryRef is a borrowing equivalent of [`TransactionHistoryResultEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionHistoryResultEntryRef<'a> {
    pub ledger_seq: u32,
    pub tx_result_set: TransactionResultSetRef<'a>,
    pub ext: TransactionHistoryResultEntryExt,
}

#[cfg(feature = "alloc")]
impl From<&TransactionHistoryResultEntryRef<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: &TransactionHistoryResultEntryRef<'_>) -> Self {
        Self {
            ledger_seq: v.ledger_seq,
            tx_result_set: (&v.tx_result_set).into(),
            ext: v.ext.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionHistoryResultEntryRef<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: TransactionHistoryResultEntryRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionHistoryResultEntryRef<'_> {
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
