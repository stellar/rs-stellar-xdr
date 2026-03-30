#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// RevokeSponsorshipResultCode is an XDR Enum defined as:
///
/// ```text
/// enum RevokeSponsorshipResultCode
/// {
///     // codes considered as "success" for the operation
///     REVOKE_SPONSORSHIP_SUCCESS = 0,
/// 
///     // codes considered as "failure" for the operation
///     REVOKE_SPONSORSHIP_DOES_NOT_EXIST = -1,
///     REVOKE_SPONSORSHIP_NOT_SPONSOR = -2,
///     REVOKE_SPONSORSHIP_LOW_RESERVE = -3,
///     REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE = -4,
///     REVOKE_SPONSORSHIP_MALFORMED = -5
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
pub enum RevokeSponsorshipResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    DoesNotExist = -1,
    NotSponsor = -2,
    LowReserve = -3,
    OnlyTransferable = -4,
    Malformed = -5,
}

impl RevokeSponsorshipResultCode {
    const _VARIANTS: &[RevokeSponsorshipResultCode] = &[
        RevokeSponsorshipResultCode::Success,
        RevokeSponsorshipResultCode::DoesNotExist,
        RevokeSponsorshipResultCode::NotSponsor,
        RevokeSponsorshipResultCode::LowReserve,
        RevokeSponsorshipResultCode::OnlyTransferable,
        RevokeSponsorshipResultCode::Malformed,
    ];
    pub const VARIANTS: [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] = {
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
        "NotSponsor",
        "LowReserve",
        "OnlyTransferable",
        "Malformed",
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
            Self::NotSponsor => "NotSponsor",
            Self::LowReserve => "LowReserve",
            Self::OnlyTransferable => "OnlyTransferable",
            Self::Malformed => "Malformed",
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<RevokeSponsorshipResultCode> for RevokeSponsorshipResultCode {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for RevokeSponsorshipResultCode {}

impl fmt::Display for RevokeSponsorshipResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for RevokeSponsorshipResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => RevokeSponsorshipResultCode::Success,
            -1 => RevokeSponsorshipResultCode::DoesNotExist,
            -2 => RevokeSponsorshipResultCode::NotSponsor,
            -3 => RevokeSponsorshipResultCode::LowReserve,
            -4 => RevokeSponsorshipResultCode::OnlyTransferable,
            -5 => RevokeSponsorshipResultCode::Malformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipResultCode> for i32 {
    #[must_use]
    fn from(e: RevokeSponsorshipResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for RevokeSponsorshipResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
