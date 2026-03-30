#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolDepositResult is an XDR Union defined as:
///
/// ```text
/// union LiquidityPoolDepositResult switch (LiquidityPoolDepositResultCode code)
/// {
/// case LIQUIDITY_POOL_DEPOSIT_SUCCESS:
///     void;
/// case LIQUIDITY_POOL_DEPOSIT_MALFORMED:
/// case LIQUIDITY_POOL_DEPOSIT_NO_TRUST:
/// case LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED:
/// case LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED:
/// case LIQUIDITY_POOL_DEPOSIT_LINE_FULL:
/// case LIQUIDITY_POOL_DEPOSIT_BAD_PRICE:
/// case LIQUIDITY_POOL_DEPOSIT_POOL_FULL:
/// case LIQUIDITY_POOL_DEPOSIT_TRUSTLINE_FROZEN:
///     void;
/// };
/// ```
///
// union with discriminant LiquidityPoolDepositResultCode
#[cfg_eval::cfg_eval]
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
pub enum LiquidityPoolDepositResult {
    Success,
    Malformed,
    NoTrust,
    NotAuthorized,
    Underfunded,
    LineFull,
    BadPrice,
    PoolFull,
    TrustlineFrozen,
}


#[cfg(feature = "alloc")]
impl Default for LiquidityPoolDepositResult {
    fn default() -> Self {
        Self::Success
    }
}

impl LiquidityPoolDepositResult {
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
    pub const fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => LiquidityPoolDepositResultCode::Success,
            Self::Malformed => LiquidityPoolDepositResultCode::Malformed,
            Self::NoTrust => LiquidityPoolDepositResultCode::NoTrust,
            Self::NotAuthorized => LiquidityPoolDepositResultCode::NotAuthorized,
            Self::Underfunded => LiquidityPoolDepositResultCode::Underfunded,
            Self::LineFull => LiquidityPoolDepositResultCode::LineFull,
            Self::BadPrice => LiquidityPoolDepositResultCode::BadPrice,
            Self::PoolFull => LiquidityPoolDepositResultCode::PoolFull,
            Self::TrustlineFrozen => LiquidityPoolDepositResultCode::TrustlineFrozen,
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolDepositResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolDepositResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {
    #[must_use]
    fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        Self::discriminant(self)
    }
}

impl Variants<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {
    fn variants() -> slice::Iter<'static, LiquidityPoolDepositResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {}

impl ReadXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LiquidityPoolDepositResultCode = <LiquidityPoolDepositResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LiquidityPoolDepositResultCode::Success => Self::Success,
                LiquidityPoolDepositResultCode::Malformed => Self::Malformed,
                LiquidityPoolDepositResultCode::NoTrust => Self::NoTrust,
                LiquidityPoolDepositResultCode::NotAuthorized => Self::NotAuthorized,
                LiquidityPoolDepositResultCode::Underfunded => Self::Underfunded,
                LiquidityPoolDepositResultCode::LineFull => Self::LineFull,
                LiquidityPoolDepositResultCode::BadPrice => Self::BadPrice,
                LiquidityPoolDepositResultCode::PoolFull => Self::PoolFull,
                LiquidityPoolDepositResultCode::TrustlineFrozen => Self::TrustlineFrozen,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::BadPrice => ().write_xdr(w)?,
                Self::PoolFull => ().write_xdr(w)?,
                Self::TrustlineFrozen => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
