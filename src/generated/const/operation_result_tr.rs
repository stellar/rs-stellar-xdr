#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationResultTr is a borrowing equivalent of [`OperationResultTr`](super::super::OperationResultTr)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl OperationResultTr {
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
}

impl OperationResultTr {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_operation_result_tr(self);
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
        w.write_type_operation_result_tr(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`OperationResultTr`], mirroring `<OperationResultTr as WriteXdr>::write_xdr`.
    pub const fn write_type_operation_result_tr(&mut self, v: &OperationResultTr) {
        let d = v.discriminant();
        self.write_type_operation_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            OperationResultTr::CreateAccount(value) => {
                self.write_type_create_account_result(value);
            }
            OperationResultTr::Payment(value) => {
                self.write_type_payment_result(value);
            }
            OperationResultTr::PathPaymentStrictReceive(value) => {
                self.write_type_path_payment_strict_receive_result(value);
            }
            OperationResultTr::ManageSellOffer(value) => {
                self.write_type_manage_sell_offer_result(value);
            }
            OperationResultTr::CreatePassiveSellOffer(value) => {
                self.write_type_manage_sell_offer_result(value);
            }
            OperationResultTr::SetOptions(value) => {
                self.write_type_set_options_result(value);
            }
            OperationResultTr::ChangeTrust(value) => {
                self.write_type_change_trust_result(value);
            }
            OperationResultTr::AllowTrust(value) => {
                self.write_type_allow_trust_result(value);
            }
            OperationResultTr::AccountMerge(value) => {
                self.write_type_account_merge_result(value);
            }
            OperationResultTr::Inflation(value) => {
                self.write_type_inflation_result(value);
            }
            OperationResultTr::ManageData(value) => {
                self.write_type_manage_data_result(value);
            }
            OperationResultTr::BumpSequence(value) => {
                self.write_type_bump_sequence_result(value);
            }
            OperationResultTr::ManageBuyOffer(value) => {
                self.write_type_manage_buy_offer_result(value);
            }
            OperationResultTr::PathPaymentStrictSend(value) => {
                self.write_type_path_payment_strict_send_result(value);
            }
            OperationResultTr::CreateClaimableBalance(value) => {
                self.write_type_create_claimable_balance_result(value);
            }
            OperationResultTr::ClaimClaimableBalance(value) => {
                self.write_type_claim_claimable_balance_result(value);
            }
            OperationResultTr::BeginSponsoringFutureReserves(value) => {
                self.write_type_begin_sponsoring_future_reserves_result(value);
            }
            OperationResultTr::EndSponsoringFutureReserves(value) => {
                self.write_type_end_sponsoring_future_reserves_result(value);
            }
            OperationResultTr::RevokeSponsorship(value) => {
                self.write_type_revoke_sponsorship_result(value);
            }
            OperationResultTr::Clawback(value) => {
                self.write_type_clawback_result(value);
            }
            OperationResultTr::ClawbackClaimableBalance(value) => {
                self.write_type_clawback_claimable_balance_result(value);
            }
            OperationResultTr::SetTrustLineFlags(value) => {
                self.write_type_set_trust_line_flags_result(value);
            }
            OperationResultTr::LiquidityPoolDeposit(value) => {
                self.write_type_liquidity_pool_deposit_result(value);
            }
            OperationResultTr::LiquidityPoolWithdraw(value) => {
                self.write_type_liquidity_pool_withdraw_result(value);
            }
            OperationResultTr::InvokeHostFunction(value) => {
                self.write_type_invoke_host_function_result(value);
            }
            OperationResultTr::ExtendFootprintTtl(value) => {
                self.write_type_extend_footprint_ttl_result(value);
            }
            OperationResultTr::RestoreFootprint(value) => {
                self.write_type_restore_footprint_result(value);
            }
        }
    }
}
