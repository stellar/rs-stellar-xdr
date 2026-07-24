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

impl ClawbackResultCode {
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

impl WriteXdr for ClawbackResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
