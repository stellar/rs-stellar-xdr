use super::{
    AccountEntry, ClaimableBalanceEntry, ConfigSettingEntry, ContractCodeEntry, ContractDataEntry,
    DataEntry, LedgerEntry, LedgerEntryData, LedgerKey, LedgerKeyAccount,
    LedgerKeyClaimableBalance, LedgerKeyConfigSetting, LedgerKeyContractCode,
    LedgerKeyContractData, LedgerKeyData, LedgerKeyLiquidityPool, LedgerKeyOffer,
    LedgerKeyTrustLine, LedgerKeyTtl, LiquidityPoolEntry, OfferEntry, TrustLineEntry, TtlEntry,
};

impl LedgerEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKey {
        self.data.to_key()
    }
}

impl LedgerEntryData {
    #[must_use]
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
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyTtl {
        LedgerKeyTtl {
            key_hash: self.key_hash.clone(),
        }
    }
}

impl AccountEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyAccount {
        LedgerKeyAccount {
            account_id: self.account_id.clone(),
        }
    }
}

impl TrustLineEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyTrustLine {
        LedgerKeyTrustLine {
            account_id: self.account_id.clone(),
            asset: self.asset.clone(),
        }
    }
}

impl OfferEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyOffer {
        LedgerKeyOffer {
            seller_id: self.seller_id.clone(),
            offer_id: self.offer_id,
        }
    }
}

impl DataEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyData {
        LedgerKeyData {
            account_id: self.account_id.clone(),
            data_name: self.data_name.clone(),
        }
    }
}

impl ClaimableBalanceEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyClaimableBalance {
        LedgerKeyClaimableBalance {
            balance_id: self.balance_id.clone(),
        }
    }
}

impl LiquidityPoolEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyLiquidityPool {
        LedgerKeyLiquidityPool {
            liquidity_pool_id: self.liquidity_pool_id.clone(),
        }
    }
}

impl ContractDataEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyContractData {
        LedgerKeyContractData {
            contract: self.contract.clone(),
            key: self.key.clone(),
            durability: self.durability,
        }
    }
}

impl ContractCodeEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyContractCode {
        LedgerKeyContractCode {
            hash: self.hash.clone(),
        }
    }
}

impl ConfigSettingEntry {
    #[must_use]
    pub fn to_key(&self) -> LedgerKeyConfigSetting {
        LedgerKeyConfigSetting {
            config_setting_id: self.discriminant(),
        }
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
