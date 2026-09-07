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
impl IntoOwned for TransactionHistoryResultEntryRef<'_> {
    type Owned = TransactionHistoryResultEntry;
    fn into_owned(self) -> TransactionHistoryResultEntry {
        TransactionHistoryResultEntry {
            ledger_seq: self.ledger_seq.into_owned(),
            tx_result_set: self.tx_result_set.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionHistoryResultEntryRef<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: &TransactionHistoryResultEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionHistoryResultEntryRef<'_>> for TransactionHistoryResultEntry {
    #[must_use]
    fn from(v: TransactionHistoryResultEntryRef<'_>) -> Self {
        v.into_owned()
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

#[cfg(feature = "const")]
impl TransactionHistoryResultEntryRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_history_result_entry(self);
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
        w.write_type_transaction_history_result_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionHistoryResultEntry`], mirroring `<TransactionHistoryResultEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_history_result_entry(
        &mut self,
        v: &TransactionHistoryResultEntryRef<'_>,
    ) {
        self.write_u32(v.ledger_seq);
        self.write_type_transaction_result_set(&v.tx_result_set);
        self.write_type_transaction_history_result_entry_ext(&v.ext);
    }
}
