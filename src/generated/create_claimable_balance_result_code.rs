#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// CreateClaimableBalanceResultCode is an XDR Enum defined as:
///
/// ```text
/// enum CreateClaimableBalanceResultCode
/// {
///     CREATE_CLAIMABLE_BALANCE_SUCCESS = 0,
///     CREATE_CLAIMABLE_BALANCE_MALFORMED = -1,
///     CREATE_CLAIMABLE_BALANCE_LOW_RESERVE = -2,
///     CREATE_CLAIMABLE_BALANCE_NO_TRUST = -3,
///     CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -4,
///     CREATE_CLAIMABLE_BALANCE_UNDERFUNDED = -5
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
pub enum CreateClaimableBalanceResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    LowReserve = -2,
    NoTrust = -3,
    NotAuthorized = -4,
    Underfunded = -5,
}

impl CreateClaimableBalanceResultCode {
    const _VARIANTS: &[CreateClaimableBalanceResultCode] = &[
        CreateClaimableBalanceResultCode::Success,
        CreateClaimableBalanceResultCode::Malformed,
        CreateClaimableBalanceResultCode::LowReserve,
        CreateClaimableBalanceResultCode::NoTrust,
        CreateClaimableBalanceResultCode::NotAuthorized,
        CreateClaimableBalanceResultCode::Underfunded,
    ];
    pub const VARIANTS: [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
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
        "LowReserve",
        "NoTrust",
        "NotAuthorized",
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
            Self::LowReserve => "LowReserve",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateClaimableBalanceResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResultCode {
    fn variants() -> slice::Iter<'static, CreateClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for CreateClaimableBalanceResultCode {}

impl fmt::Display for CreateClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CreateClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => CreateClaimableBalanceResultCode::Success,
            -1 => CreateClaimableBalanceResultCode::Malformed,
            -2 => CreateClaimableBalanceResultCode::LowReserve,
            -3 => CreateClaimableBalanceResultCode::NoTrust,
            -4 => CreateClaimableBalanceResultCode::NotAuthorized,
            -5 => CreateClaimableBalanceResultCode::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: CreateClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for CreateClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for CreateClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
