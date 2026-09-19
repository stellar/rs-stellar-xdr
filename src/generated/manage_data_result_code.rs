#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageDataResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ManageDataResultCode
/// {
///     // codes considered as "success" for the operation
///     MANAGE_DATA_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     MANAGE_DATA_NOT_SUPPORTED_YET =
///         -1, // The network hasn't moved to this protocol change yet
///     MANAGE_DATA_NAME_NOT_FOUND =
///         -2, // Trying to remove a Data Entry that isn't there
///     MANAGE_DATA_LOW_RESERVE = -3, // not enough funds to create a new Data Entry
///     MANAGE_DATA_INVALID_NAME = -4 // Name not a valid string
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
pub enum ManageDataResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    NotSupportedYet = -1,
    NameNotFound = -2,
    LowReserve = -3,
    InvalidName = -4,
}

impl ManageDataResultCode {
    const _VARIANTS: &[ManageDataResultCode] = &[
        ManageDataResultCode::Success,
        ManageDataResultCode::NotSupportedYet,
        ManageDataResultCode::NameNotFound,
        ManageDataResultCode::LowReserve,
        ManageDataResultCode::InvalidName,
    ];
    pub const VARIANTS: [ManageDataResultCode; Self::_VARIANTS.len()] = {
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
        "NotSupportedYet",
        "NameNotFound",
        "LowReserve",
        "InvalidName",
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
            Self::NotSupportedYet => "NotSupportedYet",
            Self::NameNotFound => "NameNotFound",
            Self::LowReserve => "LowReserve",
            Self::InvalidName => "InvalidName",
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageDataResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageDataResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ManageDataResultCode> for ManageDataResultCode {
    fn variants() -> slice::Iter<'static, ManageDataResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ManageDataResultCode {}

impl fmt::Display for ManageDataResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageDataResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ManageDataResultCode::Success,
            -1 => ManageDataResultCode::NotSupportedYet,
            -2 => ManageDataResultCode::NameNotFound,
            -3 => ManageDataResultCode::LowReserve,
            -4 => ManageDataResultCode::InvalidName,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageDataResultCode> for i32 {
    #[must_use]
    fn from(e: ManageDataResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageDataResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ManageDataResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
