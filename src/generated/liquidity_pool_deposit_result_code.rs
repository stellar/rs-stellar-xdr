#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolDepositResultCode is an XDR Enum defined as:
///
/// ```text
/// enum LiquidityPoolDepositResultCode
/// {
///     // codes considered as "success" for the operation
///     LIQUIDITY_POOL_DEPOSIT_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     LIQUIDITY_POOL_DEPOSIT_MALFORMED = -1,      // bad input
///     LIQUIDITY_POOL_DEPOSIT_NO_TRUST = -2,       // no trust line for one of the
///                                                 // assets
///     LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED = -3, // not authorized for one of the
///                                                 // assets
///     LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED = -4,    // not enough balance for one of
///                                                 // the assets
///     LIQUIDITY_POOL_DEPOSIT_LINE_FULL = -5,      // pool share trust line doesn't
///                                                 // have sufficient limit
///     LIQUIDITY_POOL_DEPOSIT_BAD_PRICE = -6,      // deposit price outside bounds
///     LIQUIDITY_POOL_DEPOSIT_POOL_FULL = -7,      // pool reserves are full
///     LIQUIDITY_POOL_DEPOSIT_TRUSTLINE_FROZEN = -8  // trustline for one of the
///                                                   // assets is frozen
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
pub enum LiquidityPoolDepositResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoTrust = -2,
    NotAuthorized = -3,
    Underfunded = -4,
    LineFull = -5,
    BadPrice = -6,
    PoolFull = -7,
    TrustlineFrozen = -8,
}

impl LiquidityPoolDepositResultCode {
    const _VARIANTS: &[LiquidityPoolDepositResultCode] = &[
        LiquidityPoolDepositResultCode::Success,
        LiquidityPoolDepositResultCode::Malformed,
        LiquidityPoolDepositResultCode::NoTrust,
        LiquidityPoolDepositResultCode::NotAuthorized,
        LiquidityPoolDepositResultCode::Underfunded,
        LiquidityPoolDepositResultCode::LineFull,
        LiquidityPoolDepositResultCode::BadPrice,
        LiquidityPoolDepositResultCode::PoolFull,
        LiquidityPoolDepositResultCode::TrustlineFrozen,
    ];
    pub const VARIANTS: [LiquidityPoolDepositResultCode; Self::_VARIANTS.len()] = {
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
        "NoTrust",
        "NotAuthorized",
        "Underfunded",
        "LineFull",
        "BadPrice",
        "PoolFull",
        "TrustlineFrozen",
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
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::BadPrice => "BadPrice",
            Self::PoolFull => "PoolFull",
            Self::TrustlineFrozen => "TrustlineFrozen",
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolDepositResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolDepositResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResultCode {
    fn variants() -> slice::Iter<'static, LiquidityPoolDepositResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LiquidityPoolDepositResultCode {}

impl fmt::Display for LiquidityPoolDepositResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LiquidityPoolDepositResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => LiquidityPoolDepositResultCode::Success,
            -1 => LiquidityPoolDepositResultCode::Malformed,
            -2 => LiquidityPoolDepositResultCode::NoTrust,
            -3 => LiquidityPoolDepositResultCode::NotAuthorized,
            -4 => LiquidityPoolDepositResultCode::Underfunded,
            -5 => LiquidityPoolDepositResultCode::LineFull,
            -6 => LiquidityPoolDepositResultCode::BadPrice,
            -7 => LiquidityPoolDepositResultCode::PoolFull,
            -8 => LiquidityPoolDepositResultCode::TrustlineFrozen,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolDepositResultCode> for i32 {
    #[must_use]
    fn from(e: LiquidityPoolDepositResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for LiquidityPoolDepositResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl LiquidityPoolDepositResultCode {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for LiquidityPoolDepositResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
