#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationBody is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (OperationType type)
///     {
///     case CREATE_ACCOUNT:
///         CreateAccountOp createAccountOp;
///     case PAYMENT:
///         PaymentOp paymentOp;
///     case PATH_PAYMENT_STRICT_RECEIVE:
///         PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
///     case MANAGE_SELL_OFFER:
///         ManageSellOfferOp manageSellOfferOp;
///     case CREATE_PASSIVE_SELL_OFFER:
///         CreatePassiveSellOfferOp createPassiveSellOfferOp;
///     case SET_OPTIONS:
///         SetOptionsOp setOptionsOp;
///     case CHANGE_TRUST:
///         ChangeTrustOp changeTrustOp;
///     case ALLOW_TRUST:
///         AllowTrustOp allowTrustOp;
///     case ACCOUNT_MERGE:
///         MuxedAccount destination;
///     case INFLATION:
///         void;
///     case MANAGE_DATA:
///         ManageDataOp manageDataOp;
///     case BUMP_SEQUENCE:
///         BumpSequenceOp bumpSequenceOp;
///     case MANAGE_BUY_OFFER:
///         ManageBuyOfferOp manageBuyOfferOp;
///     case PATH_PAYMENT_STRICT_SEND:
///         PathPaymentStrictSendOp pathPaymentStrictSendOp;
///     case CREATE_CLAIMABLE_BALANCE:
///         CreateClaimableBalanceOp createClaimableBalanceOp;
///     case CLAIM_CLAIMABLE_BALANCE:
///         ClaimClaimableBalanceOp claimClaimableBalanceOp;
///     case BEGIN_SPONSORING_FUTURE_RESERVES:
///         BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
///     case END_SPONSORING_FUTURE_RESERVES:
///         void;
///     case REVOKE_SPONSORSHIP:
///         RevokeSponsorshipOp revokeSponsorshipOp;
///     case CLAWBACK:
///         ClawbackOp clawbackOp;
///     case CLAWBACK_CLAIMABLE_BALANCE:
///         ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
///     case SET_TRUST_LINE_FLAGS:
///         SetTrustLineFlagsOp setTrustLineFlagsOp;
///     case LIQUIDITY_POOL_DEPOSIT:
///         LiquidityPoolDepositOp liquidityPoolDepositOp;
///     case LIQUIDITY_POOL_WITHDRAW:
///         LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
///     case INVOKE_HOST_FUNCTION:
///         InvokeHostFunctionOp invokeHostFunctionOp;
///     case EXTEND_FOOTPRINT_TTL:
///         ExtendFootprintTTLOp extendFootprintTTLOp;
///     case RESTORE_FOOTPRINT:
///         RestoreFootprintOp restoreFootprintOp;
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
pub enum OperationBody {
    CreateAccount(CreateAccountOp),
    Payment(PaymentOp),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOp),
    ManageSellOffer(ManageSellOfferOp),
    CreatePassiveSellOffer(CreatePassiveSellOfferOp),
    SetOptions(SetOptionsOp),
    ChangeTrust(ChangeTrustOp),
    AllowTrust(AllowTrustOp),
    AccountMerge(MuxedAccount),
    Inflation,
    ManageData(ManageDataOp),
    BumpSequence(BumpSequenceOp),
    ManageBuyOffer(ManageBuyOfferOp),
    PathPaymentStrictSend(PathPaymentStrictSendOp),
    CreateClaimableBalance(CreateClaimableBalanceOp),
    ClaimClaimableBalance(ClaimClaimableBalanceOp),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp),
    EndSponsoringFutureReserves,
    RevokeSponsorship(RevokeSponsorshipOp),
    Clawback(ClawbackOp),
    ClawbackClaimableBalance(ClawbackClaimableBalanceOp),
    SetTrustLineFlags(SetTrustLineFlagsOp),
    LiquidityPoolDeposit(LiquidityPoolDepositOp),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawOp),
    InvokeHostFunction(InvokeHostFunctionOp),
    ExtendFootprintTtl(ExtendFootprintTtlOp),
    RestoreFootprint(RestoreFootprintOp),
}

#[cfg(feature = "alloc")]
impl Default for OperationBody {
    fn default() -> Self {
        Self::CreateAccount(CreateAccountOp::default())
    }
}

impl OperationBody {
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
            Self::Inflation => "Inflation",
            Self::ManageData(_) => "ManageData",
            Self::BumpSequence(_) => "BumpSequence",
            Self::ManageBuyOffer(_) => "ManageBuyOffer",
            Self::PathPaymentStrictSend(_) => "PathPaymentStrictSend",
            Self::CreateClaimableBalance(_) => "CreateClaimableBalance",
            Self::ClaimClaimableBalance(_) => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves(_) => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves => "EndSponsoringFutureReserves",
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
            Self::Inflation => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves => OperationType::EndSponsoringFutureReserves,
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

impl Name for OperationBody {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<OperationType> for OperationBody {
    #[must_use]
    fn discriminant(&self) -> OperationType {
        Self::discriminant(self)
    }
}

impl Variants<OperationType> for OperationBody {
    fn variants() -> slice::Iter<'static, OperationType> {
        Self::VARIANTS.iter()
    }
}

impl Union<OperationType> for OperationBody {}

impl ReadXdr for OperationBody {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: OperationType = <OperationType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                OperationType::CreateAccount => Self::CreateAccount(CreateAccountOp::read_xdr(r)?),
                OperationType::Payment => Self::Payment(PaymentOp::read_xdr(r)?),
                OperationType::PathPaymentStrictReceive => {
                    Self::PathPaymentStrictReceive(PathPaymentStrictReceiveOp::read_xdr(r)?)
                }
                OperationType::ManageSellOffer => {
                    Self::ManageSellOffer(ManageSellOfferOp::read_xdr(r)?)
                }
                OperationType::CreatePassiveSellOffer => {
                    Self::CreatePassiveSellOffer(CreatePassiveSellOfferOp::read_xdr(r)?)
                }
                OperationType::SetOptions => Self::SetOptions(SetOptionsOp::read_xdr(r)?),
                OperationType::ChangeTrust => Self::ChangeTrust(ChangeTrustOp::read_xdr(r)?),
                OperationType::AllowTrust => Self::AllowTrust(AllowTrustOp::read_xdr(r)?),
                OperationType::AccountMerge => Self::AccountMerge(MuxedAccount::read_xdr(r)?),
                OperationType::Inflation => Self::Inflation,
                OperationType::ManageData => Self::ManageData(ManageDataOp::read_xdr(r)?),
                OperationType::BumpSequence => Self::BumpSequence(BumpSequenceOp::read_xdr(r)?),
                OperationType::ManageBuyOffer => {
                    Self::ManageBuyOffer(ManageBuyOfferOp::read_xdr(r)?)
                }
                OperationType::PathPaymentStrictSend => {
                    Self::PathPaymentStrictSend(PathPaymentStrictSendOp::read_xdr(r)?)
                }
                OperationType::CreateClaimableBalance => {
                    Self::CreateClaimableBalance(CreateClaimableBalanceOp::read_xdr(r)?)
                }
                OperationType::ClaimClaimableBalance => {
                    Self::ClaimClaimableBalance(ClaimClaimableBalanceOp::read_xdr(r)?)
                }
                OperationType::BeginSponsoringFutureReserves => {
                    Self::BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp::read_xdr(
                        r,
                    )?)
                }
                OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves,
                OperationType::RevokeSponsorship => {
                    Self::RevokeSponsorship(RevokeSponsorshipOp::read_xdr(r)?)
                }
                OperationType::Clawback => Self::Clawback(ClawbackOp::read_xdr(r)?),
                OperationType::ClawbackClaimableBalance => {
                    Self::ClawbackClaimableBalance(ClawbackClaimableBalanceOp::read_xdr(r)?)
                }
                OperationType::SetTrustLineFlags => {
                    Self::SetTrustLineFlags(SetTrustLineFlagsOp::read_xdr(r)?)
                }
                OperationType::LiquidityPoolDeposit => {
                    Self::LiquidityPoolDeposit(LiquidityPoolDepositOp::read_xdr(r)?)
                }
                OperationType::LiquidityPoolWithdraw => {
                    Self::LiquidityPoolWithdraw(LiquidityPoolWithdrawOp::read_xdr(r)?)
                }
                OperationType::InvokeHostFunction => {
                    Self::InvokeHostFunction(InvokeHostFunctionOp::read_xdr(r)?)
                }
                OperationType::ExtendFootprintTtl => {
                    Self::ExtendFootprintTtl(ExtendFootprintTtlOp::read_xdr(r)?)
                }
                OperationType::RestoreFootprint => {
                    Self::RestoreFootprint(RestoreFootprintOp::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for OperationBody {
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
                Self::Inflation => ().write_xdr(w)?,
                Self::ManageData(v) => v.write_xdr(w)?,
                Self::BumpSequence(v) => v.write_xdr(w)?,
                Self::ManageBuyOffer(v) => v.write_xdr(w)?,
                Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
                Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
                Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
                Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
                Self::EndSponsoringFutureReserves => ().write_xdr(w)?,
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
