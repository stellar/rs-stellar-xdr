#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationResultCode is an XDR Enum defined as:
///
/// ```text
/// enum OperationResultCode
/// {
///     opINNER = 0, // inner object result is valid
///
///     opBAD_AUTH = -1,            // too few valid signatures / wrong network
///     opNO_ACCOUNT = -2,          // source account was not found
///     opNOT_SUPPORTED = -3,       // operation not supported at this time
///     opTOO_MANY_SUBENTRIES = -4, // max number of subentries already reached
///     opEXCEEDED_WORK_LIMIT = -5, // operation did too much work
///     opTOO_MANY_SPONSORING = -6  // account is sponsoring too many entries
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
pub enum OperationResultCode {
    #[cfg_attr(feature = "alloc", default)]
    OpInner = 0,
    OpBadAuth = -1,
    OpNoAccount = -2,
    OpNotSupported = -3,
    OpTooManySubentries = -4,
    OpExceededWorkLimit = -5,
    OpTooManySponsoring = -6,
}

impl OperationResultCode {
    const _VARIANTS: &[OperationResultCode] = &[
        OperationResultCode::OpInner,
        OperationResultCode::OpBadAuth,
        OperationResultCode::OpNoAccount,
        OperationResultCode::OpNotSupported,
        OperationResultCode::OpTooManySubentries,
        OperationResultCode::OpExceededWorkLimit,
        OperationResultCode::OpTooManySponsoring,
    ];
    pub const VARIANTS: [OperationResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "OpInner",
        "OpBadAuth",
        "OpNoAccount",
        "OpNotSupported",
        "OpTooManySubentries",
        "OpExceededWorkLimit",
        "OpTooManySponsoring",
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
            Self::OpInner => "OpInner",
            Self::OpBadAuth => "OpBadAuth",
            Self::OpNoAccount => "OpNoAccount",
            Self::OpNotSupported => "OpNotSupported",
            Self::OpTooManySubentries => "OpTooManySubentries",
            Self::OpExceededWorkLimit => "OpExceededWorkLimit",
            Self::OpTooManySponsoring => "OpTooManySponsoring",
        }
    }

    #[must_use]
    pub const fn variants() -> [OperationResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OperationResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<OperationResultCode> for OperationResultCode {
    fn variants() -> slice::Iter<'static, OperationResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for OperationResultCode {}

impl fmt::Display for OperationResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OperationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => OperationResultCode::OpInner,
            -1 => OperationResultCode::OpBadAuth,
            -2 => OperationResultCode::OpNoAccount,
            -3 => OperationResultCode::OpNotSupported,
            -4 => OperationResultCode::OpTooManySubentries,
            -5 => OperationResultCode::OpExceededWorkLimit,
            -6 => OperationResultCode::OpTooManySponsoring,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationResultCode> for i32 {
    #[must_use]
    fn from(e: OperationResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for OperationResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl OperationResultCode {
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

impl WriteXdr for OperationResultCode {
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
