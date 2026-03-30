#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// EndSponsoringFutureReservesResultCode is an XDR Enum defined as:
///
/// ```text
/// enum EndSponsoringFutureReservesResultCode
/// {
///     // codes considered as "success" for the operation
///     END_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
/// 
///     // codes considered as "failure" for the operation
///     END_SPONSORING_FUTURE_RESERVES_NOT_SPONSORED = -1
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
pub enum EndSponsoringFutureReservesResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    NotSponsored = -1,
}

impl EndSponsoringFutureReservesResultCode {
    const _VARIANTS: &[EndSponsoringFutureReservesResultCode] = &[
        EndSponsoringFutureReservesResultCode::Success,
        EndSponsoringFutureReservesResultCode::NotSponsored,
    ];
    pub const VARIANTS: [EndSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] = {
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
        "NotSponsored",
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
            Self::NotSponsored => "NotSponsored",
        }
    }

    #[must_use]
    pub const fn variants() -> [EndSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for EndSponsoringFutureReservesResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<EndSponsoringFutureReservesResultCode> for EndSponsoringFutureReservesResultCode {
    fn variants() -> slice::Iter<'static, EndSponsoringFutureReservesResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for EndSponsoringFutureReservesResultCode {}

impl fmt::Display for EndSponsoringFutureReservesResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for EndSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => EndSponsoringFutureReservesResultCode::Success,
            -1 => EndSponsoringFutureReservesResultCode::NotSponsored,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EndSponsoringFutureReservesResultCode> for i32 {
    #[must_use]
    fn from(e: EndSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for EndSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for EndSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
