#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultResult is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (TransactionResultCode code)
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
/// ```
///
// union with discriminant TransactionResultCode
#[cfg_eval::cfg_eval]
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
pub enum TransactionResultResult {
    TxFeeBumpInnerSuccess(
        InnerTransactionResultPair,
    ),
    TxFeeBumpInnerFailed(
        InnerTransactionResultPair,
    ),
    TxSuccess(
        VecM<OperationResult>,
    ),
    TxFailed(
        VecM<OperationResult>,
    ),
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


#[cfg(feature = "alloc")]
impl Default for TransactionResultResult {
    fn default() -> Self {
        Self::TxFeeBumpInnerSuccess(InnerTransactionResultPair::default())
    }
}

impl TransactionResultResult {
    const _VARIANTS: &[TransactionResultCode] = &[
        TransactionResultCode::TxFeeBumpInnerSuccess,
        TransactionResultCode::TxFeeBumpInnerFailed,
        TransactionResultCode::TxSuccess,
        TransactionResultCode::TxFailed,
        TransactionResultCode::TxTooEarly,
        TransactionResultCode::TxTooLate,
        TransactionResultCode::TxMissingOperation,
        TransactionResultCode::TxBadSeq,
        TransactionResultCode::TxBadAuth,
        TransactionResultCode::TxInsufficientBalance,
        TransactionResultCode::TxNoAccount,
        TransactionResultCode::TxInsufficientFee,
        TransactionResultCode::TxBadAuthExtra,
        TransactionResultCode::TxInternalError,
        TransactionResultCode::TxNotSupported,
        TransactionResultCode::TxBadSponsorship,
        TransactionResultCode::TxBadMinSeqAgeOrGap,
        TransactionResultCode::TxMalformed,
        TransactionResultCode::TxSorobanInvalid,
        TransactionResultCode::TxFrozenKeyAccessed,
    ];
    pub const VARIANTS: [TransactionResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "TxFeeBumpInnerSuccess",
        "TxFeeBumpInnerFailed",
        "TxSuccess",
        "TxFailed",
        "TxTooEarly",
        "TxTooLate",
        "TxMissingOperation",
        "TxBadSeq",
        "TxBadAuth",
        "TxInsufficientBalance",
        "TxNoAccount",
        "TxInsufficientFee",
        "TxBadAuthExtra",
        "TxInternalError",
        "TxNotSupported",
        "TxBadSponsorship",
        "TxBadMinSeqAgeOrGap",
        "TxMalformed",
        "TxSorobanInvalid",
        "TxFrozenKeyAccessed",
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
            Self::TxFeeBumpInnerSuccess(_) => "TxFeeBumpInnerSuccess",
            Self::TxFeeBumpInnerFailed(_) => "TxFeeBumpInnerFailed",
            Self::TxSuccess(_) => "TxSuccess",
            Self::TxFailed(_) => "TxFailed",
            Self::TxTooEarly => "TxTooEarly",
            Self::TxTooLate => "TxTooLate",
            Self::TxMissingOperation => "TxMissingOperation",
            Self::TxBadSeq => "TxBadSeq",
            Self::TxBadAuth => "TxBadAuth",
            Self::TxInsufficientBalance => "TxInsufficientBalance",
            Self::TxNoAccount => "TxNoAccount",
            Self::TxInsufficientFee => "TxInsufficientFee",
            Self::TxBadAuthExtra => "TxBadAuthExtra",
            Self::TxInternalError => "TxInternalError",
            Self::TxNotSupported => "TxNotSupported",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
            Self::TxSorobanInvalid => "TxSorobanInvalid",
            Self::TxFrozenKeyAccessed => "TxFrozenKeyAccessed",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [TransactionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionResultResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<TransactionResultCode> for TransactionResultResult {
    #[must_use]
    fn discriminant(&self) -> TransactionResultCode {
        Self::discriminant(self)
    }
}

impl Variants<TransactionResultCode> for TransactionResultResult {
    fn variants() -> slice::Iter<'static, TransactionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<TransactionResultCode> for TransactionResultResult {}

impl ReadXdr for TransactionResultResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: TransactionResultCode = <TransactionResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                TransactionResultCode::TxFeeBumpInnerSuccess => Self::TxFeeBumpInnerSuccess(InnerTransactionResultPair::read_xdr(r)?),
                TransactionResultCode::TxFeeBumpInnerFailed => Self::TxFeeBumpInnerFailed(InnerTransactionResultPair::read_xdr(r)?),
                TransactionResultCode::TxSuccess => Self::TxSuccess(VecM::<OperationResult>::read_xdr(r)?),
                TransactionResultCode::TxFailed => Self::TxFailed(VecM::<OperationResult>::read_xdr(r)?),
                TransactionResultCode::TxTooEarly => Self::TxTooEarly,
                TransactionResultCode::TxTooLate => Self::TxTooLate,
                TransactionResultCode::TxMissingOperation => Self::TxMissingOperation,
                TransactionResultCode::TxBadSeq => Self::TxBadSeq,
                TransactionResultCode::TxBadAuth => Self::TxBadAuth,
                TransactionResultCode::TxInsufficientBalance => Self::TxInsufficientBalance,
                TransactionResultCode::TxNoAccount => Self::TxNoAccount,
                TransactionResultCode::TxInsufficientFee => Self::TxInsufficientFee,
                TransactionResultCode::TxBadAuthExtra => Self::TxBadAuthExtra,
                TransactionResultCode::TxInternalError => Self::TxInternalError,
                TransactionResultCode::TxNotSupported => Self::TxNotSupported,
                TransactionResultCode::TxBadSponsorship => Self::TxBadSponsorship,
                TransactionResultCode::TxBadMinSeqAgeOrGap => Self::TxBadMinSeqAgeOrGap,
                TransactionResultCode::TxMalformed => Self::TxMalformed,
                TransactionResultCode::TxSorobanInvalid => Self::TxSorobanInvalid,
                TransactionResultCode::TxFrozenKeyAccessed => Self::TxFrozenKeyAccessed,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionResultResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxFeeBumpInnerSuccess(v) => v.write_xdr(w)?,
                Self::TxFeeBumpInnerFailed(v) => v.write_xdr(w)?,
                Self::TxSuccess(v) => v.write_xdr(w)?,
                Self::TxFailed(v) => v.write_xdr(w)?,
                Self::TxTooEarly => ().write_xdr(w)?,
                Self::TxTooLate => ().write_xdr(w)?,
                Self::TxMissingOperation => ().write_xdr(w)?,
                Self::TxBadSeq => ().write_xdr(w)?,
                Self::TxBadAuth => ().write_xdr(w)?,
                Self::TxInsufficientBalance => ().write_xdr(w)?,
                Self::TxNoAccount => ().write_xdr(w)?,
                Self::TxInsufficientFee => ().write_xdr(w)?,
                Self::TxBadAuthExtra => ().write_xdr(w)?,
                Self::TxInternalError => ().write_xdr(w)?,
                Self::TxNotSupported => ().write_xdr(w)?,
                Self::TxBadSponsorship => ().write_xdr(w)?,
                Self::TxBadMinSeqAgeOrGap => ().write_xdr(w)?,
                Self::TxMalformed => ().write_xdr(w)?,
                Self::TxSorobanInvalid => ().write_xdr(w)?,
                Self::TxFrozenKeyAccessed => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
