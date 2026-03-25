#[allow(unused_imports)]
use super::*;
/// SetTrustLineFlagsResultCode is an XDR Enum defined as:
///
/// ```text
/// enum SetTrustLineFlagsResultCode
/// {
///     // codes considered as "success" for the operation
///     SET_TRUST_LINE_FLAGS_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     SET_TRUST_LINE_FLAGS_MALFORMED = -1,
///     SET_TRUST_LINE_FLAGS_NO_TRUST_LINE = -2,
///     SET_TRUST_LINE_FLAGS_CANT_REVOKE = -3,
///     SET_TRUST_LINE_FLAGS_INVALID_STATE = -4,
///     SET_TRUST_LINE_FLAGS_LOW_RESERVE = -5 // claimable balances can't be created
///                                           // on revoke due to low reserves
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
pub enum SetTrustLineFlagsResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoTrustLine = -2,
    CantRevoke = -3,
    InvalidState = -4,
    LowReserve = -5,
}

impl SetTrustLineFlagsResultCode {
    const _VARIANTS: &[SetTrustLineFlagsResultCode] = &[
        SetTrustLineFlagsResultCode::Success,
        SetTrustLineFlagsResultCode::Malformed,
        SetTrustLineFlagsResultCode::NoTrustLine,
        SetTrustLineFlagsResultCode::CantRevoke,
        SetTrustLineFlagsResultCode::InvalidState,
        SetTrustLineFlagsResultCode::LowReserve,
    ];
    pub const VARIANTS: [SetTrustLineFlagsResultCode; Self::_VARIANTS.len()] = {
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
        "Malformed",
        "NoTrustLine",
        "CantRevoke",
        "InvalidState",
        "LowReserve",
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
            Self::Malformed => "Malformed",
            Self::NoTrustLine => "NoTrustLine",
            Self::CantRevoke => "CantRevoke",
            Self::InvalidState => "InvalidState",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn variants() -> [SetTrustLineFlagsResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SetTrustLineFlagsResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<SetTrustLineFlagsResultCode> for SetTrustLineFlagsResultCode {
    fn variants() -> slice::Iter<'static, SetTrustLineFlagsResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for SetTrustLineFlagsResultCode {}

impl fmt::Display for SetTrustLineFlagsResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SetTrustLineFlagsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => SetTrustLineFlagsResultCode::Success,
            -1 => SetTrustLineFlagsResultCode::Malformed,
            -2 => SetTrustLineFlagsResultCode::NoTrustLine,
            -3 => SetTrustLineFlagsResultCode::CantRevoke,
            -4 => SetTrustLineFlagsResultCode::InvalidState,
            -5 => SetTrustLineFlagsResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetTrustLineFlagsResultCode> for i32 {
    #[must_use]
    fn from(e: SetTrustLineFlagsResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for SetTrustLineFlagsResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for SetTrustLineFlagsResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
