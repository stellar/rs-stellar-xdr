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

impl LedgerCloseMetaV1 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ext.const_write_xdr(w);
        self.ledger_header.const_write_xdr(w);
        self.tx_set.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.tx_processing.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.upgrades_processing.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.scp_info.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.write_u64(self.total_byte_size_of_live_soroban_state);
        {
            w.enter_depth();
            let __s0 = self.evicted_keys.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.unused.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
