#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ChangeTrustResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ChangeTrustResultCode
/// {
///     // codes considered as "success" for the operation
///     CHANGE_TRUST_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     CHANGE_TRUST_MALFORMED = -1,     // bad input
///     CHANGE_TRUST_NO_ISSUER = -2,     // could not find issuer
///     CHANGE_TRUST_INVALID_LIMIT = -3, // cannot drop limit below balance
///                                      // cannot create with a limit of 0
///     CHANGE_TRUST_LOW_RESERVE =
///         -4, // not enough funds to create a new trust line,
///     CHANGE_TRUST_SELF_NOT_ALLOWED = -5,   // trusting self is not allowed
///     CHANGE_TRUST_TRUST_LINE_MISSING = -6, // Asset trustline is missing for pool
///     CHANGE_TRUST_CANNOT_DELETE =
///         -7, // Asset trustline is still referenced in a pool
///     CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES =
///         -8 // Asset trustline is deauthorized
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
pub enum ChangeTrustResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoIssuer = -2,
    InvalidLimit = -3,
    LowReserve = -4,
    SelfNotAllowed = -5,
    TrustLineMissing = -6,
    CannotDelete = -7,
    NotAuthMaintainLiabilities = -8,
}

impl ChangeTrustResultCode {
    const _VARIANTS: &[ChangeTrustResultCode] = &[
        ChangeTrustResultCode::Success,
        ChangeTrustResultCode::Malformed,
        ChangeTrustResultCode::NoIssuer,
        ChangeTrustResultCode::InvalidLimit,
        ChangeTrustResultCode::LowReserve,
        ChangeTrustResultCode::SelfNotAllowed,
        ChangeTrustResultCode::TrustLineMissing,
        ChangeTrustResultCode::CannotDelete,
        ChangeTrustResultCode::NotAuthMaintainLiabilities,
    ];
    pub const VARIANTS: [ChangeTrustResultCode; Self::_VARIANTS.len()] = {
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
        "NoIssuer",
        "InvalidLimit",
        "LowReserve",
        "SelfNotAllowed",
        "TrustLineMissing",
        "CannotDelete",
        "NotAuthMaintainLiabilities",
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
            Self::NoIssuer => "NoIssuer",
            Self::InvalidLimit => "InvalidLimit",
            Self::LowReserve => "LowReserve",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::TrustLineMissing => "TrustLineMissing",
            Self::CannotDelete => "CannotDelete",
            Self::NotAuthMaintainLiabilities => "NotAuthMaintainLiabilities",
        }
    }

    #[must_use]
    pub const fn variants() -> [ChangeTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ChangeTrustResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ChangeTrustResultCode> for ChangeTrustResultCode {
    fn variants() -> slice::Iter<'static, ChangeTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ChangeTrustResultCode {}

impl fmt::Display for ChangeTrustResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ChangeTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ChangeTrustResultCode::Success,
            -1 => ChangeTrustResultCode::Malformed,
            -2 => ChangeTrustResultCode::NoIssuer,
            -3 => ChangeTrustResultCode::InvalidLimit,
            -4 => ChangeTrustResultCode::LowReserve,
            -5 => ChangeTrustResultCode::SelfNotAllowed,
            -6 => ChangeTrustResultCode::TrustLineMissing,
            -7 => ChangeTrustResultCode::CannotDelete,
            -8 => ChangeTrustResultCode::NotAuthMaintainLiabilities,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ChangeTrustResultCode> for i32 {
    #[must_use]
    fn from(e: ChangeTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ChangeTrustResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ChangeTrustResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
