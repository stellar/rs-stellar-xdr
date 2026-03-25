#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ClawbackResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ClawbackResultCode
/// {
///     // codes considered as "success" for the operation
///     CLAWBACK_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     CLAWBACK_MALFORMED = -1,
///     CLAWBACK_NOT_CLAWBACK_ENABLED = -2,
///     CLAWBACK_NO_TRUST = -3,
///     CLAWBACK_UNDERFUNDED = -4
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
pub enum ClawbackResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NotClawbackEnabled = -2,
    NoTrust = -3,
    Underfunded = -4,
}

impl ClawbackResultCode {
    const _VARIANTS: &[ClawbackResultCode] = &[
        ClawbackResultCode::Success,
        ClawbackResultCode::Malformed,
        ClawbackResultCode::NotClawbackEnabled,
        ClawbackResultCode::NoTrust,
        ClawbackResultCode::Underfunded,
    ];
    pub const VARIANTS: [ClawbackResultCode; Self::_VARIANTS.len()] = {
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
        "NotClawbackEnabled",
        "NoTrust",
        "Underfunded",
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
            Self::NotClawbackEnabled => "NotClawbackEnabled",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn variants() -> [ClawbackResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClawbackResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ClawbackResultCode> for ClawbackResultCode {
    fn variants() -> slice::Iter<'static, ClawbackResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ClawbackResultCode {}

impl fmt::Display for ClawbackResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClawbackResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ClawbackResultCode::Success,
            -1 => ClawbackResultCode::Malformed,
            -2 => ClawbackResultCode::NotClawbackEnabled,
            -3 => ClawbackResultCode::NoTrust,
            -4 => ClawbackResultCode::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackResultCode> for i32 {
    #[must_use]
    fn from(e: ClawbackResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClawbackResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ClawbackResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
