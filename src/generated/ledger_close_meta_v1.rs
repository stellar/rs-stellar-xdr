#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerCloseMetaV1 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaV1
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
///     TransactionResultMeta txProcessing<>;
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
///
///     // Maintained for backwards compatibility, should never be populated.
///     LedgerEntry unused<>;
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
pub struct LedgerCloseMetaV1 {
    pub ext: LedgerCloseMetaExt,
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: GeneralizedTransactionSet,
    pub tx_processing: VecM<TransactionResultMeta>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_byte_size_of_live_soroban_state: u64,
    pub evicted_keys: VecM<LedgerKey>,
    pub unused: VecM<LedgerEntry>,
}

impl ReadXdr for LedgerCloseMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: LedgerCloseMetaExt::read_xdr(r)?,
                ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
                tx_set: GeneralizedTransactionSet::read_xdr(r)?,
                tx_processing: VecM::<TransactionResultMeta>::read_xdr(r)?,
                upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
                scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
                total_byte_size_of_live_soroban_state: u64::read_xdr(r)?,
                evicted_keys: VecM::<LedgerKey>::read_xdr(r)?,
                unused: VecM::<LedgerEntry>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaV1 {
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
            self.unused.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerCloseMetaV1Ref is a borrowing equivalent of [`LedgerCloseMetaV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaV1Ref<'a> {
    pub ext: LedgerCloseMetaExt,
    pub ledger_header: LedgerHeaderHistoryEntryRef<'a>,
    pub tx_set: GeneralizedTransactionSetRef<'a>,
    pub tx_processing: VecMRef<'a, TransactionResultMetaRef<'a>>,
    pub upgrades_processing: VecMRef<'a, UpgradeEntryMetaRef<'a>>,
    pub scp_info: VecMRef<'a, ScpHistoryEntryRef<'a>>,
    pub total_byte_size_of_live_soroban_state: u64,
    pub evicted_keys: VecMRef<'a, LedgerKeyRef<'a>>,
    pub unused: VecMRef<'a, LedgerEntryRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerCloseMetaV1Ref<'_> {
    type Owned = LedgerCloseMetaV1;
    fn into_owned(self) -> LedgerCloseMetaV1 {
        LedgerCloseMetaV1 {
            ext: self.ext.into_owned(),
            ledger_header: self.ledger_header.into_owned(),
            tx_set: self.tx_set.into_owned(),
            tx_processing: self.tx_processing.into_owned(),
            upgrades_processing: self.upgrades_processing.into_owned(),
            scp_info: self.scp_info.into_owned(),
            total_byte_size_of_live_soroban_state: self
                .total_byte_size_of_live_soroban_state
                .into_owned(),
            evicted_keys: self.evicted_keys.into_owned(),
            unused: self.unused.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseMetaV1Ref<'_>> for LedgerCloseMetaV1 {
    #[must_use]
    fn from(v: &LedgerCloseMetaV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseMetaV1Ref<'_>> for LedgerCloseMetaV1 {
    #[must_use]
    fn from(v: LedgerCloseMetaV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for LedgerCloseMetaV1Ref<'_> {
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
            self.unused.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl LedgerCloseMetaV1Ref<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_close_meta_v1(self);
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
        w.write_type_ledger_close_meta_v1(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerCloseMetaV1`], mirroring `<LedgerCloseMetaV1 as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_close_meta_v1(&mut self, v: &LedgerCloseMetaV1Ref<'_>) {
        self.write_type_ledger_close_meta_ext(&v.ext);
        self.write_type_ledger_header_history_entry(&v.ledger_header);
        self.write_type_generalized_transaction_set(&v.tx_set);
        self.write_type_vec_transaction_result_meta(&v.tx_processing);
        self.write_type_vec_upgrade_entry_meta(&v.upgrades_processing);
        self.write_type_vec_scp_history_entry(&v.scp_info);
        self.write_u64(v.total_byte_size_of_live_soroban_state);
        self.write_type_vec_ledger_key(&v.evicted_keys);
        self.write_type_vec_ledger_entry(&v.unused);
    }
}
