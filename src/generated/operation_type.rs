#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// OperationType is an XDR Enum defined as:
///
/// ```text
/// enum OperationType
/// {
///     CREATE_ACCOUNT = 0,
///     PAYMENT = 1,
///     PATH_PAYMENT_STRICT_RECEIVE = 2,
///     MANAGE_SELL_OFFER = 3,
///     CREATE_PASSIVE_SELL_OFFER = 4,
///     SET_OPTIONS = 5,
///     CHANGE_TRUST = 6,
///     ALLOW_TRUST = 7,
///     ACCOUNT_MERGE = 8,
///     INFLATION = 9,
///     MANAGE_DATA = 10,
///     BUMP_SEQUENCE = 11,
///     MANAGE_BUY_OFFER = 12,
///     PATH_PAYMENT_STRICT_SEND = 13,
///     CREATE_CLAIMABLE_BALANCE = 14,
///     CLAIM_CLAIMABLE_BALANCE = 15,
///     BEGIN_SPONSORING_FUTURE_RESERVES = 16,
///     END_SPONSORING_FUTURE_RESERVES = 17,
///     REVOKE_SPONSORSHIP = 18,
///     CLAWBACK = 19,
///     CLAWBACK_CLAIMABLE_BALANCE = 20,
///     SET_TRUST_LINE_FLAGS = 21,
///     LIQUIDITY_POOL_DEPOSIT = 22,
///     LIQUIDITY_POOL_WITHDRAW = 23,
///     INVOKE_HOST_FUNCTION = 24,
///     EXTEND_FOOTPRINT_TTL = 25,
///     RESTORE_FOOTPRINT = 26
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum OperationType {
    #[cfg_attr(feature = "alloc", default)]
    CreateAccount = 0,
    Payment = 1,
    PathPaymentStrictReceive = 2,
    ManageSellOffer = 3,
    CreatePassiveSellOffer = 4,
    SetOptions = 5,
    ChangeTrust = 6,
    AllowTrust = 7,
    AccountMerge = 8,
    Inflation = 9,
    ManageData = 10,
    BumpSequence = 11,
    ManageBuyOffer = 12,
    PathPaymentStrictSend = 13,
    CreateClaimableBalance = 14,
    ClaimClaimableBalance = 15,
    BeginSponsoringFutureReserves = 16,
    EndSponsoringFutureReserves = 17,
    RevokeSponsorship = 18,
    Clawback = 19,
    ClawbackClaimableBalance = 20,
    SetTrustLineFlags = 21,
    LiquidityPoolDeposit = 22,
    LiquidityPoolWithdraw = 23,
    InvokeHostFunction = 24,
    ExtendFootprintTtl = 25,
    RestoreFootprint = 26,
}

impl OperationType {
    const _VARIANTS: &[OperationType] = &[
        OperationType::CreateAccount,
        OperationType::Payment,
        OperationType::PathPaymentStrictReceive,
        OperationType::ManageSellOffer,
        OperationType::CreatePassiveSellOffer,
        OperationType::SetOptions,
        OperationType::ChangeTrust,
        OperationType::AllowTrust,
        OperationType::AccountMerge,
        OperationType::Inflation,
        OperationType::ManageData,
        OperationType::BumpSequence,
        OperationType::ManageBuyOffer,
        OperationType::PathPaymentStrictSend,
        OperationType::CreateClaimableBalance,
        OperationType::ClaimClaimableBalance,
        OperationType::BeginSponsoringFutureReserves,
        OperationType::EndSponsoringFutureReserves,
        OperationType::RevokeSponsorship,
        OperationType::Clawback,
        OperationType::ClawbackClaimableBalance,
        OperationType::SetTrustLineFlags,
        OperationType::LiquidityPoolDeposit,
        OperationType::LiquidityPoolWithdraw,
        OperationType::InvokeHostFunction,
        OperationType::ExtendFootprintTtl,
        OperationType::RestoreFootprint,
    ];
    pub const VARIANTS: [OperationType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "CreateAccount",
        "Payment",
        "PathPaymentStrictReceive",
        "ManageSellOffer",
        "CreatePassiveSellOffer",
        "SetOptions",
        "ChangeTrust",
        "AllowTrust",
        "AccountMerge",
        "Inflation",
        "ManageData",
        "BumpSequence",
        "ManageBuyOffer",
        "PathPaymentStrictSend",
        "CreateClaimableBalance",
        "ClaimClaimableBalance",
        "BeginSponsoringFutureReserves",
        "EndSponsoringFutureReserves",
        "RevokeSponsorship",
        "Clawback",
        "ClawbackClaimableBalance",
        "SetTrustLineFlags",
        "LiquidityPoolDeposit",
        "LiquidityPoolWithdraw",
        "InvokeHostFunction",
        "ExtendFootprintTtl",
        "RestoreFootprint",
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
            Self::CreateAccount => "CreateAccount",
            Self::Payment => "Payment",
            Self::PathPaymentStrictReceive => "PathPaymentStrictReceive",
            Self::ManageSellOffer => "ManageSellOffer",
            Self::CreatePassiveSellOffer => "CreatePassiveSellOffer",
            Self::SetOptions => "SetOptions",
            Self::ChangeTrust => "ChangeTrust",
            Self::AllowTrust => "AllowTrust",
            Self::AccountMerge => "AccountMerge",
            Self::Inflation => "Inflation",
            Self::ManageData => "ManageData",
            Self::BumpSequence => "BumpSequence",
            Self::ManageBuyOffer => "ManageBuyOffer",
            Self::PathPaymentStrictSend => "PathPaymentStrictSend",
            Self::CreateClaimableBalance => "CreateClaimableBalance",
            Self::ClaimClaimableBalance => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship => "RevokeSponsorship",
            Self::Clawback => "Clawback",
            Self::ClawbackClaimableBalance => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction => "InvokeHostFunction",
            Self::ExtendFootprintTtl => "ExtendFootprintTtl",
            Self::RestoreFootprint => "RestoreFootprint",
        }
    }

    #[must_use]
    pub const fn variants() -> [OperationType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OperationType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<OperationType> for OperationType {
    fn variants() -> slice::Iter<'static, OperationType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for OperationType {}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OperationType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => OperationType::CreateAccount,
            1 => OperationType::Payment,
            2 => OperationType::PathPaymentStrictReceive,
            3 => OperationType::ManageSellOffer,
            4 => OperationType::CreatePassiveSellOffer,
            5 => OperationType::SetOptions,
            6 => OperationType::ChangeTrust,
            7 => OperationType::AllowTrust,
            8 => OperationType::AccountMerge,
            9 => OperationType::Inflation,
            10 => OperationType::ManageData,
            11 => OperationType::BumpSequence,
            12 => OperationType::ManageBuyOffer,
            13 => OperationType::PathPaymentStrictSend,
            14 => OperationType::CreateClaimableBalance,
            15 => OperationType::ClaimClaimableBalance,
            16 => OperationType::BeginSponsoringFutureReserves,
            17 => OperationType::EndSponsoringFutureReserves,
            18 => OperationType::RevokeSponsorship,
            19 => OperationType::Clawback,
            20 => OperationType::ClawbackClaimableBalance,
            21 => OperationType::SetTrustLineFlags,
            22 => OperationType::LiquidityPoolDeposit,
            23 => OperationType::LiquidityPoolWithdraw,
            24 => OperationType::InvokeHostFunction,
            25 => OperationType::ExtendFootprintTtl,
            26 => OperationType::RestoreFootprint,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationType> for i32 {
    #[must_use]
    fn from(e: OperationType) -> Self {
        e as Self
    }
}

impl ReadXdr for OperationType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for OperationType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
