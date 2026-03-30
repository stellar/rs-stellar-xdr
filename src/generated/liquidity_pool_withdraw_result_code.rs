#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolWithdrawResultCode is an XDR Enum defined as:
///
/// ```text
/// enum LiquidityPoolWithdrawResultCode
/// {
///     // codes considered as "success" for the operation
///     LIQUIDITY_POOL_WITHDRAW_SUCCESS = 0,
/// 
///     // codes considered as "failure" for the operation
///     LIQUIDITY_POOL_WITHDRAW_MALFORMED = -1,    // bad input
///     LIQUIDITY_POOL_WITHDRAW_NO_TRUST = -2,     // no trust line for one of the
///                                                // assets
///     LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED = -3,  // not enough balance of the
///                                                // pool share
///     LIQUIDITY_POOL_WITHDRAW_LINE_FULL = -4,    // would go above limit for one
///                                                // of the assets
///     LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM = -5, // didn't withdraw enough
///     LIQUIDITY_POOL_WITHDRAW_TRUSTLINE_FROZEN = -6  // trustline for one of the 
///                                                    // assets is frozen
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
pub enum LiquidityPoolWithdrawResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoTrust = -2,
    Underfunded = -3,
    LineFull = -4,
    UnderMinimum = -5,
    TrustlineFrozen = -6,
}

impl LiquidityPoolWithdrawResultCode {
    const _VARIANTS: &[LiquidityPoolWithdrawResultCode] = &[
        LiquidityPoolWithdrawResultCode::Success,
        LiquidityPoolWithdrawResultCode::Malformed,
        LiquidityPoolWithdrawResultCode::NoTrust,
        LiquidityPoolWithdrawResultCode::Underfunded,
        LiquidityPoolWithdrawResultCode::LineFull,
        LiquidityPoolWithdrawResultCode::UnderMinimum,
        LiquidityPoolWithdrawResultCode::TrustlineFrozen,
    ];
    pub const VARIANTS: [LiquidityPoolWithdrawResultCode; Self::_VARIANTS.len()] = {
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
        "Underfunded",
        "LineFull",
        "UnderMinimum",
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
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::UnderMinimum => "UnderMinimum",
            Self::TrustlineFrozen => "TrustlineFrozen",
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolWithdrawResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolWithdrawResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LiquidityPoolWithdrawResultCode> for LiquidityPoolWithdrawResultCode {
    fn variants() -> slice::Iter<'static, LiquidityPoolWithdrawResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LiquidityPoolWithdrawResultCode {}

impl fmt::Display for LiquidityPoolWithdrawResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LiquidityPoolWithdrawResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => LiquidityPoolWithdrawResultCode::Success,
            -1 => LiquidityPoolWithdrawResultCode::Malformed,
            -2 => LiquidityPoolWithdrawResultCode::NoTrust,
            -3 => LiquidityPoolWithdrawResultCode::Underfunded,
            -4 => LiquidityPoolWithdrawResultCode::LineFull,
            -5 => LiquidityPoolWithdrawResultCode::UnderMinimum,
            -6 => LiquidityPoolWithdrawResultCode::TrustlineFrozen,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolWithdrawResultCode> for i32 {
    #[must_use]
    fn from(e: LiquidityPoolWithdrawResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for LiquidityPoolWithdrawResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for LiquidityPoolWithdrawResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
