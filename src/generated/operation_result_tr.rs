#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationResultTr is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (OperationType type)
///     {
///     case CREATE_ACCOUNT:
///         CreateAccountResult createAccountResult;
///     case PAYMENT:
///         PaymentResult paymentResult;
///     case PATH_PAYMENT_STRICT_RECEIVE:
///         PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
///     case MANAGE_SELL_OFFER:
///         ManageSellOfferResult manageSellOfferResult;
///     case CREATE_PASSIVE_SELL_OFFER:
///         ManageSellOfferResult createPassiveSellOfferResult;
///     case SET_OPTIONS:
///         SetOptionsResult setOptionsResult;
///     case CHANGE_TRUST:
///         ChangeTrustResult changeTrustResult;
///     case ALLOW_TRUST:
///         AllowTrustResult allowTrustResult;
///     case ACCOUNT_MERGE:
///         AccountMergeResult accountMergeResult;
///     case INFLATION:
///         InflationResult inflationResult;
///     case MANAGE_DATA:
///         ManageDataResult manageDataResult;
///     case BUMP_SEQUENCE:
///         BumpSequenceResult bumpSeqResult;
///     case MANAGE_BUY_OFFER:
///         ManageBuyOfferResult manageBuyOfferResult;
///     case PATH_PAYMENT_STRICT_SEND:
///         PathPaymentStrictSendResult pathPaymentStrictSendResult;
///     case CREATE_CLAIMABLE_BALANCE:
///         CreateClaimableBalanceResult createClaimableBalanceResult;
///     case CLAIM_CLAIMABLE_BALANCE:
///         ClaimClaimableBalanceResult claimClaimableBalanceResult;
///     case BEGIN_SPONSORING_FUTURE_RESERVES:
///         BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
///     case END_SPONSORING_FUTURE_RESERVES:
///         EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
///     case REVOKE_SPONSORSHIP:
///         RevokeSponsorshipResult revokeSponsorshipResult;
///     case CLAWBACK:
///         ClawbackResult clawbackResult;
///     case CLAWBACK_CLAIMABLE_BALANCE:
///         ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
///     case SET_TRUST_LINE_FLAGS:
///         SetTrustLineFlagsResult setTrustLineFlagsResult;
///     case LIQUIDITY_POOL_DEPOSIT:
///         LiquidityPoolDepositResult liquidityPoolDepositResult;
///     case LIQUIDITY_POOL_WITHDRAW:
///         LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
///     case INVOKE_HOST_FUNCTION:
///         InvokeHostFunctionResult invokeHostFunctionResult;
///     case EXTEND_FOOTPRINT_TTL:
///         ExtendFootprintTTLResult extendFootprintTTLResult;
///     case RESTORE_FOOTPRINT:
///         RestoreFootprintResult restoreFootprintResult;
///     }
/// ```
///
// union with discriminant OperationType
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
pub enum OperationResultTr {
    CreateAccount(CreateAccountResult),
    Payment(PaymentResult),
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    ManageSellOffer(ManageSellOfferResult),
    CreatePassiveSellOffer(ManageSellOfferResult),
    SetOptions(SetOptionsResult),
    ChangeTrust(ChangeTrustResult),
    AllowTrust(AllowTrustResult),
    AccountMerge(AccountMergeResult),
    Inflation(InflationResult),
    ManageData(ManageDataResult),
    BumpSequence(BumpSequenceResult),
    ManageBuyOffer(ManageBuyOfferResult),
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    CreateClaimableBalance(CreateClaimableBalanceResult),
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    RevokeSponsorship(RevokeSponsorshipResult),
    Clawback(ClawbackResult),
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    SetTrustLineFlags(SetTrustLineFlagsResult),
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
    InvokeHostFunction(InvokeHostFunctionResult),
    ExtendFootprintTtl(ExtendFootprintTtlResult),
    RestoreFootprint(RestoreFootprintResult),
}

#[cfg(feature = "alloc")]
impl Default for OperationResultTr {
    fn default() -> Self {
        Self::CreateAccount(CreateAccountResult::default())
    }
}

impl OperationResultTr {
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
            Self::CreateAccount(_) => "CreateAccount",
            Self::Payment(_) => "Payment",
            Self::PathPaymentStrictReceive(_) => "PathPaymentStrictReceive",
            Self::ManageSellOffer(_) => "ManageSellOffer",
            Self::CreatePassiveSellOffer(_) => "CreatePassiveSellOffer",
            Self::SetOptions(_) => "SetOptions",
            Self::ChangeTrust(_) => "ChangeTrust",
            Self::AllowTrust(_) => "AllowTrust",
            Self::AccountMerge(_) => "AccountMerge",
            Self::Inflation(_) => "Inflation",
            Self::ManageData(_) => "ManageData",
            Self::BumpSequence(_) => "BumpSequence",
            Self::ManageBuyOffer(_) => "ManageBuyOffer",
            Self::PathPaymentStrictSend(_) => "PathPaymentStrictSend",
            Self::CreateClaimableBalance(_) => "CreateClaimableBalance",
            Self::ClaimClaimableBalance(_) => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves(_) => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves(_) => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship(_) => "RevokeSponsorship",
            Self::Clawback(_) => "Clawback",
            Self::ClawbackClaimableBalance(_) => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags(_) => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit(_) => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw(_) => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction(_) => "InvokeHostFunction",
            Self::ExtendFootprintTtl(_) => "ExtendFootprintTtl",
            Self::RestoreFootprint(_) => "RestoreFootprint",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> OperationType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation(_) => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves(_) => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
            Self::InvokeHostFunction(_) => OperationType::InvokeHostFunction,
            Self::ExtendFootprintTtl(_) => OperationType::ExtendFootprintTtl,
            Self::RestoreFootprint(_) => OperationType::RestoreFootprint,
        }
    }

    #[must_use]
    pub const fn variants() -> [OperationType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OperationResultTr {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<OperationType> for OperationResultTr {
    #[must_use]
    fn discriminant(&self) -> OperationType {
        Self::discriminant(self)
    }
}

impl Variants<OperationType> for OperationResultTr {
    fn variants() -> slice::Iter<'static, OperationType> {
        Self::VARIANTS.iter()
    }
}

impl Union<OperationType> for OperationResultTr {}

impl ReadXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: OperationType = <OperationType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                OperationType::CreateAccount => {
                    Self::CreateAccount(CreateAccountResult::read_xdr(r)?)
                }
                OperationType::Payment => Self::Payment(PaymentResult::read_xdr(r)?),
                OperationType::PathPaymentStrictReceive => {
                    Self::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::read_xdr(r)?)
                }
                OperationType::ManageSellOffer => {
                    Self::ManageSellOffer(ManageSellOfferResult::read_xdr(r)?)
                }
                OperationType::CreatePassiveSellOffer => {
                    Self::CreatePassiveSellOffer(ManageSellOfferResult::read_xdr(r)?)
                }
                OperationType::SetOptions => Self::SetOptions(SetOptionsResult::read_xdr(r)?),
                OperationType::ChangeTrust => Self::ChangeTrust(ChangeTrustResult::read_xdr(r)?),
                OperationType::AllowTrust => Self::AllowTrust(AllowTrustResult::read_xdr(r)?),
                OperationType::AccountMerge => Self::AccountMerge(AccountMergeResult::read_xdr(r)?),
                OperationType::Inflation => Self::Inflation(InflationResult::read_xdr(r)?),
                OperationType::ManageData => Self::ManageData(ManageDataResult::read_xdr(r)?),
                OperationType::BumpSequence => Self::BumpSequence(BumpSequenceResult::read_xdr(r)?),
                OperationType::ManageBuyOffer => {
                    Self::ManageBuyOffer(ManageBuyOfferResult::read_xdr(r)?)
                }
                OperationType::PathPaymentStrictSend => {
                    Self::PathPaymentStrictSend(PathPaymentStrictSendResult::read_xdr(r)?)
                }
                OperationType::CreateClaimableBalance => {
                    Self::CreateClaimableBalance(CreateClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::ClaimClaimableBalance => {
                    Self::ClaimClaimableBalance(ClaimClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::BeginSponsoringFutureReserves => {
                    Self::BeginSponsoringFutureReserves(
                        BeginSponsoringFutureReservesResult::read_xdr(r)?,
                    )
                }
                OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves(
                    EndSponsoringFutureReservesResult::read_xdr(r)?,
                ),
                OperationType::RevokeSponsorship => {
                    Self::RevokeSponsorship(RevokeSponsorshipResult::read_xdr(r)?)
                }
                OperationType::Clawback => Self::Clawback(ClawbackResult::read_xdr(r)?),
                OperationType::ClawbackClaimableBalance => {
                    Self::ClawbackClaimableBalance(ClawbackClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::SetTrustLineFlags => {
                    Self::SetTrustLineFlags(SetTrustLineFlagsResult::read_xdr(r)?)
                }
                OperationType::LiquidityPoolDeposit => {
                    Self::LiquidityPoolDeposit(LiquidityPoolDepositResult::read_xdr(r)?)
                }
                OperationType::LiquidityPoolWithdraw => {
                    Self::LiquidityPoolWithdraw(LiquidityPoolWithdrawResult::read_xdr(r)?)
                }
                OperationType::InvokeHostFunction => {
                    Self::InvokeHostFunction(InvokeHostFunctionResult::read_xdr(r)?)
                }
                OperationType::ExtendFootprintTtl => {
                    Self::ExtendFootprintTtl(ExtendFootprintTtlResult::read_xdr(r)?)
                }
                OperationType::RestoreFootprint => {
                    Self::RestoreFootprint(RestoreFootprintResult::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::CreateAccount(v) => v.write_xdr(w)?,
                Self::Payment(v) => v.write_xdr(w)?,
                Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
                Self::ManageSellOffer(v) => v.write_xdr(w)?,
                Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
                Self::SetOptions(v) => v.write_xdr(w)?,
                Self::ChangeTrust(v) => v.write_xdr(w)?,
                Self::AllowTrust(v) => v.write_xdr(w)?,
                Self::AccountMerge(v) => v.write_xdr(w)?,
                Self::Inflation(v) => v.write_xdr(w)?,
                Self::ManageData(v) => v.write_xdr(w)?,
                Self::BumpSequence(v) => v.write_xdr(w)?,
                Self::ManageBuyOffer(v) => v.write_xdr(w)?,
                Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
                Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
                Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
                Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
                Self::EndSponsoringFutureReserves(v) => v.write_xdr(w)?,
                Self::RevokeSponsorship(v) => v.write_xdr(w)?,
                Self::Clawback(v) => v.write_xdr(w)?,
                Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
                Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
                Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
                Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
                Self::InvokeHostFunction(v) => v.write_xdr(w)?,
                Self::ExtendFootprintTtl(v) => v.write_xdr(w)?,
                Self::RestoreFootprint(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
