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
#[cfg_eval::cfg_eval]
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
