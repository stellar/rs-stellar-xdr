#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryData is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (LedgerEntryType type)
///     {
///     case ACCOUNT:
///         AccountEntry account;
///     case TRUSTLINE:
///         TrustLineEntry trustLine;
///     case OFFER:
///         OfferEntry offer;
///     case DATA:
///         DataEntry data;
///     case CLAIMABLE_BALANCE:
///         ClaimableBalanceEntry claimableBalance;
///     case LIQUIDITY_POOL:
///         LiquidityPoolEntry liquidityPool;
///     case CONTRACT_DATA:
///         ContractDataEntry contractData;
///     case CONTRACT_CODE:
///         ContractCodeEntry contractCode;
///     case CONFIG_SETTING:
///         ConfigSettingEntry configSetting;
///     case TTL:
///         TTLEntry ttl;
///     }
/// ```
///
// union with discriminant LedgerEntryType
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

#[cfg(feature = "alloc")]
impl Default for LedgerEntryData {
    fn default() -> Self {
        Self::Account(AccountEntry::default())
    }
}

impl LedgerEntryData {
    const _VARIANTS: &[LedgerEntryType] = &[
        LedgerEntryType::Account,
        LedgerEntryType::Trustline,
        LedgerEntryType::Offer,
        LedgerEntryType::Data,
        LedgerEntryType::ClaimableBalance,
        LedgerEntryType::LiquidityPool,
        LedgerEntryType::ContractData,
        LedgerEntryType::ContractCode,
        LedgerEntryType::ConfigSetting,
        LedgerEntryType::Ttl,
    ];
    pub const VARIANTS: [LedgerEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Account",
        "Trustline",
        "Offer",
        "Data",
        "ClaimableBalance",
        "LiquidityPool",
        "ContractData",
        "ContractCode",
        "ConfigSetting",
        "Ttl",
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
            Self::Account(_) => "Account",
            Self::Trustline(_) => "Trustline",
            Self::Offer(_) => "Offer",
            Self::Data(_) => "Data",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
            Self::ContractData(_) => "ContractData",
            Self::ContractCode(_) => "ContractCode",
            Self::ConfigSetting(_) => "ConfigSetting",
            Self::Ttl(_) => "Ttl",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [LedgerEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryData {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerEntryType> for LedgerEntryData {
    #[must_use]
    fn discriminant(&self) -> LedgerEntryType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerEntryType> for LedgerEntryData {
    fn variants() -> slice::Iter<'static, LedgerEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerEntryType> for LedgerEntryData {}

impl ReadXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerEntryType = <LedgerEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerEntryType::Account => Self::Account(AccountEntry::read_xdr(r)?),
                LedgerEntryType::Trustline => Self::Trustline(TrustLineEntry::read_xdr(r)?),
                LedgerEntryType::Offer => Self::Offer(OfferEntry::read_xdr(r)?),
                LedgerEntryType::Data => Self::Data(DataEntry::read_xdr(r)?),
                LedgerEntryType::ClaimableBalance => {
                    Self::ClaimableBalance(ClaimableBalanceEntry::read_xdr(r)?)
                }
                LedgerEntryType::LiquidityPool => {
                    Self::LiquidityPool(LiquidityPoolEntry::read_xdr(r)?)
                }
                LedgerEntryType::ContractData => {
                    Self::ContractData(ContractDataEntry::read_xdr(r)?)
                }
                LedgerEntryType::ContractCode => {
                    Self::ContractCode(ContractCodeEntry::read_xdr(r)?)
                }
                LedgerEntryType::ConfigSetting => {
                    Self::ConfigSetting(ConfigSettingEntry::read_xdr(r)?)
                }
                LedgerEntryType::Ttl => Self::Ttl(TtlEntry::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Account(v) => v.write_xdr(w)?,
                Self::Trustline(v) => v.write_xdr(w)?,
                Self::Offer(v) => v.write_xdr(w)?,
                Self::Data(v) => v.write_xdr(w)?,
                Self::ClaimableBalance(v) => v.write_xdr(w)?,
                Self::LiquidityPool(v) => v.write_xdr(w)?,
                Self::ContractData(v) => v.write_xdr(w)?,
                Self::ContractCode(v) => v.write_xdr(w)?,
                Self::ConfigSetting(v) => v.write_xdr(w)?,
                Self::Ttl(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
