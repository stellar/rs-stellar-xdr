#[allow(unused_imports, clippy::wildcard_imports)]
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

impl ScErrorCode {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ScErrorCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
