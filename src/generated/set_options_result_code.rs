#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// SetOptionsResultCode is an XDR Enum defined as:
///
/// ```text
/// enum SetOptionsResultCode
/// {
///     // codes considered as "success" for the operation
///     SET_OPTIONS_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     SET_OPTIONS_LOW_RESERVE = -1,      // not enough funds to add a signer
///     SET_OPTIONS_TOO_MANY_SIGNERS = -2, // max number of signers already reached
///     SET_OPTIONS_BAD_FLAGS = -3,        // invalid combination of clear/set flags
///     SET_OPTIONS_INVALID_INFLATION = -4,      // inflation account does not exist
///     SET_OPTIONS_CANT_CHANGE = -5,            // can no longer change this option
///     SET_OPTIONS_UNKNOWN_FLAG = -6,           // can't set an unknown flag
///     SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7, // bad value for weight/threshold
///     SET_OPTIONS_BAD_SIGNER = -8,             // signer cannot be masterkey
///     SET_OPTIONS_INVALID_HOME_DOMAIN = -9,    // malformed home domain
///     SET_OPTIONS_AUTH_REVOCABLE_REQUIRED =
///         -10 // auth revocable is required for clawback
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
pub enum SetOptionsResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    LowReserve = -1,
    TooManySigners = -2,
    BadFlags = -3,
    InvalidInflation = -4,
    CantChange = -5,
    UnknownFlag = -6,
    ThresholdOutOfRange = -7,
    BadSigner = -8,
    InvalidHomeDomain = -9,
    AuthRevocableRequired = -10,
}

impl SetOptionsResultCode {
    const _VARIANTS: &[SetOptionsResultCode] = &[
        SetOptionsResultCode::Success,
        SetOptionsResultCode::LowReserve,
        SetOptionsResultCode::TooManySigners,
        SetOptionsResultCode::BadFlags,
        SetOptionsResultCode::InvalidInflation,
        SetOptionsResultCode::CantChange,
        SetOptionsResultCode::UnknownFlag,
        SetOptionsResultCode::ThresholdOutOfRange,
        SetOptionsResultCode::BadSigner,
        SetOptionsResultCode::InvalidHomeDomain,
        SetOptionsResultCode::AuthRevocableRequired,
    ];
    pub const VARIANTS: [SetOptionsResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "LowReserve",
        "TooManySigners",
        "BadFlags",
        "InvalidInflation",
        "CantChange",
        "UnknownFlag",
        "ThresholdOutOfRange",
        "BadSigner",
        "InvalidHomeDomain",
        "AuthRevocableRequired",
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
            Self::Success => "Success",
            Self::LowReserve => "LowReserve",
            Self::TooManySigners => "TooManySigners",
            Self::BadFlags => "BadFlags",
            Self::InvalidInflation => "InvalidInflation",
            Self::CantChange => "CantChange",
            Self::UnknownFlag => "UnknownFlag",
            Self::ThresholdOutOfRange => "ThresholdOutOfRange",
            Self::BadSigner => "BadSigner",
            Self::InvalidHomeDomain => "InvalidHomeDomain",
            Self::AuthRevocableRequired => "AuthRevocableRequired",
        }
    }

    #[must_use]
    pub const fn variants() -> [SetOptionsResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SetOptionsResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<SetOptionsResultCode> for SetOptionsResultCode {
    fn variants() -> slice::Iter<'static, SetOptionsResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for SetOptionsResultCode {}

impl fmt::Display for SetOptionsResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SetOptionsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => SetOptionsResultCode::Success,
            -1 => SetOptionsResultCode::LowReserve,
            -2 => SetOptionsResultCode::TooManySigners,
            -3 => SetOptionsResultCode::BadFlags,
            -4 => SetOptionsResultCode::InvalidInflation,
            -5 => SetOptionsResultCode::CantChange,
            -6 => SetOptionsResultCode::UnknownFlag,
            -7 => SetOptionsResultCode::ThresholdOutOfRange,
            -8 => SetOptionsResultCode::BadSigner,
            -9 => SetOptionsResultCode::InvalidHomeDomain,
            -10 => SetOptionsResultCode::AuthRevocableRequired,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetOptionsResultCode> for i32 {
    #[must_use]
    fn from(e: SetOptionsResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for SetOptionsResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for SetOptionsResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
