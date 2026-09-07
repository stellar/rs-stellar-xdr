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

/// LedgerEntryDataRef is a borrowing equivalent of [`LedgerEntryData`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryDataRef<'a> {
    Account(AccountEntryRef<'a>),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntryRef<'a>),
    ClaimableBalance(ClaimableBalanceEntryRef<'a>),
    LiquidityPool(LiquidityPoolEntry),
    ContractData(ContractDataEntryRef<'a>),
    ContractCode(ContractCodeEntryRef<'a>),
    ConfigSetting(ConfigSettingEntryRef<'a>),
    Ttl(TtlEntry),
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerEntryDataRef<'_> {
    type Owned = LedgerEntryData;
    fn into_owned(self) -> LedgerEntryData {
        #[allow(clippy::match_same_arms)]
        match self {
            LedgerEntryDataRef::Account(value) => LedgerEntryData::Account(value.into_owned()),
            LedgerEntryDataRef::Trustline(value) => LedgerEntryData::Trustline(value.into_owned()),
            LedgerEntryDataRef::Offer(value) => LedgerEntryData::Offer(value.into_owned()),
            LedgerEntryDataRef::Data(value) => LedgerEntryData::Data(value.into_owned()),
            LedgerEntryDataRef::ClaimableBalance(value) => {
                LedgerEntryData::ClaimableBalance(value.into_owned())
            }
            LedgerEntryDataRef::LiquidityPool(value) => {
                LedgerEntryData::LiquidityPool(value.into_owned())
            }
            LedgerEntryDataRef::ContractData(value) => {
                LedgerEntryData::ContractData(value.into_owned())
            }
            LedgerEntryDataRef::ContractCode(value) => {
                LedgerEntryData::ContractCode(value.into_owned())
            }
            LedgerEntryDataRef::ConfigSetting(value) => {
                LedgerEntryData::ConfigSetting(value.into_owned())
            }
            LedgerEntryDataRef::Ttl(value) => LedgerEntryData::Ttl(value.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&LedgerEntryDataRef<'_>> for LedgerEntryData {
    #[must_use]
    fn from(v: &LedgerEntryDataRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerEntryDataRef<'_>> for LedgerEntryData {
    #[must_use]
    fn from(v: LedgerEntryDataRef<'_>) -> Self {
        v.into_owned()
    }
}

impl LedgerEntryDataRef<'_> {
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

impl WriteXdr for LedgerEntryDataRef<'_> {
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

#[cfg(feature = "const")]
impl LedgerEntryDataRef<'_> {
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
        w.write_type_ledger_entry_data(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerEntryData`], mirroring `<LedgerEntryData as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_entry_data(&mut self, v: &LedgerEntryDataRef<'_>) {
        let d = v.discriminant();
        self.write_type_ledger_entry_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            LedgerEntryDataRef::Account(value) => {
                self.write_type_account_entry(value);
            }
            LedgerEntryDataRef::Trustline(value) => {
                self.write_type_trust_line_entry(value);
            }
            LedgerEntryDataRef::Offer(value) => {
                self.write_type_offer_entry(value);
            }
            LedgerEntryDataRef::Data(value) => {
                self.write_type_data_entry(value);
            }
            LedgerEntryDataRef::ClaimableBalance(value) => {
                self.write_type_claimable_balance_entry(value);
            }
            LedgerEntryDataRef::LiquidityPool(value) => {
                self.write_type_liquidity_pool_entry(value);
            }
            LedgerEntryDataRef::ContractData(value) => {
                self.write_type_contract_data_entry(value);
            }
            LedgerEntryDataRef::ContractCode(value) => {
                self.write_type_contract_code_entry(value);
            }
            LedgerEntryDataRef::ConfigSetting(value) => {
                self.write_type_config_setting_entry(value);
            }
            LedgerEntryDataRef::Ttl(value) => {
                self.write_type_ttl_entry(value);
            }
        }
    }
}
