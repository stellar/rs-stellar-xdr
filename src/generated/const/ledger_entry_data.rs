#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryData is a borrowing equivalent of [`LedgerEntryData`](super::super::LedgerEntryData)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
    ClaimableBalance(ClaimableBalanceEntry),
    LiquidityPool(LiquidityPoolEntry),
    ContractData(ContractDataEntry),
    ContractCode(ContractCodeEntry),
    ConfigSetting(ConfigSettingEntry),
    Ttl(TtlEntry),
}

impl LedgerEntryData {
    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
            Self::ContractData(_) => LedgerEntryType::ContractData,
            Self::ContractCode(_) => LedgerEntryType::ContractCode,
            Self::ConfigSetting(_) => LedgerEntryType::ConfigSetting,
            Self::Ttl(_) => LedgerEntryType::Ttl,
        }
    }
}

impl LedgerEntryData {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_entry_data(self);
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
        w.write_type_ledger_entry_data(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`LedgerEntryData`], mirroring `<LedgerEntryData as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_entry_data(&mut self, v: &LedgerEntryData) {
        let d = v.discriminant();
        self.write_type_ledger_entry_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            LedgerEntryData::Account(value) => {
                self.write_type_account_entry(value);
            }
            LedgerEntryData::Trustline(value) => {
                self.write_type_trust_line_entry(value);
            }
            LedgerEntryData::Offer(value) => {
                self.write_type_offer_entry(value);
            }
            LedgerEntryData::Data(value) => {
                self.write_type_data_entry(value);
            }
            LedgerEntryData::ClaimableBalance(value) => {
                self.write_type_claimable_balance_entry(value);
            }
            LedgerEntryData::LiquidityPool(value) => {
                self.write_type_liquidity_pool_entry(value);
            }
            LedgerEntryData::ContractData(value) => {
                self.write_type_contract_data_entry(value);
            }
            LedgerEntryData::ContractCode(value) => {
                self.write_type_contract_code_entry(value);
            }
            LedgerEntryData::ConfigSetting(value) => {
                self.write_type_config_setting_entry(value);
            }
            LedgerEntryData::Ttl(value) => {
                self.write_type_ttl_entry(value);
            }
        }
    }
}
