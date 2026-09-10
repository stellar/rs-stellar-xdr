#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingEntry is an XDR Union defined as:
///
/// ```text
/// union ConfigSettingEntry switch (ConfigSettingID configSettingID)
/// {
/// case CONFIG_SETTING_CONTRACT_MAX_SIZE_BYTES:
///     uint32 contractMaxSizeBytes;
/// case CONFIG_SETTING_CONTRACT_COMPUTE_V0:
///     ConfigSettingContractComputeV0 contractCompute;
/// case CONFIG_SETTING_CONTRACT_LEDGER_COST_V0:
///     ConfigSettingContractLedgerCostV0 contractLedgerCost;
/// case CONFIG_SETTING_CONTRACT_HISTORICAL_DATA_V0:
///     ConfigSettingContractHistoricalDataV0 contractHistoricalData;
/// case CONFIG_SETTING_CONTRACT_EVENTS_V0:
///     ConfigSettingContractEventsV0 contractEvents;
/// case CONFIG_SETTING_CONTRACT_BANDWIDTH_V0:
///     ConfigSettingContractBandwidthV0 contractBandwidth;
/// case CONFIG_SETTING_CONTRACT_COST_PARAMS_CPU_INSTRUCTIONS:
///     ContractCostParams contractCostParamsCpuInsns;
/// case CONFIG_SETTING_CONTRACT_COST_PARAMS_MEMORY_BYTES:
///     ContractCostParams contractCostParamsMemBytes;
/// case CONFIG_SETTING_CONTRACT_DATA_KEY_SIZE_BYTES:
///     uint32 contractDataKeySizeBytes;
/// case CONFIG_SETTING_CONTRACT_DATA_ENTRY_SIZE_BYTES:
///     uint32 contractDataEntrySizeBytes;
/// case CONFIG_SETTING_STATE_ARCHIVAL:
///     StateArchivalSettings stateArchivalSettings;
/// case CONFIG_SETTING_CONTRACT_EXECUTION_LANES:
///     ConfigSettingContractExecutionLanesV0 contractExecutionLanes;
/// case CONFIG_SETTING_LIVE_SOROBAN_STATE_SIZE_WINDOW:
///     uint64 liveSorobanStateSizeWindow<>;
/// case CONFIG_SETTING_EVICTION_ITERATOR:
///     EvictionIterator evictionIterator;
/// case CONFIG_SETTING_CONTRACT_PARALLEL_COMPUTE_V0:
///     ConfigSettingContractParallelComputeV0 contractParallelCompute;
/// case CONFIG_SETTING_CONTRACT_LEDGER_COST_EXT_V0:
///     ConfigSettingContractLedgerCostExtV0 contractLedgerCostExt;
/// case CONFIG_SETTING_SCP_TIMING:
///     ConfigSettingSCPTiming contractSCPTiming;
/// case CONFIG_SETTING_FROZEN_LEDGER_KEYS:
///     FrozenLedgerKeys frozenLedgerKeys;
/// case CONFIG_SETTING_FROZEN_LEDGER_KEYS_DELTA:
///     FrozenLedgerKeysDelta frozenLedgerKeysDelta;
/// case CONFIG_SETTING_FREEZE_BYPASS_TXS:
///     FreezeBypassTxs freezeBypassTxs;
/// case CONFIG_SETTING_FREEZE_BYPASS_TXS_DELTA:
///     FreezeBypassTxsDelta freezeBypassTxsDelta;
/// };
/// ```
///
// union with discriminant ConfigSettingId
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
    LiveSorobanStateSizeWindow(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "VecM<NumberOrString>")
        )]
        VecM<u64>,
    ),
    EvictionIterator(EvictionIterator),
    ContractParallelComputeV0(ConfigSettingContractParallelComputeV0),
    ContractLedgerCostExtV0(ConfigSettingContractLedgerCostExtV0),
    ScpTiming(ConfigSettingScpTiming),
    FrozenLedgerKeys(FrozenLedgerKeys),
    FrozenLedgerKeysDelta(FrozenLedgerKeysDelta),
    FreezeBypassTxs(FreezeBypassTxs),
    FreezeBypassTxsDelta(FreezeBypassTxsDelta),
}

#[cfg(feature = "alloc")]
impl Default for ConfigSettingEntry {
    fn default() -> Self {
        Self::ContractMaxSizeBytes(u32::default())
    }
}

impl ConfigSettingEntry {
    const _VARIANTS: &[ConfigSettingId] = &[
        ConfigSettingId::ContractMaxSizeBytes,
        ConfigSettingId::ContractComputeV0,
        ConfigSettingId::ContractLedgerCostV0,
        ConfigSettingId::ContractHistoricalDataV0,
        ConfigSettingId::ContractEventsV0,
        ConfigSettingId::ContractBandwidthV0,
        ConfigSettingId::ContractCostParamsCpuInstructions,
        ConfigSettingId::ContractCostParamsMemoryBytes,
        ConfigSettingId::ContractDataKeySizeBytes,
        ConfigSettingId::ContractDataEntrySizeBytes,
        ConfigSettingId::StateArchival,
        ConfigSettingId::ContractExecutionLanes,
        ConfigSettingId::LiveSorobanStateSizeWindow,
        ConfigSettingId::EvictionIterator,
        ConfigSettingId::ContractParallelComputeV0,
        ConfigSettingId::ContractLedgerCostExtV0,
        ConfigSettingId::ScpTiming,
        ConfigSettingId::FrozenLedgerKeys,
        ConfigSettingId::FrozenLedgerKeysDelta,
        ConfigSettingId::FreezeBypassTxs,
        ConfigSettingId::FreezeBypassTxsDelta,
    ];
    pub const VARIANTS: [ConfigSettingId; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ContractMaxSizeBytes",
        "ContractComputeV0",
        "ContractLedgerCostV0",
        "ContractHistoricalDataV0",
        "ContractEventsV0",
        "ContractBandwidthV0",
        "ContractCostParamsCpuInstructions",
        "ContractCostParamsMemoryBytes",
        "ContractDataKeySizeBytes",
        "ContractDataEntrySizeBytes",
        "StateArchival",
        "ContractExecutionLanes",
        "LiveSorobanStateSizeWindow",
        "EvictionIterator",
        "ContractParallelComputeV0",
        "ContractLedgerCostExtV0",
        "ScpTiming",
        "FrozenLedgerKeys",
        "FrozenLedgerKeysDelta",
        "FreezeBypassTxs",
        "FreezeBypassTxsDelta",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ContractMaxSizeBytes(_) => "ContractMaxSizeBytes",
            Self::ContractComputeV0(_) => "ContractComputeV0",
            Self::ContractLedgerCostV0(_) => "ContractLedgerCostV0",
            Self::ContractHistoricalDataV0(_) => "ContractHistoricalDataV0",
            Self::ContractEventsV0(_) => "ContractEventsV0",
            Self::ContractBandwidthV0(_) => "ContractBandwidthV0",
            Self::ContractCostParamsCpuInstructions(_) => "ContractCostParamsCpuInstructions",
            Self::ContractCostParamsMemoryBytes(_) => "ContractCostParamsMemoryBytes",
            Self::ContractDataKeySizeBytes(_) => "ContractDataKeySizeBytes",
            Self::ContractDataEntrySizeBytes(_) => "ContractDataEntrySizeBytes",
            Self::StateArchival(_) => "StateArchival",
            Self::ContractExecutionLanes(_) => "ContractExecutionLanes",
            Self::LiveSorobanStateSizeWindow(_) => "LiveSorobanStateSizeWindow",
            Self::EvictionIterator(_) => "EvictionIterator",
            Self::ContractParallelComputeV0(_) => "ContractParallelComputeV0",
            Self::ContractLedgerCostExtV0(_) => "ContractLedgerCostExtV0",
            Self::ScpTiming(_) => "ScpTiming",
            Self::FrozenLedgerKeys(_) => "FrozenLedgerKeys",
            Self::FrozenLedgerKeysDelta(_) => "FrozenLedgerKeysDelta",
            Self::FreezeBypassTxs(_) => "FreezeBypassTxs",
            Self::FreezeBypassTxsDelta(_) => "FreezeBypassTxsDelta",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [ConfigSettingId; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ConfigSettingEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ConfigSettingId> for ConfigSettingEntry {
    #[must_use]
    fn discriminant(&self) -> ConfigSettingId {
        Self::discriminant(self)
    }
}

impl Variants<ConfigSettingId> for ConfigSettingEntry {
    fn variants() -> slice::Iter<'static, ConfigSettingId> {
        Self::VARIANTS.iter()
    }
}

impl Union<ConfigSettingId> for ConfigSettingEntry {}

impl ReadXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ConfigSettingId = <ConfigSettingId as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ConfigSettingId::ContractMaxSizeBytes => {
                    Self::ContractMaxSizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::ContractComputeV0 => {
                    Self::ContractComputeV0(ConfigSettingContractComputeV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractLedgerCostV0 => {
                    Self::ContractLedgerCostV0(ConfigSettingContractLedgerCostV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractHistoricalDataV0 => Self::ContractHistoricalDataV0(
                    ConfigSettingContractHistoricalDataV0::read_xdr(r)?,
                ),
                ConfigSettingId::ContractEventsV0 => {
                    Self::ContractEventsV0(ConfigSettingContractEventsV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractBandwidthV0 => {
                    Self::ContractBandwidthV0(ConfigSettingContractBandwidthV0::read_xdr(r)?)
                }
                ConfigSettingId::ContractCostParamsCpuInstructions => {
                    Self::ContractCostParamsCpuInstructions(ContractCostParams::read_xdr(r)?)
                }
                ConfigSettingId::ContractCostParamsMemoryBytes => {
                    Self::ContractCostParamsMemoryBytes(ContractCostParams::read_xdr(r)?)
                }
                ConfigSettingId::ContractDataKeySizeBytes => {
                    Self::ContractDataKeySizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::ContractDataEntrySizeBytes => {
                    Self::ContractDataEntrySizeBytes(u32::read_xdr(r)?)
                }
                ConfigSettingId::StateArchival => {
                    Self::StateArchival(StateArchivalSettings::read_xdr(r)?)
                }
                ConfigSettingId::ContractExecutionLanes => Self::ContractExecutionLanes(
                    ConfigSettingContractExecutionLanesV0::read_xdr(r)?,
                ),
                ConfigSettingId::LiveSorobanStateSizeWindow => {
                    Self::LiveSorobanStateSizeWindow(VecM::<u64>::read_xdr(r)?)
                }
                ConfigSettingId::EvictionIterator => {
                    Self::EvictionIterator(EvictionIterator::read_xdr(r)?)
                }
                ConfigSettingId::ContractParallelComputeV0 => Self::ContractParallelComputeV0(
                    ConfigSettingContractParallelComputeV0::read_xdr(r)?,
                ),
                ConfigSettingId::ContractLedgerCostExtV0 => Self::ContractLedgerCostExtV0(
                    ConfigSettingContractLedgerCostExtV0::read_xdr(r)?,
                ),
                ConfigSettingId::ScpTiming => Self::ScpTiming(ConfigSettingScpTiming::read_xdr(r)?),
                ConfigSettingId::FrozenLedgerKeys => {
                    Self::FrozenLedgerKeys(FrozenLedgerKeys::read_xdr(r)?)
                }
                ConfigSettingId::FrozenLedgerKeysDelta => {
                    Self::FrozenLedgerKeysDelta(FrozenLedgerKeysDelta::read_xdr(r)?)
                }
                ConfigSettingId::FreezeBypassTxs => {
                    Self::FreezeBypassTxs(FreezeBypassTxs::read_xdr(r)?)
                }
                ConfigSettingId::FreezeBypassTxsDelta => {
                    Self::FreezeBypassTxsDelta(FreezeBypassTxsDelta::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ContractMaxSizeBytes(v) => v.write_xdr(w)?,
                Self::ContractComputeV0(v) => v.write_xdr(w)?,
                Self::ContractLedgerCostV0(v) => v.write_xdr(w)?,
                Self::ContractHistoricalDataV0(v) => v.write_xdr(w)?,
                Self::ContractEventsV0(v) => v.write_xdr(w)?,
                Self::ContractBandwidthV0(v) => v.write_xdr(w)?,
                Self::ContractCostParamsCpuInstructions(v) => v.write_xdr(w)?,
                Self::ContractCostParamsMemoryBytes(v) => v.write_xdr(w)?,
                Self::ContractDataKeySizeBytes(v) => v.write_xdr(w)?,
                Self::ContractDataEntrySizeBytes(v) => v.write_xdr(w)?,
                Self::StateArchival(v) => v.write_xdr(w)?,
                Self::ContractExecutionLanes(v) => v.write_xdr(w)?,
                Self::LiveSorobanStateSizeWindow(v) => v.write_xdr(w)?,
                Self::EvictionIterator(v) => v.write_xdr(w)?,
                Self::ContractParallelComputeV0(v) => v.write_xdr(w)?,
                Self::ContractLedgerCostExtV0(v) => v.write_xdr(w)?,
                Self::ScpTiming(v) => v.write_xdr(w)?,
                Self::FrozenLedgerKeys(v) => v.write_xdr(w)?,
                Self::FrozenLedgerKeysDelta(v) => v.write_xdr(w)?,
                Self::FreezeBypassTxs(v) => v.write_xdr(w)?,
                Self::FreezeBypassTxsDelta(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// ConfigSettingEntryConst is a borrowing equivalent of [`ConfigSettingEntry`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ConfigSettingEntryConst {
    ContractMaxSizeBytes(u32),
    ContractComputeV0(ConfigSettingContractComputeV0),
    ContractLedgerCostV0(ConfigSettingContractLedgerCostV0),
    ContractHistoricalDataV0(ConfigSettingContractHistoricalDataV0),
    ContractEventsV0(ConfigSettingContractEventsV0),
    ContractBandwidthV0(ConfigSettingContractBandwidthV0),
    ContractCostParamsCpuInstructions(ContractCostParamsConst),
    ContractCostParamsMemoryBytes(ContractCostParamsConst),
    ContractDataKeySizeBytes(u32),
    ContractDataEntrySizeBytes(u32),
    StateArchival(StateArchivalSettings),
    ContractExecutionLanes(ConfigSettingContractExecutionLanesV0),
    LiveSorobanStateSizeWindow(VecMConst<u64>),
    EvictionIterator(EvictionIterator),
    ContractParallelComputeV0(ConfigSettingContractParallelComputeV0),
    ContractLedgerCostExtV0(ConfigSettingContractLedgerCostExtV0),
    ScpTiming(ConfigSettingScpTiming),
    FrozenLedgerKeys(FrozenLedgerKeysConst),
    FrozenLedgerKeysDelta(FrozenLedgerKeysDeltaConst),
    FreezeBypassTxs(FreezeBypassTxsConst),
    FreezeBypassTxsDelta(FreezeBypassTxsDeltaConst),
}

#[cfg(feature = "const")]
impl ConfigSettingEntryConst {
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

#[cfg(feature = "const")]
impl ConfigSettingEntryConst {
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
        w.write_type_config_setting_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ConfigSettingEntry`], mirroring `<ConfigSettingEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_config_setting_entry(&mut self, v: &ConfigSettingEntryConst) {
        let d = v.discriminant();
        self.write_type_config_setting_id(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ConfigSettingEntryConst::ContractMaxSizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntryConst::ContractComputeV0(value) => {
                self.write_type_config_setting_contract_compute_v0(value);
            }
            ConfigSettingEntryConst::ContractLedgerCostV0(value) => {
                self.write_type_config_setting_contract_ledger_cost_v0(value);
            }
            ConfigSettingEntryConst::ContractHistoricalDataV0(value) => {
                self.write_type_config_setting_contract_historical_data_v0(value);
            }
            ConfigSettingEntryConst::ContractEventsV0(value) => {
                self.write_type_config_setting_contract_events_v0(value);
            }
            ConfigSettingEntryConst::ContractBandwidthV0(value) => {
                self.write_type_config_setting_contract_bandwidth_v0(value);
            }
            ConfigSettingEntryConst::ContractCostParamsCpuInstructions(value) => {
                self.write_type_contract_cost_params(value);
            }
            ConfigSettingEntryConst::ContractCostParamsMemoryBytes(value) => {
                self.write_type_contract_cost_params(value);
            }
            ConfigSettingEntryConst::ContractDataKeySizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntryConst::ContractDataEntrySizeBytes(value) => {
                self.write_u32(*value);
            }
            ConfigSettingEntryConst::StateArchival(value) => {
                self.write_type_state_archival_settings(value);
            }
            ConfigSettingEntryConst::ContractExecutionLanes(value) => {
                self.write_type_config_setting_contract_execution_lanes_v0(value);
            }
            ConfigSettingEntryConst::LiveSorobanStateSizeWindow(value) => {
                self.write_vec_u64(value);
            }
            ConfigSettingEntryConst::EvictionIterator(value) => {
                self.write_type_eviction_iterator(value);
            }
            ConfigSettingEntryConst::ContractParallelComputeV0(value) => {
                self.write_type_config_setting_contract_parallel_compute_v0(value);
            }
            ConfigSettingEntryConst::ContractLedgerCostExtV0(value) => {
                self.write_type_config_setting_contract_ledger_cost_ext_v0(value);
            }
            ConfigSettingEntryConst::ScpTiming(value) => {
                self.write_type_config_setting_scp_timing(value);
            }
            ConfigSettingEntryConst::FrozenLedgerKeys(value) => {
                self.write_type_frozen_ledger_keys(value);
            }
            ConfigSettingEntryConst::FrozenLedgerKeysDelta(value) => {
                self.write_type_frozen_ledger_keys_delta(value);
            }
            ConfigSettingEntryConst::FreezeBypassTxs(value) => {
                self.write_type_freeze_bypass_txs(value);
            }
            ConfigSettingEntryConst::FreezeBypassTxsDelta(value) => {
                self.write_type_freeze_bypass_txs_delta(value);
            }
        }
    }

    /// Serializes a variable-length array of [`ConfigSettingEntry`], mirroring `<VecM<ConfigSettingEntry, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_config_setting_entry<const MAX: u32>(
        &mut self,
        v: &VecMConst<ConfigSettingEntryConst, MAX>,
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
