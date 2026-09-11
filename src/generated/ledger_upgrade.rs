#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerUpgrade is an XDR Union defined as:
///
/// ```text
/// union LedgerUpgrade switch (LedgerUpgradeType type)
/// {
/// case LEDGER_UPGRADE_VERSION:
///     uint32 newLedgerVersion; // update ledgerVersion
/// case LEDGER_UPGRADE_BASE_FEE:
///     uint32 newBaseFee; // update baseFee
/// case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
///     uint32 newMaxTxSetSize; // update maxTxSetSize
/// case LEDGER_UPGRADE_BASE_RESERVE:
///     uint32 newBaseReserve; // update baseReserve
/// case LEDGER_UPGRADE_FLAGS:
///     uint32 newFlags; // update flags
/// case LEDGER_UPGRADE_CONFIG:
///     // Update arbitrary `ConfigSetting` entries identified by the key.
///     ConfigUpgradeSetKey newConfig;
/// case LEDGER_UPGRADE_MAX_SOROBAN_TX_SET_SIZE:
///     // Update ConfigSettingContractExecutionLanesV0.ledgerMaxTxCount without
///     // using `LEDGER_UPGRADE_CONFIG`.
///     uint32 newMaxSorobanTxSetSize;
/// };
/// ```
///
// union with discriminant LedgerUpgradeType
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
pub enum LedgerUpgrade {
    Version(u32),
    BaseFee(u32),
    MaxTxSetSize(u32),
    BaseReserve(u32),
    Flags(u32),
    Config(ConfigUpgradeSetKey),
    MaxSorobanTxSetSize(u32),
}

#[cfg(feature = "alloc")]
impl Default for LedgerUpgrade {
    fn default() -> Self {
        Self::Version(u32::default())
    }
}

impl LedgerUpgrade {
    const _VARIANTS: &[LedgerUpgradeType] = &[
        LedgerUpgradeType::Version,
        LedgerUpgradeType::BaseFee,
        LedgerUpgradeType::MaxTxSetSize,
        LedgerUpgradeType::BaseReserve,
        LedgerUpgradeType::Flags,
        LedgerUpgradeType::Config,
        LedgerUpgradeType::MaxSorobanTxSetSize,
    ];
    pub const VARIANTS: [LedgerUpgradeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Version",
        "BaseFee",
        "MaxTxSetSize",
        "BaseReserve",
        "Flags",
        "Config",
        "MaxSorobanTxSetSize",
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
            Self::Version(_) => "Version",
            Self::BaseFee(_) => "BaseFee",
            Self::MaxTxSetSize(_) => "MaxTxSetSize",
            Self::BaseReserve(_) => "BaseReserve",
            Self::Flags(_) => "Flags",
            Self::Config(_) => "Config",
            Self::MaxSorobanTxSetSize(_) => "MaxSorobanTxSetSize",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LedgerUpgradeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version(_) => LedgerUpgradeType::Version,
            Self::BaseFee(_) => LedgerUpgradeType::BaseFee,
            Self::MaxTxSetSize(_) => LedgerUpgradeType::MaxTxSetSize,
            Self::BaseReserve(_) => LedgerUpgradeType::BaseReserve,
            Self::Flags(_) => LedgerUpgradeType::Flags,
            Self::Config(_) => LedgerUpgradeType::Config,
            Self::MaxSorobanTxSetSize(_) => LedgerUpgradeType::MaxSorobanTxSetSize,
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerUpgradeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerUpgrade {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerUpgradeType> for LedgerUpgrade {
    #[must_use]
    fn discriminant(&self) -> LedgerUpgradeType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerUpgradeType> for LedgerUpgrade {
    fn variants() -> slice::Iter<'static, LedgerUpgradeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerUpgradeType> for LedgerUpgrade {}

impl ReadXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerUpgradeType = <LedgerUpgradeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerUpgradeType::Version => Self::Version(u32::read_xdr(r)?),
                LedgerUpgradeType::BaseFee => Self::BaseFee(u32::read_xdr(r)?),
                LedgerUpgradeType::MaxTxSetSize => Self::MaxTxSetSize(u32::read_xdr(r)?),
                LedgerUpgradeType::BaseReserve => Self::BaseReserve(u32::read_xdr(r)?),
                LedgerUpgradeType::Flags => Self::Flags(u32::read_xdr(r)?),
                LedgerUpgradeType::Config => Self::Config(ConfigUpgradeSetKey::read_xdr(r)?),
                LedgerUpgradeType::MaxSorobanTxSetSize => {
                    Self::MaxSorobanTxSetSize(u32::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Version(v) => v.write_xdr(w)?,
                Self::BaseFee(v) => v.write_xdr(w)?,
                Self::MaxTxSetSize(v) => v.write_xdr(w)?,
                Self::BaseReserve(v) => v.write_xdr(w)?,
                Self::Flags(v) => v.write_xdr(w)?,
                Self::Config(v) => v.write_xdr(w)?,
                Self::MaxSorobanTxSetSize(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
