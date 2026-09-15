#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingEntry is a borrowing equivalent of [`ConfigSettingEntry`](super::super::ConfigSettingEntry)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum ConfigSettingEntry {
    ContractMaxSizeBytes(u32),
    ContractComputeV0(ConfigSettingContractComputeV0),
    ContractLedgerCostV0(ConfigSettingContractLedgerCostV0),
    ContractHistoricalDataV0(ConfigSettingContractHistoricalDataV0),
    ContractEventsV0(ConfigSettingContractEventsV0),
    ContractBandwidthV0(ConfigSettingContractBandwidthV0),
    ContractCostParamsCpuInstructions(ContractCostParams),
    ContractCostParamsMemoryBytes(ContractCostParams),
    ContractDataKeySizeBytes(u32),
    ContractDataEntrySizeBytes(u32),
    StateArchival(StateArchivalSettings),
    ContractExecutionLanes(ConfigSettingContractExecutionLanesV0),
    LiveSorobanStateSizeWindow(VecM<u64>),
    EvictionIterator(EvictionIterator),
    ContractParallelComputeV0(ConfigSettingContractParallelComputeV0),
    ContractLedgerCostExtV0(ConfigSettingContractLedgerCostExtV0),
    ScpTiming(ConfigSettingScpTiming),
    FrozenLedgerKeys(FrozenLedgerKeys),
    FrozenLedgerKeysDelta(FrozenLedgerKeysDelta),
    FreezeBypassTxs(FreezeBypassTxs),
    FreezeBypassTxsDelta(FreezeBypassTxsDelta),
}

impl ConfigSettingEntry {
    #[must_use]
    pub const fn discriminant(&self) -> ConfigSettingId {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ContractMaxSizeBytes(_) => ConfigSettingId::ContractMaxSizeBytes,
            Self::ContractComputeV0(_) => ConfigSettingId::ContractComputeV0,
            Self::ContractLedgerCostV0(_) => ConfigSettingId::ContractLedgerCostV0,
            Self::ContractHistoricalDataV0(_) => ConfigSettingId::ContractHistoricalDataV0,
            Self::ContractEventsV0(_) => ConfigSettingId::ContractEventsV0,
            Self::ContractBandwidthV0(_) => ConfigSettingId::ContractBandwidthV0,
            Self::ContractCostParamsCpuInstructions(_) => {
                ConfigSettingId::ContractCostParamsCpuInstructions
            }
            Self::ContractCostParamsMemoryBytes(_) => {
                ConfigSettingId::ContractCostParamsMemoryBytes
            }
            Self::ContractDataKeySizeBytes(_) => ConfigSettingId::ContractDataKeySizeBytes,
            Self::ContractDataEntrySizeBytes(_) => ConfigSettingId::ContractDataEntrySizeBytes,
            Self::StateArchival(_) => ConfigSettingId::StateArchival,
            Self::ContractExecutionLanes(_) => ConfigSettingId::ContractExecutionLanes,
            Self::LiveSorobanStateSizeWindow(_) => ConfigSettingId::LiveSorobanStateSizeWindow,
            Self::EvictionIterator(_) => ConfigSettingId::EvictionIterator,
            Self::ContractParallelComputeV0(_) => ConfigSettingId::ContractParallelComputeV0,
            Self::ContractLedgerCostExtV0(_) => ConfigSettingId::ContractLedgerCostExtV0,
            Self::ScpTiming(_) => ConfigSettingId::ScpTiming,
            Self::FrozenLedgerKeys(_) => ConfigSettingId::FrozenLedgerKeys,
            Self::FrozenLedgerKeysDelta(_) => ConfigSettingId::FrozenLedgerKeysDelta,
            Self::FreezeBypassTxs(_) => ConfigSettingId::FreezeBypassTxs,
            Self::FreezeBypassTxsDelta(_) => ConfigSettingId::FreezeBypassTxsDelta,
        }
    }
}

impl ConfigSettingEntry {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_config_setting_entry(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_config_setting_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ConfigSettingEntry`], mirroring `<ConfigSettingEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_config_setting_entry(&mut self, v: &ConfigSettingEntry) {
        let d = v.discriminant();
        self.write_type_config_setting_id(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ConfigSettingEntry::ContractMaxSizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntry::ContractComputeV0(value) => {
                self.write_type_config_setting_contract_compute_v0(value);
            }
            ConfigSettingEntry::ContractLedgerCostV0(value) => {
                self.write_type_config_setting_contract_ledger_cost_v0(value);
            }
            ConfigSettingEntry::ContractHistoricalDataV0(value) => {
                self.write_type_config_setting_contract_historical_data_v0(value);
            }
            ConfigSettingEntry::ContractEventsV0(value) => {
                self.write_type_config_setting_contract_events_v0(value);
            }
            ConfigSettingEntry::ContractBandwidthV0(value) => {
                self.write_type_config_setting_contract_bandwidth_v0(value);
            }
            ConfigSettingEntry::ContractCostParamsCpuInstructions(value) => {
                self.write_type_contract_cost_params(value);
            }
            ConfigSettingEntry::ContractCostParamsMemoryBytes(value) => {
                self.write_type_contract_cost_params(value);
            }
            ConfigSettingEntry::ContractDataKeySizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntry::ContractDataEntrySizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntry::StateArchival(value) => {
                self.write_type_state_archival_settings(value);
            }
            ConfigSettingEntry::ContractExecutionLanes(value) => {
                self.write_type_config_setting_contract_execution_lanes_v0(value);
            }
            ConfigSettingEntry::LiveSorobanStateSizeWindow(value) => {
                self.write_vec_u64(value);
            }
            ConfigSettingEntry::EvictionIterator(value) => {
                self.write_type_eviction_iterator(value);
            }
            ConfigSettingEntry::ContractParallelComputeV0(value) => {
                self.write_type_config_setting_contract_parallel_compute_v0(value);
            }
            ConfigSettingEntry::ContractLedgerCostExtV0(value) => {
                self.write_type_config_setting_contract_ledger_cost_ext_v0(value);
            }
            ConfigSettingEntry::ScpTiming(value) => {
                self.write_type_config_setting_scp_timing(value);
            }
            ConfigSettingEntry::FrozenLedgerKeys(value) => {
                self.write_type_frozen_ledger_keys(value);
            }
            ConfigSettingEntry::FrozenLedgerKeysDelta(value) => {
                self.write_type_frozen_ledger_keys_delta(value);
            }
            ConfigSettingEntry::FreezeBypassTxs(value) => {
                self.write_type_freeze_bypass_txs(value);
            }
            ConfigSettingEntry::FreezeBypassTxsDelta(value) => {
                self.write_type_freeze_bypass_txs_delta(value);
            }
        }
    }

    /// Serializes a variable-length array of [`ConfigSettingEntry`], mirroring `<VecM<ConfigSettingEntry, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_config_setting_entry<const MAX: u32>(
        &mut self,
        v: &VecM<ConfigSettingEntry, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_config_setting_entry(&s[i]);
            i += 1;
        }
    }
}
