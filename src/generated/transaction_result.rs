#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResult is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResult
/// {
///     int64 feeCharged; // actual fee charged for the transaction
///
///     union switch (TransactionResultCode code)
///     {
///     case txFEE_BUMP_INNER_SUCCESS:
///     case txFEE_BUMP_INNER_FAILED:
///         InnerTransactionResultPair innerResultPair;
///     case txSUCCESS:
///     case txFAILED:
///         OperationResult results<>;
///     case txTOO_EARLY:
///     case txTOO_LATE:
///     case txMISSING_OPERATION:
///     case txBAD_SEQ:
///     case txBAD_AUTH:
///     case txINSUFFICIENT_BALANCE:
///     case txNO_ACCOUNT:
///     case txINSUFFICIENT_FEE:
///     case txBAD_AUTH_EXTRA:
///     case txINTERNAL_ERROR:
///     case txNOT_SUPPORTED:
///     // case txFEE_BUMP_INNER_FAILED: handled above
///     case txBAD_SPONSORSHIP:
///     case txBAD_MIN_SEQ_AGE_OR_GAP:
///     case txMALFORMED:
///     case txSOROBAN_INVALID:
///     case txFROZEN_KEY_ACCESSED:
///         void;
///     }
///     result;
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TransactionResult {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_charged: i64,
    pub result: TransactionResultResult,
    pub ext: TransactionResultExt,
}

impl ReadXdr for TransactionResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                fee_charged: i64::read_xdr(r)?,
                result: TransactionResultResult::read_xdr(r)?,
                ext: TransactionResultExt::read_xdr(r)?,
            })
        })
    }
}

impl TransactionResult {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i64(self.fee_charged);
        self.result.const_to_xdr(w);
        self.ext.const_to_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for TransactionResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.fee_charged.write_xdr(w)?;
            self.result.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
