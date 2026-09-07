#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerCloseMetaV0 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaV0
/// {
///     LedgerHeaderHistoryEntry ledgerHeader;
///     // NB: txSet is sorted in "Hash order"
///     TransactionSet txSet;
///
///     // NB: transactions are sorted in apply order here
///     // fees for all transactions are processed first
///     // followed by applying transactions
///     TransactionResultMeta txProcessing<>;
///
///     // upgrades are applied last
///     UpgradeEntryMeta upgradesProcessing<>;
///
///     // other misc information attached to the ledger close
///     SCPHistoryEntry scpInfo<>;
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
pub struct LedgerCloseMetaV0 {
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: TransactionSet,
    pub tx_processing: VecM<TransactionResultMeta>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
}

impl ReadXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
                tx_set: TransactionSet::read_xdr(r)?,
                tx_processing: VecM::<TransactionResultMeta>::read_xdr(r)?,
                upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
                scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerCloseMetaV0View is a borrowing equivalent of [`LedgerCloseMetaV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaV0View<'a> {
    pub ledger_header: LedgerHeaderHistoryEntryView<'a>,
    pub tx_set: TransactionSetView<'a>,
    pub tx_processing: VecMView<'a, TransactionResultMetaView<'a>>,
    pub upgrades_processing: VecMView<'a, UpgradeEntryMetaView<'a>>,
    pub scp_info: VecMView<'a, ScpHistoryEntryView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerCloseMetaV0View<'_> {
    type Owned = LedgerCloseMetaV0;
    fn into_owned(&self) -> LedgerCloseMetaV0 {
        LedgerCloseMetaV0 {
            ledger_header: IntoOwned::into_owned(&self.ledger_header),
            tx_set: IntoOwned::into_owned(&self.tx_set),
            tx_processing: IntoOwned::into_owned(&self.tx_processing),
            upgrades_processing: IntoOwned::into_owned(&self.upgrades_processing),
            scp_info: IntoOwned::into_owned(&self.scp_info),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseMetaV0View<'_>> for LedgerCloseMetaV0 {
    #[must_use]
    fn from(v: &LedgerCloseMetaV0View<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseMetaV0View<'_>> for LedgerCloseMetaV0 {
    #[must_use]
    fn from(v: LedgerCloseMetaV0View<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for LedgerCloseMetaV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            Ok(())
        })
    }
}
