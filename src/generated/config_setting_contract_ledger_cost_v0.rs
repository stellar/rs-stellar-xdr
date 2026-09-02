#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractLedgerCostV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractLedgerCostV0
/// {
///     // Maximum number of disk entry read operations per ledger
///     uint32 ledgerMaxDiskReadEntries;
///     // Maximum number of bytes of disk reads that can be performed per ledger
///     uint32 ledgerMaxDiskReadBytes;
///     // Maximum number of ledger entry write operations per ledger
///     uint32 ledgerMaxWriteLedgerEntries;
///     // Maximum number of bytes that can be written per ledger
///     uint32 ledgerMaxWriteBytes;
///
///     // Maximum number of disk entry read operations per transaction
///     uint32 txMaxDiskReadEntries;
///     // Maximum number of bytes of disk reads that can be performed per transaction
///     uint32 txMaxDiskReadBytes;
///     // Maximum number of ledger entry write operations per transaction
///     uint32 txMaxWriteLedgerEntries;
///     // Maximum number of bytes that can be written per transaction
///     uint32 txMaxWriteBytes;
///
///     int64 feeDiskReadLedgerEntry;  // Fee per disk ledger entry read
///     int64 feeWriteLedgerEntry;     // Fee per ledger entry write
///
///     int64 feeDiskRead1KB;          // Fee for reading 1KB disk
///
///     // The following parameters determine the write fee per 1KB.
///     // Rent fee grows linearly until soroban state reaches this size
///     int64 sorobanStateTargetSizeBytes;
///     // Fee per 1KB rent when the soroban state is empty
///     int64 rentFee1KBSorobanStateSizeLow;
///     // Fee per 1KB rent when the soroban state has reached `sorobanStateTargetSizeBytes`
///     int64 rentFee1KBSorobanStateSizeHigh;
///     // Rent fee multiplier for any additional data past the first `sorobanStateTargetSizeBytes`
///     uint32 sorobanStateRentFeeGrowthFactor;
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
pub struct ConfigSettingContractLedgerCostV0 {
    pub ledger_max_disk_read_entries: u32,
    pub ledger_max_disk_read_bytes: u32,
    pub ledger_max_write_ledger_entries: u32,
    pub ledger_max_write_bytes: u32,
    pub tx_max_disk_read_entries: u32,
    pub tx_max_disk_read_bytes: u32,
    pub tx_max_write_ledger_entries: u32,
    pub tx_max_write_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_disk_read_ledger_entry: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_write_ledger_entry: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_disk_read1_kb: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub soroban_state_target_size_bytes: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee1_kb_soroban_state_size_low: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee1_kb_soroban_state_size_high: i64,
    pub soroban_state_rent_fee_growth_factor: u32,
}

impl ReadXdr for ConfigSettingContractLedgerCostV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_disk_read_entries: u32::read_xdr(r)?,
                ledger_max_disk_read_bytes: u32::read_xdr(r)?,
                ledger_max_write_ledger_entries: u32::read_xdr(r)?,
                ledger_max_write_bytes: u32::read_xdr(r)?,
                tx_max_disk_read_entries: u32::read_xdr(r)?,
                tx_max_disk_read_bytes: u32::read_xdr(r)?,
                tx_max_write_ledger_entries: u32::read_xdr(r)?,
                tx_max_write_bytes: u32::read_xdr(r)?,
                fee_disk_read_ledger_entry: i64::read_xdr(r)?,
                fee_write_ledger_entry: i64::read_xdr(r)?,
                fee_disk_read1_kb: i64::read_xdr(r)?,
                soroban_state_target_size_bytes: i64::read_xdr(r)?,
                rent_fee1_kb_soroban_state_size_low: i64::read_xdr(r)?,
                rent_fee1_kb_soroban_state_size_high: i64::read_xdr(r)?,
                soroban_state_rent_fee_growth_factor: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractLedgerCostV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_disk_read_entries.write_xdr(w)?;
            self.ledger_max_disk_read_bytes.write_xdr(w)?;
            self.ledger_max_write_ledger_entries.write_xdr(w)?;
            self.ledger_max_write_bytes.write_xdr(w)?;
            self.tx_max_disk_read_entries.write_xdr(w)?;
            self.tx_max_disk_read_bytes.write_xdr(w)?;
            self.tx_max_write_ledger_entries.write_xdr(w)?;
            self.tx_max_write_bytes.write_xdr(w)?;
            self.fee_disk_read_ledger_entry.write_xdr(w)?;
            self.fee_write_ledger_entry.write_xdr(w)?;
            self.fee_disk_read1_kb.write_xdr(w)?;
            self.soroban_state_target_size_bytes.write_xdr(w)?;
            self.rent_fee1_kb_soroban_state_size_low.write_xdr(w)?;
            self.rent_fee1_kb_soroban_state_size_high.write_xdr(w)?;
            self.soroban_state_rent_fee_growth_factor.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for ConfigSettingContractLedgerCostV0 {
    type Owned = ConfigSettingContractLedgerCostV0;
    fn into_owned(self) -> ConfigSettingContractLedgerCostV0 {
        self
    }
}

#[cfg(feature = "const")]
impl ConfigSettingContractLedgerCostV0 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_config_setting_contract_ledger_cost_v0(self);
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
        w.write_type_config_setting_contract_ledger_cost_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ConfigSettingContractLedgerCostV0`], mirroring `<ConfigSettingContractLedgerCostV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_config_setting_contract_ledger_cost_v0(
        &mut self,
        v: &ConfigSettingContractLedgerCostV0,
    ) {
        self.write_u32(v.ledger_max_disk_read_entries);
        self.write_u32(v.ledger_max_disk_read_bytes);
        self.write_u32(v.ledger_max_write_ledger_entries);
        self.write_u32(v.ledger_max_write_bytes);
        self.write_u32(v.tx_max_disk_read_entries);
        self.write_u32(v.tx_max_disk_read_bytes);
        self.write_u32(v.tx_max_write_ledger_entries);
        self.write_u32(v.tx_max_write_bytes);
        self.write_i64(v.fee_disk_read_ledger_entry);
        self.write_i64(v.fee_write_ledger_entry);
        self.write_i64(v.fee_disk_read1_kb);
        self.write_i64(v.soroban_state_target_size_bytes);
        self.write_i64(v.rent_fee1_kb_soroban_state_size_low);
        self.write_i64(v.rent_fee1_kb_soroban_state_size_high);
        self.write_u32(v.soroban_state_rent_fee_growth_factor);
    }
}
