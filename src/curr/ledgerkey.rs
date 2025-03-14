use super::{
    AccountEntry, ClaimableBalanceEntry, ConfigSettingEntry, ConfigSettingId, ContractCodeEntry,
    ContractDataEntry, DataEntry, LedgerEntry, LedgerEntryData, LedgerKey, LedgerKeyAccount,
    LedgerKeyClaimableBalance, LedgerKeyConfigSetting, LedgerKeyContractCode,
    LedgerKeyContractData, LedgerKeyData, LedgerKeyLiquidityPool, LedgerKeyOffer,
    LedgerKeyTrustLine, LedgerKeyTtl, LiquidityPoolEntry, OfferEntry, TrustLineEntry, TtlEntry,
};

impl LedgerEntry {
    pub fn to_key(&self) -> LedgerKey {
        self.data.to_key()
    }
}

impl LedgerEntryData {
    pub fn to_key(&self) -> LedgerKey {
        match &self {
            LedgerEntryData::Ttl(e) => e.to_key().into(),
            LedgerEntryData::Account(e) => e.to_key().into(),
            LedgerEntryData::Trustline(e) => e.to_key().into(),
            LedgerEntryData::Offer(e) => e.to_key().into(),
            LedgerEntryData::Data(e) => e.to_key().into(),
            LedgerEntryData::ClaimableBalance(e) => e.to_key().into(),
            LedgerEntryData::LiquidityPool(e) => e.to_key().into(),
            LedgerEntryData::ContractData(e) => e.to_key().into(),
            LedgerEntryData::ContractCode(e) => e.to_key().into(),
            LedgerEntryData::ConfigSetting(e) => e.to_key().into(),
        }
    }
}

impl TtlEntry {
    pub fn to_key(&self) -> LedgerKeyTtl {
        LedgerKeyTtl {
            key_hash: self.key_hash.clone(),
        }
    }
}

impl AccountEntry {
    pub fn to_key(&self) -> LedgerKeyAccount {
        LedgerKeyAccount {
            account_id: self.account_id.clone(),
        }
    }
}

impl TrustLineEntry {
    pub fn to_key(&self) -> LedgerKeyTrustLine {
        LedgerKeyTrustLine {
            account_id: self.account_id.clone(),
            asset: self.asset.clone(),
        }
    }
}

impl OfferEntry {
    pub fn to_key(&self) -> LedgerKeyOffer {
        LedgerKeyOffer {
            seller_id: self.seller_id.clone(),
            offer_id: self.offer_id,
        }
    }
}

impl DataEntry {
    pub fn to_key(&self) -> LedgerKeyData {
        LedgerKeyData {
            account_id: self.account_id.clone(),
            data_name: self.data_name.clone(),
        }
    }
}

impl ClaimableBalanceEntry {
    pub fn to_key(&self) -> LedgerKeyClaimableBalance {
        LedgerKeyClaimableBalance {
            balance_id: self.balance_id.clone(),
        }
    }
}

impl LiquidityPoolEntry {
    pub fn to_key(&self) -> LedgerKeyLiquidityPool {
        LedgerKeyLiquidityPool {
            liquidity_pool_id: self.liquidity_pool_id.clone(),
        }
    }
}

impl ContractDataEntry {
    pub fn to_key(&self) -> LedgerKeyContractData {
        LedgerKeyContractData {
            contract: self.contract.clone(),
            key: self.key.clone(),
            durability: self.durability,
        }
    }
}

impl ContractCodeEntry {
    pub fn to_key(&self) -> LedgerKeyContractCode {
        LedgerKeyContractCode {
            hash: self.hash.clone(),
        }
    }
}

impl ConfigSettingEntry {
    pub fn to_key(&self) -> LedgerKeyConfigSetting {
        let config_setting_id = match self {
            ConfigSettingEntry::ContractMaxSizeBytes(_) => ConfigSettingId::ContractMaxSizeBytes,
            ConfigSettingEntry::ContractComputeV0(_) => ConfigSettingId::ContractComputeV0,
            ConfigSettingEntry::ContractLedgerCostV0(_) => ConfigSettingId::ContractLedgerCostV0,
            ConfigSettingEntry::ContractHistoricalDataV0(_) => {
                ConfigSettingId::ContractHistoricalDataV0
            }
            ConfigSettingEntry::ContractEventsV0(_) => ConfigSettingId::ContractEventsV0,
            ConfigSettingEntry::ContractBandwidthV0(_) => ConfigSettingId::ContractBandwidthV0,
            ConfigSettingEntry::ContractCostParamsCpuInstructions(_) => {
                ConfigSettingId::ContractCostParamsCpuInstructions
            }
            ConfigSettingEntry::ContractCostParamsMemoryBytes(_) => {
                ConfigSettingId::ContractCostParamsMemoryBytes
            }
            ConfigSettingEntry::ContractDataKeySizeBytes(_) => {
                ConfigSettingId::ContractDataKeySizeBytes
            }
            ConfigSettingEntry::ContractDataEntrySizeBytes(_) => {
                ConfigSettingId::ContractDataEntrySizeBytes
            }
            ConfigSettingEntry::ContractExecutionLanes(_) => {
                ConfigSettingId::ContractExecutionLanes
            }
            ConfigSettingEntry::BucketlistSizeWindow(_) => ConfigSettingId::BucketlistSizeWindow,
            ConfigSettingEntry::EvictionIterator(_) => ConfigSettingId::EvictionIterator,
            ConfigSettingEntry::StateArchival(_) => ConfigSettingId::StateArchival,
        };

        LedgerKeyConfigSetting { config_setting_id }
    }
}

impl From<LedgerKeyTtl> for LedgerKey {
    fn from(key: LedgerKeyTtl) -> Self {
        LedgerKey::Ttl(key)
    }
}

impl From<LedgerKeyAccount> for LedgerKey {
    fn from(key: LedgerKeyAccount) -> Self {
        LedgerKey::Account(key)
    }
}

impl From<LedgerKeyTrustLine> for LedgerKey {
    fn from(key: LedgerKeyTrustLine) -> Self {
        LedgerKey::Trustline(key)
    }
}

impl From<LedgerKeyOffer> for LedgerKey {
    fn from(key: LedgerKeyOffer) -> Self {
        LedgerKey::Offer(key)
    }
}

impl From<LedgerKeyData> for LedgerKey {
    fn from(key: LedgerKeyData) -> Self {
        LedgerKey::Data(key)
    }
}

impl From<LedgerKeyClaimableBalance> for LedgerKey {
    fn from(key: LedgerKeyClaimableBalance) -> Self {
        LedgerKey::ClaimableBalance(key)
    }
}

impl From<LedgerKeyLiquidityPool> for LedgerKey {
    fn from(key: LedgerKeyLiquidityPool) -> Self {
        LedgerKey::LiquidityPool(key)
    }
}

impl From<LedgerKeyContractData> for LedgerKey {
    fn from(key: LedgerKeyContractData) -> Self {
        LedgerKey::ContractData(key)
    }
}

impl From<LedgerKeyContractCode> for LedgerKey {
    fn from(key: LedgerKeyContractCode) -> Self {
        LedgerKey::ContractCode(key)
    }
}

impl From<LedgerKeyConfigSetting> for LedgerKey {
    fn from(key: LedgerKeyConfigSetting) -> Self {
        LedgerKey::ConfigSetting(key)
    }
}
