#[allow(unused_imports)]
use super::*;
/// BeginSponsoringFutureReservesResultCode is an XDR Enum defined as:
///
/// ```text
/// enum BeginSponsoringFutureReservesResultCode
/// {
///     // codes considered as "success" for the operation
///     BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED = -1,
///     BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED = -2,
///     BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE = -3
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
pub enum BeginSponsoringFutureReservesResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    AlreadySponsored = -2,
    Recursive = -3,
}

impl BeginSponsoringFutureReservesResultCode {
    const _VARIANTS: &[BeginSponsoringFutureReservesResultCode] = &[
        BeginSponsoringFutureReservesResultCode::Success,
        BeginSponsoringFutureReservesResultCode::Malformed,
        BeginSponsoringFutureReservesResultCode::AlreadySponsored,
        BeginSponsoringFutureReservesResultCode::Recursive,
    ];
    pub const VARIANTS: [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "Malformed", "AlreadySponsored", "Recursive"];
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
            Self::AlreadySponsored => "AlreadySponsored",
            Self::Recursive => "Recursive",
        }
    }

    #[must_use]
    pub const fn variants() -> [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BeginSponsoringFutureReservesResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResultCode {
    fn variants() -> slice::Iter<'static, BeginSponsoringFutureReservesResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for BeginSponsoringFutureReservesResultCode {}

impl fmt::Display for BeginSponsoringFutureReservesResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BeginSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => BeginSponsoringFutureReservesResultCode::Success,
            -1 => BeginSponsoringFutureReservesResultCode::Malformed,
            -2 => BeginSponsoringFutureReservesResultCode::AlreadySponsored,
            -3 => BeginSponsoringFutureReservesResultCode::Recursive,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BeginSponsoringFutureReservesResultCode> for i32 {
    #[must_use]
    fn from(e: BeginSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for BeginSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for BeginSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
