#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InnerTransactionResultResult is a borrowing equivalent of [`InnerTransactionResultResult`](super::super::InnerTransactionResultResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum InnerTransactionResultResult {
    TxSuccess(VecM<OperationResult>),
    TxFailed(VecM<OperationResult>),
    TxTooEarly,
    TxTooLate,
    TxMissingOperation,
    TxBadSeq,
    TxBadAuth,
    TxInsufficientBalance,
    TxNoAccount,
    TxInsufficientFee,
    TxBadAuthExtra,
    TxInternalError,
    TxNotSupported,
    TxBadSponsorship,
    TxBadMinSeqAgeOrGap,
    TxMalformed,
    TxSorobanInvalid,
    TxFrozenKeyAccessed,
}

impl InnerTransactionResultResult {
    #[must_use]
    pub const fn discriminant(&self) -> TransactionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
            Self::TxTooEarly => TransactionResultCode::TxTooEarly,
            Self::TxTooLate => TransactionResultCode::TxTooLate,
            Self::TxMissingOperation => TransactionResultCode::TxMissingOperation,
            Self::TxBadSeq => TransactionResultCode::TxBadSeq,
            Self::TxBadAuth => TransactionResultCode::TxBadAuth,
            Self::TxInsufficientBalance => TransactionResultCode::TxInsufficientBalance,
            Self::TxNoAccount => TransactionResultCode::TxNoAccount,
            Self::TxInsufficientFee => TransactionResultCode::TxInsufficientFee,
            Self::TxBadAuthExtra => TransactionResultCode::TxBadAuthExtra,
            Self::TxInternalError => TransactionResultCode::TxInternalError,
            Self::TxNotSupported => TransactionResultCode::TxNotSupported,
            Self::TxBadSponsorship => TransactionResultCode::TxBadSponsorship,
            Self::TxBadMinSeqAgeOrGap => TransactionResultCode::TxBadMinSeqAgeOrGap,
            Self::TxMalformed => TransactionResultCode::TxMalformed,
            Self::TxSorobanInvalid => TransactionResultCode::TxSorobanInvalid,
            Self::TxFrozenKeyAccessed => TransactionResultCode::TxFrozenKeyAccessed,
        }
    }
}

impl InnerTransactionResultResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_inner_transaction_result_result(self);
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
        w.write_type_inner_transaction_result_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`InnerTransactionResultResult`], mirroring `<InnerTransactionResultResult as WriteXdr>::write_xdr`.
    pub const fn write_type_inner_transaction_result_result(
        &mut self,
        v: &InnerTransactionResultResult,
    ) {
        let d = v.discriminant();
        self.write_type_transaction_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            InnerTransactionResultResult::TxSuccess(value) => {
                self.write_type_vec_operation_result(value);
            }
            InnerTransactionResultResult::TxFailed(value) => {
                self.write_type_vec_operation_result(value);
            }
            InnerTransactionResultResult::TxTooEarly => {}
            InnerTransactionResultResult::TxTooLate => {}
            InnerTransactionResultResult::TxMissingOperation => {}
            InnerTransactionResultResult::TxBadSeq => {}
            InnerTransactionResultResult::TxBadAuth => {}
            InnerTransactionResultResult::TxInsufficientBalance => {}
            InnerTransactionResultResult::TxNoAccount => {}
            InnerTransactionResultResult::TxInsufficientFee => {}
            InnerTransactionResultResult::TxBadAuthExtra => {}
            InnerTransactionResultResult::TxInternalError => {}
            InnerTransactionResultResult::TxNotSupported => {}
            InnerTransactionResultResult::TxBadSponsorship => {}
            InnerTransactionResultResult::TxBadMinSeqAgeOrGap => {}
            InnerTransactionResultResult::TxMalformed => {}
            InnerTransactionResultResult::TxSorobanInvalid => {}
            InnerTransactionResultResult::TxFrozenKeyAccessed => {}
        }
    }
}
