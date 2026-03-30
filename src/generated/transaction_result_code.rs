#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultCode is an XDR Enum defined as:
///
/// ```text
/// enum TransactionResultCode
/// {
///     txFEE_BUMP_INNER_SUCCESS = 1, // fee bump inner transaction succeeded
///     txSUCCESS = 0,                // all operations succeeded
/// 
///     txFAILED = -1, // one of the operations failed (none were applied)
/// 
///     txTOO_EARLY = -2,         // ledger closeTime before minTime
///     txTOO_LATE = -3,          // ledger closeTime after maxTime
///     txMISSING_OPERATION = -4, // no operation was specified
///     txBAD_SEQ = -5,           // sequence number does not match source account
/// 
///     txBAD_AUTH = -6,             // too few valid signatures / wrong network
///     txINSUFFICIENT_BALANCE = -7, // fee would bring account below reserve
///     txNO_ACCOUNT = -8,           // source account not found
///     txINSUFFICIENT_FEE = -9,     // fee is too small
///     txBAD_AUTH_EXTRA = -10,      // unused signatures attached to transaction
///     txINTERNAL_ERROR = -11,      // an unknown error occurred
/// 
///     txNOT_SUPPORTED = -12,          // transaction type not supported
///     txFEE_BUMP_INNER_FAILED = -13,  // fee bump inner transaction failed
///     txBAD_SPONSORSHIP = -14,        // sponsorship not confirmed
///     txBAD_MIN_SEQ_AGE_OR_GAP = -15, // minSeqAge or minSeqLedgerGap conditions not met
///     txMALFORMED = -16,              // precondition is invalid
///     txSOROBAN_INVALID = -17,        // soroban-specific preconditions were not met
///     txFROZEN_KEY_ACCESSED = -18     // a 'frozen' ledger key is accessed by any operation
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
pub enum TransactionResultCode {
    #[cfg_attr(feature = "alloc", default)]
    TxFeeBumpInnerSuccess = 1,
    TxSuccess = 0,
    TxFailed = -1,
    TxTooEarly = -2,
    TxTooLate = -3,
    TxMissingOperation = -4,
    TxBadSeq = -5,
    TxBadAuth = -6,
    TxInsufficientBalance = -7,
    TxNoAccount = -8,
    TxInsufficientFee = -9,
    TxBadAuthExtra = -10,
    TxInternalError = -11,
    TxNotSupported = -12,
    TxFeeBumpInnerFailed = -13,
    TxBadSponsorship = -14,
    TxBadMinSeqAgeOrGap = -15,
    TxMalformed = -16,
    TxSorobanInvalid = -17,
    TxFrozenKeyAccessed = -18,
}

impl TransactionResultCode {
    const _VARIANTS: &[TransactionResultCode] = &[
        TransactionResultCode::TxFeeBumpInnerSuccess,
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
        TransactionResultCode::TxFeeBumpInnerFailed,
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
        "TxFeeBumpInnerFailed",
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
            Self::TxFeeBumpInnerSuccess => "TxFeeBumpInnerSuccess",
            Self::TxSuccess => "TxSuccess",
            Self::TxFailed => "TxFailed",
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
            Self::TxFeeBumpInnerFailed => "TxFeeBumpInnerFailed",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
            Self::TxSorobanInvalid => "TxSorobanInvalid",
            Self::TxFrozenKeyAccessed => "TxFrozenKeyAccessed",
        }
    }

    #[must_use]
    pub const fn variants() -> [TransactionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<TransactionResultCode> for TransactionResultCode {
    fn variants() -> slice::Iter<'static, TransactionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for TransactionResultCode {}

impl fmt::Display for TransactionResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for TransactionResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            1 => TransactionResultCode::TxFeeBumpInnerSuccess,
            0 => TransactionResultCode::TxSuccess,
            -1 => TransactionResultCode::TxFailed,
            -2 => TransactionResultCode::TxTooEarly,
            -3 => TransactionResultCode::TxTooLate,
            -4 => TransactionResultCode::TxMissingOperation,
            -5 => TransactionResultCode::TxBadSeq,
            -6 => TransactionResultCode::TxBadAuth,
            -7 => TransactionResultCode::TxInsufficientBalance,
            -8 => TransactionResultCode::TxNoAccount,
            -9 => TransactionResultCode::TxInsufficientFee,
            -10 => TransactionResultCode::TxBadAuthExtra,
            -11 => TransactionResultCode::TxInternalError,
            -12 => TransactionResultCode::TxNotSupported,
            -13 => TransactionResultCode::TxFeeBumpInnerFailed,
            -14 => TransactionResultCode::TxBadSponsorship,
            -15 => TransactionResultCode::TxBadMinSeqAgeOrGap,
            -16 => TransactionResultCode::TxMalformed,
            -17 => TransactionResultCode::TxSorobanInvalid,
            -18 => TransactionResultCode::TxFrozenKeyAccessed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TransactionResultCode> for i32 {
    #[must_use]
    fn from(e: TransactionResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for TransactionResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
