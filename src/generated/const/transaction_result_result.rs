#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultResult is a borrowing equivalent of [`TransactionResultResult`](super::super::TransactionResultResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionResultResult {
    TxFeeBumpInnerSuccess(InnerTransactionResultPair),
    TxFeeBumpInnerFailed(InnerTransactionResultPair),
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

impl TransactionResultResult {
    #[must_use]
    pub const fn discriminant(&self) -> TransactionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxFeeBumpInnerSuccess(_) => TransactionResultCode::TxFeeBumpInnerSuccess,
            Self::TxFeeBumpInnerFailed(_) => TransactionResultCode::TxFeeBumpInnerFailed,
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

impl TransactionResultResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_result_result(self);
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
        w.write_type_transaction_result_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`TransactionResultResult`], mirroring `<TransactionResultResult as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_result_result(&mut self, v: &TransactionResultResult) {
        let d = v.discriminant();
        self.write_type_transaction_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            TransactionResultResult::TxFeeBumpInnerSuccess(value) => {
                self.write_type_inner_transaction_result_pair(value);
            }
            TransactionResultResult::TxFeeBumpInnerFailed(value) => {
                self.write_type_inner_transaction_result_pair(value);
            }
            TransactionResultResult::TxSuccess(value) => {
                self.write_type_vec_operation_result(value);
            }
            TransactionResultResult::TxFailed(value) => {
                self.write_type_vec_operation_result(value);
            }
            TransactionResultResult::TxTooEarly => {}
            TransactionResultResult::TxTooLate => {}
            TransactionResultResult::TxMissingOperation => {}
            TransactionResultResult::TxBadSeq => {}
            TransactionResultResult::TxBadAuth => {}
            TransactionResultResult::TxInsufficientBalance => {}
            TransactionResultResult::TxNoAccount => {}
            TransactionResultResult::TxInsufficientFee => {}
            TransactionResultResult::TxBadAuthExtra => {}
            TransactionResultResult::TxInternalError => {}
            TransactionResultResult::TxNotSupported => {}
            TransactionResultResult::TxBadSponsorship => {}
            TransactionResultResult::TxBadMinSeqAgeOrGap => {}
            TransactionResultResult::TxMalformed => {}
            TransactionResultResult::TxSorobanInvalid => {}
            TransactionResultResult::TxFrozenKeyAccessed => {}
        }
    }
}
