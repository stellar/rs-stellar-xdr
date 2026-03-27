#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolWithdrawResult is an XDR Union defined as:
///
/// ```text
/// union LiquidityPoolWithdrawResult switch (LiquidityPoolWithdrawResultCode code)
/// {
/// case LIQUIDITY_POOL_WITHDRAW_SUCCESS:
///     void;
/// case LIQUIDITY_POOL_WITHDRAW_MALFORMED:
/// case LIQUIDITY_POOL_WITHDRAW_NO_TRUST:
/// case LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED:
/// case LIQUIDITY_POOL_WITHDRAW_LINE_FULL:
/// case LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM:
/// case LIQUIDITY_POOL_WITHDRAW_TRUSTLINE_FROZEN:
///     void;
/// };
/// ```
///
// union with discriminant LiquidityPoolWithdrawResultCode
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum LiquidityPoolWithdrawResult {
    Success,
    Malformed,
    NoTrust,
    Underfunded,
    LineFull,
    UnderMinimum,
    TrustlineFrozen,
}

#[cfg(feature = "alloc")]
impl Default for LiquidityPoolWithdrawResult {
    fn default() -> Self {
        Self::Success
    }
}

impl LiquidityPoolWithdrawResult {
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
    pub const fn discriminant(&self) -> LiquidityPoolWithdrawResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => LiquidityPoolWithdrawResultCode::Success,
            Self::Malformed => LiquidityPoolWithdrawResultCode::Malformed,
            Self::NoTrust => LiquidityPoolWithdrawResultCode::NoTrust,
            Self::Underfunded => LiquidityPoolWithdrawResultCode::Underfunded,
            Self::LineFull => LiquidityPoolWithdrawResultCode::LineFull,
            Self::UnderMinimum => LiquidityPoolWithdrawResultCode::UnderMinimum,
            Self::TrustlineFrozen => LiquidityPoolWithdrawResultCode::TrustlineFrozen,
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolWithdrawResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolWithdrawResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LiquidityPoolWithdrawResultCode> for LiquidityPoolWithdrawResult {
    #[must_use]
    fn discriminant(&self) -> LiquidityPoolWithdrawResultCode {
        Self::discriminant(self)
    }
}

impl Variants<LiquidityPoolWithdrawResultCode> for LiquidityPoolWithdrawResult {
    fn variants() -> slice::Iter<'static, LiquidityPoolWithdrawResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<LiquidityPoolWithdrawResultCode> for LiquidityPoolWithdrawResult {}

impl ReadXdr for LiquidityPoolWithdrawResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LiquidityPoolWithdrawResultCode =
                <LiquidityPoolWithdrawResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LiquidityPoolWithdrawResultCode::Success => Self::Success,
                LiquidityPoolWithdrawResultCode::Malformed => Self::Malformed,
                LiquidityPoolWithdrawResultCode::NoTrust => Self::NoTrust,
                LiquidityPoolWithdrawResultCode::Underfunded => Self::Underfunded,
                LiquidityPoolWithdrawResultCode::LineFull => Self::LineFull,
                LiquidityPoolWithdrawResultCode::UnderMinimum => Self::UnderMinimum,
                LiquidityPoolWithdrawResultCode::TrustlineFrozen => Self::TrustlineFrozen,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LiquidityPoolWithdrawResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::UnderMinimum => ().write_xdr(w)?,
                Self::TrustlineFrozen => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
