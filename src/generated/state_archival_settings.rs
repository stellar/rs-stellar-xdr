#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StateArchivalSettings is an XDR Struct defined as:
///
/// ```text
/// struct StateArchivalSettings {
///     uint32 maxEntryTTL;
///     uint32 minTemporaryTTL;
///     uint32 minPersistentTTL;
///
///     // rent_fee = wfee_rate_average / rent_rate_denominator_for_type
///     int64 persistentRentRateDenominator;
///     int64 tempRentRateDenominator;
///
///     // max number of entries that emit archival meta in a single ledger
///     uint32 maxEntriesToArchive;
///
///     // Number of snapshots to use when calculating average live Soroban State size
///     uint32 liveSorobanStateSizeWindowSampleSize;
///
///     // How often to sample the live Soroban State size for the average, in ledgers
///     uint32 liveSorobanStateSizeWindowSamplePeriod;
///
///     // Maximum number of bytes that we scan for eviction per ledger
///     uint32 evictionScanSize;
///
///     // Lowest BucketList level to be scanned to evict entries
///     uint32 startingEvictionScanLevel;
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
pub struct StateArchivalSettings {
    pub max_entry_ttl: u32,
    pub min_temporary_ttl: u32,
    pub min_persistent_ttl: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub persistent_rent_rate_denominator: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub temp_rent_rate_denominator: i64,
    pub max_entries_to_archive: u32,
    pub live_soroban_state_size_window_sample_size: u32,
    pub live_soroban_state_size_window_sample_period: u32,
    pub eviction_scan_size: u32,
    pub starting_eviction_scan_level: u32,
}

impl ReadXdr for StateArchivalSettings {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                max_entry_ttl: u32::read_xdr(r)?,
                min_temporary_ttl: u32::read_xdr(r)?,
                min_persistent_ttl: u32::read_xdr(r)?,
                persistent_rent_rate_denominator: i64::read_xdr(r)?,
                temp_rent_rate_denominator: i64::read_xdr(r)?,
                max_entries_to_archive: u32::read_xdr(r)?,
                live_soroban_state_size_window_sample_size: u32::read_xdr(r)?,
                live_soroban_state_size_window_sample_period: u32::read_xdr(r)?,
                eviction_scan_size: u32::read_xdr(r)?,
                starting_eviction_scan_level: u32::read_xdr(r)?,
            })
        })
    }
}

impl StateArchivalSettings {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.max_entry_ttl);
        w.write_u32(self.min_temporary_ttl);
        w.write_u32(self.min_persistent_ttl);
        w.write_i64(self.persistent_rent_rate_denominator);
        w.write_i64(self.temp_rent_rate_denominator);
        w.write_u32(self.max_entries_to_archive);
        w.write_u32(self.live_soroban_state_size_window_sample_size);
        w.write_u32(self.live_soroban_state_size_window_sample_period);
        w.write_u32(self.eviction_scan_size);
        w.write_u32(self.starting_eviction_scan_level);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for StateArchivalSettings {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.max_entry_ttl.write_xdr(w)?;
            self.min_temporary_ttl.write_xdr(w)?;
            self.min_persistent_ttl.write_xdr(w)?;
            self.persistent_rent_rate_denominator.write_xdr(w)?;
            self.temp_rent_rate_denominator.write_xdr(w)?;
            self.max_entries_to_archive.write_xdr(w)?;
            self.live_soroban_state_size_window_sample_size
                .write_xdr(w)?;
            self.live_soroban_state_size_window_sample_period
                .write_xdr(w)?;
            self.eviction_scan_size.write_xdr(w)?;
            self.starting_eviction_scan_level.write_xdr(w)?;
            Ok(())
        })
    }
}
