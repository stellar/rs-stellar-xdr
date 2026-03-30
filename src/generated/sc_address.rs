#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScAddress is an XDR Union defined as:
///
/// ```text
/// union SCAddress switch (SCAddressType type)
/// {
/// case SC_ADDRESS_TYPE_ACCOUNT:
///     AccountID accountId;
/// case SC_ADDRESS_TYPE_CONTRACT:
///     ContractID contractId;
/// case SC_ADDRESS_TYPE_MUXED_ACCOUNT:
///     MuxedEd25519Account muxedAccount;
/// case SC_ADDRESS_TYPE_CLAIMABLE_BALANCE:
///     ClaimableBalanceID claimableBalanceId;
/// case SC_ADDRESS_TYPE_LIQUIDITY_POOL:
///     PoolID liquidityPoolId;
/// };
/// ```
///
// union with discriminant ScAddressType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum ScAddress {
    Account(
        AccountId,
    ),
    Contract(
        ContractId,
    ),
    MuxedAccount(
        MuxedEd25519Account,
    ),
    ClaimableBalance(
        ClaimableBalanceId,
    ),
    LiquidityPool(
        PoolId,
    ),
}


#[cfg(feature = "alloc")]
impl Default for ScAddress {
    fn default() -> Self {
        Self::Account(AccountId::default())
    }
}

impl ScAddress {
    const _VARIANTS: &[ScAddressType] = &[
        ScAddressType::Account,
        ScAddressType::Contract,
        ScAddressType::MuxedAccount,
        ScAddressType::ClaimableBalance,
        ScAddressType::LiquidityPool,
    ];
    pub const VARIANTS: [ScAddressType; Self::_VARIANTS.len()] = {
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
        "Contract",
        "MuxedAccount",
        "ClaimableBalance",
        "LiquidityPool",
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
            Self::Contract(_) => "Contract",
            Self::MuxedAccount(_) => "MuxedAccount",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScAddressType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => ScAddressType::Account,
            Self::Contract(_) => ScAddressType::Contract,
            Self::MuxedAccount(_) => ScAddressType::MuxedAccount,
            Self::ClaimableBalance(_) => ScAddressType::ClaimableBalance,
            Self::LiquidityPool(_) => ScAddressType::LiquidityPool,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScAddressType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScAddress {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScAddressType> for ScAddress {
    #[must_use]
    fn discriminant(&self) -> ScAddressType {
        Self::discriminant(self)
    }
}

impl Variants<ScAddressType> for ScAddress {
    fn variants() -> slice::Iter<'static, ScAddressType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScAddressType> for ScAddress {}

impl ReadXdr for ScAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScAddressType = <ScAddressType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScAddressType::Account => Self::Account(AccountId::read_xdr(r)?),
                ScAddressType::Contract => Self::Contract(ContractId::read_xdr(r)?),
                ScAddressType::MuxedAccount => Self::MuxedAccount(MuxedEd25519Account::read_xdr(r)?),
                ScAddressType::ClaimableBalance => Self::ClaimableBalance(ClaimableBalanceId::read_xdr(r)?),
                ScAddressType::LiquidityPool => Self::LiquidityPool(PoolId::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Account(v) => v.write_xdr(w)?,
                Self::Contract(v) => v.write_xdr(w)?,
                Self::MuxedAccount(v) => v.write_xdr(w)?,
                Self::ClaimableBalance(v) => v.write_xdr(w)?,
                Self::LiquidityPool(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
