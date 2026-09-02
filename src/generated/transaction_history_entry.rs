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
impl From<&TransactionHistoryEntryView<'_>> for TransactionHistoryEntry {
    #[must_use]
    fn from(v: &TransactionHistoryEntryView<'_>) -> Self {
        Self {
            ledger_seq: v.ledger_seq,
            tx_set: (&v.tx_set).into(),
            ext: (&v.ext).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionHistoryEntryView<'_>> for TransactionHistoryEntry {
    #[must_use]
    fn from(v: TransactionHistoryEntryView<'_>) -> Self {
        Self::from(&v)
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

#[cfg(feature = "const")]
impl TransactionHistoryEntryView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_history_entry(self);
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
        w.write_type_transaction_history_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionHistoryEntry`], mirroring `<TransactionHistoryEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_history_entry(
        &mut self,
        v: &TransactionHistoryEntryView<'_>,
    ) {
        self.write_u32(v.ledger_seq);
        self.write_type_transaction_set(&v.tx_set);
        self.write_type_transaction_history_entry_ext(&v.ext);
    }
}
