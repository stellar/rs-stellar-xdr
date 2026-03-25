#[allow(unused_imports)]
use super::*;
/// ScErrorCode is an XDR Enum defined as:
///
/// ```text
/// enum SCErrorCode
/// {
///     SCEC_ARITH_DOMAIN = 0,      // Some arithmetic was undefined (overflow, divide-by-zero).
///     SCEC_INDEX_BOUNDS = 1,      // Something was indexed beyond its bounds.
///     SCEC_INVALID_INPUT = 2,     // User provided some otherwise-bad data.
///     SCEC_MISSING_VALUE = 3,     // Some value was required but not provided.
///     SCEC_EXISTING_VALUE = 4,    // Some value was provided where not allowed.
///     SCEC_EXCEEDED_LIMIT = 5,    // Some arbitrary limit -- gas or otherwise -- was hit.
///     SCEC_INVALID_ACTION = 6,    // Data was valid but action requested was not.
///     SCEC_INTERNAL_ERROR = 7,    // The host detected an error in its own logic.
///     SCEC_UNEXPECTED_TYPE = 8,   // Some type wasn't as expected.
///     SCEC_UNEXPECTED_SIZE = 9    // Something's size wasn't as expected.
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
pub enum ScErrorCode {
    #[cfg_attr(feature = "alloc", default)]
    ArithDomain = 0,
    IndexBounds = 1,
    InvalidInput = 2,
    MissingValue = 3,
    ExistingValue = 4,
    ExceededLimit = 5,
    InvalidAction = 6,
    InternalError = 7,
    UnexpectedType = 8,
    UnexpectedSize = 9,
}

impl ScErrorCode {
    const _VARIANTS: &[ScErrorCode] = &[
        ScErrorCode::ArithDomain,
        ScErrorCode::IndexBounds,
        ScErrorCode::InvalidInput,
        ScErrorCode::MissingValue,
        ScErrorCode::ExistingValue,
        ScErrorCode::ExceededLimit,
        ScErrorCode::InvalidAction,
        ScErrorCode::InternalError,
        ScErrorCode::UnexpectedType,
        ScErrorCode::UnexpectedSize,
    ];
    pub const VARIANTS: [ScErrorCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ArithDomain",
        "IndexBounds",
        "InvalidInput",
        "MissingValue",
        "ExistingValue",
        "ExceededLimit",
        "InvalidAction",
        "InternalError",
        "UnexpectedType",
        "UnexpectedSize",
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
            Self::ArithDomain => "ArithDomain",
            Self::IndexBounds => "IndexBounds",
            Self::InvalidInput => "InvalidInput",
            Self::MissingValue => "MissingValue",
            Self::ExistingValue => "ExistingValue",
            Self::ExceededLimit => "ExceededLimit",
            Self::InvalidAction => "InvalidAction",
            Self::InternalError => "InternalError",
            Self::UnexpectedType => "UnexpectedType",
            Self::UnexpectedSize => "UnexpectedSize",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScErrorCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScErrorCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScErrorCode> for ScErrorCode {
    fn variants() -> slice::Iter<'static, ScErrorCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScErrorCode {}

impl fmt::Display for ScErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScErrorCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScErrorCode::ArithDomain,
            1 => ScErrorCode::IndexBounds,
            2 => ScErrorCode::InvalidInput,
            3 => ScErrorCode::MissingValue,
            4 => ScErrorCode::ExistingValue,
            5 => ScErrorCode::ExceededLimit,
            6 => ScErrorCode::InvalidAction,
            7 => ScErrorCode::InternalError,
            8 => ScErrorCode::UnexpectedType,
            9 => ScErrorCode::UnexpectedSize,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScErrorCode> for i32 {
    #[must_use]
    fn from(e: ScErrorCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ScErrorCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScErrorCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
