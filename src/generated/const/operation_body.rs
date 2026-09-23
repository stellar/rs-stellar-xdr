#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationBody is a borrowing equivalent of [`OperationBody`](super::super::OperationBody)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
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

impl OperationBody {
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
}

impl OperationBody {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_operation_body(self);
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
        w.write_type_operation_body(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`OperationBody`], mirroring `<OperationBody as WriteXdr>::write_xdr`.
    pub const fn write_type_operation_body(&mut self, v: &OperationBody) {
        let d = v.discriminant();
        self.write_type_operation_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            OperationBody::CreateAccount(value) => {
                self.write_type_create_account_op(value);
            }
            OperationBody::Payment(value) => {
                self.write_type_payment_op(value);
            }
            OperationBody::PathPaymentStrictReceive(value) => {
                self.write_type_path_payment_strict_receive_op(value);
            }
            OperationBody::ManageSellOffer(value) => {
                self.write_type_manage_sell_offer_op(value);
            }
            OperationBody::CreatePassiveSellOffer(value) => {
                self.write_type_create_passive_sell_offer_op(value);
            }
            OperationBody::SetOptions(value) => {
                self.write_type_set_options_op(value);
            }
            OperationBody::ChangeTrust(value) => {
                self.write_type_change_trust_op(value);
            }
            OperationBody::AllowTrust(value) => {
                self.write_type_allow_trust_op(value);
            }
            OperationBody::AccountMerge(value) => {
                self.write_type_muxed_account(value);
            }
            OperationBody::Inflation => {}
            OperationBody::ManageData(value) => {
                self.write_type_manage_data_op(value);
            }
            OperationBody::BumpSequence(value) => {
                self.write_type_bump_sequence_op(value);
            }
            OperationBody::ManageBuyOffer(value) => {
                self.write_type_manage_buy_offer_op(value);
            }
            OperationBody::PathPaymentStrictSend(value) => {
                self.write_type_path_payment_strict_send_op(value);
            }
            OperationBody::CreateClaimableBalance(value) => {
                self.write_type_create_claimable_balance_op(value);
            }
            OperationBody::ClaimClaimableBalance(value) => {
                self.write_type_claim_claimable_balance_op(value);
            }
            OperationBody::BeginSponsoringFutureReserves(value) => {
                self.write_type_begin_sponsoring_future_reserves_op(value);
            }
            OperationBody::EndSponsoringFutureReserves => {}
            OperationBody::RevokeSponsorship(value) => {
                self.write_type_revoke_sponsorship_op(value);
            }
            OperationBody::Clawback(value) => {
                self.write_type_clawback_op(value);
            }
            OperationBody::ClawbackClaimableBalance(value) => {
                self.write_type_clawback_claimable_balance_op(value);
            }
            OperationBody::SetTrustLineFlags(value) => {
                self.write_type_set_trust_line_flags_op(value);
            }
            OperationBody::LiquidityPoolDeposit(value) => {
                self.write_type_liquidity_pool_deposit_op(value);
            }
            OperationBody::LiquidityPoolWithdraw(value) => {
                self.write_type_liquidity_pool_withdraw_op(value);
            }
            OperationBody::InvokeHostFunction(value) => {
                self.write_type_invoke_host_function_op(value);
            }
            OperationBody::ExtendFootprintTtl(value) => {
                self.write_type_extend_footprint_ttl_op(value);
            }
            OperationBody::RestoreFootprint(value) => {
                self.write_type_restore_footprint_op(value);
            }
        }
    }
}
