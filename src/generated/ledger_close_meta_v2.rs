#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerCloseMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaV2
/// {
///     LedgerCloseMetaExt ext;
///
///     LedgerHeaderHistoryEntry ledgerHeader;
///
///     GeneralizedTransactionSet txSet;
///
///     // NB: transactions are sorted in apply order here
///     // fees for all transactions are processed first
///     // followed by applying transactions
///     TransactionResultMetaV1 txProcessing<>;
///
///     // upgrades are applied last
///     UpgradeEntryMeta upgradesProcessing<>;
///
///     // other misc information attached to the ledger close
///     SCPHistoryEntry scpInfo<>;
///
///     // Size in bytes of live Soroban state, to support downstream
///     // systems calculating storage fees correctly.
///     uint64 totalByteSizeOfLiveSorobanState;
///
///     // TTL and data/code keys that have been evicted at this ledger.
///     LedgerKey evictedKeys<>;
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
pub struct LedgerCloseMetaV2 {
    pub ext: LedgerCloseMetaExt,
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: GeneralizedTransactionSet,
    pub tx_processing: VecM<TransactionResultMetaV1>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_byte_size_of_live_soroban_state: u64,
    pub evicted_keys: VecM<LedgerKey>,
}

impl ReadXdr for LedgerCloseMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: LedgerCloseMetaExt::read_xdr(r)?,
                ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
                tx_set: GeneralizedTransactionSet::read_xdr(r)?,
                tx_processing: VecM::<TransactionResultMetaV1>::read_xdr(r)?,
                upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
                scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
                total_byte_size_of_live_soroban_state: u64::read_xdr(r)?,
                evicted_keys: VecM::<LedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            self.total_byte_size_of_live_soroban_state.write_xdr(w)?;
            self.evicted_keys.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerCloseMetaV2View is a borrowing equivalent of [`LedgerCloseMetaV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaV2View<'a> {
    pub ext: LedgerCloseMetaExt,
    pub ledger_header: LedgerHeaderHistoryEntryView<'a>,
    pub tx_set: GeneralizedTransactionSetView<'a>,
    pub tx_processing: VecMView<'a, TransactionResultMetaV1View<'a>>,
    pub upgrades_processing: VecMView<'a, UpgradeEntryMetaView<'a>>,
    pub scp_info: VecMView<'a, ScpHistoryEntryView<'a>>,
    pub total_byte_size_of_live_soroban_state: u64,
    pub evicted_keys: VecMView<'a, LedgerKeyView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseMetaV2View<'_>> for LedgerCloseMetaV2 {
    #[must_use]
    fn from(v: &LedgerCloseMetaV2View<'_>) -> Self {
        Self {
            ext: v.ext.clone(),
            ledger_header: (&v.ledger_header).into(),
            tx_set: (&v.tx_set).into(),
            tx_processing: v.tx_processing.to_vecm(),
            upgrades_processing: v.upgrades_processing.to_vecm(),
            scp_info: v.scp_info.to_vecm(),
            total_byte_size_of_live_soroban_state: v.total_byte_size_of_live_soroban_state,
            evicted_keys: v.evicted_keys.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseMetaV2View<'_>> for LedgerCloseMetaV2 {
    #[must_use]
    fn from(v: LedgerCloseMetaV2View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for LedgerCloseMetaV2View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.ledger_header.write_xdr(w)?;
            self.tx_set.write_xdr(w)?;
            self.tx_processing.write_xdr(w)?;
            self.upgrades_processing.write_xdr(w)?;
            self.scp_info.write_xdr(w)?;
            self.total_byte_size_of_live_soroban_state.write_xdr(w)?;
            self.evicted_keys.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl LedgerCloseMetaV2View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_close_meta_v2(self);
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
        w.write_type_ledger_close_meta_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerCloseMetaV2`], mirroring `<LedgerCloseMetaV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_close_meta_v2(&mut self, v: &LedgerCloseMetaV2View<'_>) {
        self.write_type_ledger_close_meta_ext(&v.ext);
        self.write_type_ledger_header_history_entry(&v.ledger_header);
        self.write_type_generalized_transaction_set(&v.tx_set);
        self.write_type_vec_transaction_result_meta_v1(&v.tx_processing);
        self.write_type_vec_upgrade_entry_meta(&v.upgrades_processing);
        self.write_type_vec_scp_history_entry(&v.scp_info);
        self.write_u64(v.total_byte_size_of_live_soroban_state);
        self.write_type_vec_ledger_key(&v.evicted_keys);
    }
}
