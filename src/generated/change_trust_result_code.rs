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

impl ChangeTrustResultCode {
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

impl WriteXdr for ChangeTrustResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
