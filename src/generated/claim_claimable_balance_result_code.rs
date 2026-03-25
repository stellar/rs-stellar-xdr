#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ClaimClaimableBalanceResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ClaimClaimableBalanceResultCode
/// {
///     CLAIM_CLAIMABLE_BALANCE_SUCCESS = 0,
///     CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
///     CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM = -2,
///     CLAIM_CLAIMABLE_BALANCE_LINE_FULL = -3,
///     CLAIM_CLAIMABLE_BALANCE_NO_TRUST = -4,
///     CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -5,
///     CLAIM_CLAIMABLE_BALANCE_TRUSTLINE_FROZEN = -6
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
pub enum ClaimClaimableBalanceResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    DoesNotExist = -1,
    CannotClaim = -2,
    LineFull = -3,
    NoTrust = -4,
    NotAuthorized = -5,
    TrustlineFrozen = -6,
}

impl ClaimClaimableBalanceResultCode {
    const _VARIANTS: &[ClaimClaimableBalanceResultCode] = &[
        ClaimClaimableBalanceResultCode::Success,
        ClaimClaimableBalanceResultCode::DoesNotExist,
        ClaimClaimableBalanceResultCode::CannotClaim,
        ClaimClaimableBalanceResultCode::LineFull,
        ClaimClaimableBalanceResultCode::NoTrust,
        ClaimClaimableBalanceResultCode::NotAuthorized,
        ClaimClaimableBalanceResultCode::TrustlineFrozen,
    ];
    pub const VARIANTS: [ClaimClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
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
        "DoesNotExist",
        "CannotClaim",
        "LineFull",
        "NoTrust",
        "NotAuthorized",
        "TrustlineFrozen",
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
            Self::DoesNotExist => "DoesNotExist",
            Self::CannotClaim => "CannotClaim",
            Self::LineFull => "LineFull",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::TrustlineFrozen => "TrustlineFrozen",
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimClaimableBalanceResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ClaimClaimableBalanceResultCode> for ClaimClaimableBalanceResultCode {
    fn variants() -> slice::Iter<'static, ClaimClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ClaimClaimableBalanceResultCode {}

impl fmt::Display for ClaimClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ClaimClaimableBalanceResultCode::Success,
            -1 => ClaimClaimableBalanceResultCode::DoesNotExist,
            -2 => ClaimClaimableBalanceResultCode::CannotClaim,
            -3 => ClaimClaimableBalanceResultCode::LineFull,
            -4 => ClaimClaimableBalanceResultCode::NoTrust,
            -5 => ClaimClaimableBalanceResultCode::NotAuthorized,
            -6 => ClaimClaimableBalanceResultCode::TrustlineFrozen,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: ClaimClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
