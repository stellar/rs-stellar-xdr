#[allow(unused_imports)]
use super::*;
/// ChangeTrustAsset is an XDR Union defined as:
///
/// ```text
/// union ChangeTrustAsset switch (AssetType type)
/// {
/// case ASSET_TYPE_NATIVE: // Not credit
///     void;
///
/// case ASSET_TYPE_CREDIT_ALPHANUM4:
///     AlphaNum4 alphaNum4;
///
/// case ASSET_TYPE_CREDIT_ALPHANUM12:
///     AlphaNum12 alphaNum12;
///
/// case ASSET_TYPE_POOL_SHARE:
///     LiquidityPoolParameters liquidityPool;
///
///     // add other asset types here in the future
/// };
/// ```
///
// union with discriminant AssetType
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
pub enum ChangeTrustAsset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
    PoolShare(LiquidityPoolParameters),
}

#[cfg(feature = "alloc")]
impl Default for ChangeTrustAsset {
    fn default() -> Self {
        Self::Native
    }
}

impl ChangeTrustAsset {
    const _VARIANTS: &[AssetType] = &[
        AssetType::Native,
        AssetType::CreditAlphanum4,
        AssetType::CreditAlphanum12,
        AssetType::PoolShare,
    ];
    pub const VARIANTS: [AssetType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Native", "CreditAlphanum4", "CreditAlphanum12", "PoolShare"];
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
            Self::Native => "Native",
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
            Self::PoolShare(_) => "PoolShare",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => AssetType::Native,
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
            Self::PoolShare(_) => AssetType::PoolShare,
        }
    }

    #[must_use]
    pub const fn variants() -> [AssetType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ChangeTrustAsset {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AssetType> for ChangeTrustAsset {
    #[must_use]
    fn discriminant(&self) -> AssetType {
        Self::discriminant(self)
    }
}

impl Variants<AssetType> for ChangeTrustAsset {
    fn variants() -> slice::Iter<'static, AssetType> {
        Self::VARIANTS.iter()
    }
}

impl Union<AssetType> for ChangeTrustAsset {}

impl ReadXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AssetType::Native => Self::Native,
                AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AlphaNum4::read_xdr(r)?),
                AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AlphaNum12::read_xdr(r)?),
                AssetType::PoolShare => Self::PoolShare(LiquidityPoolParameters::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Native => ().write_xdr(w)?,
                Self::CreditAlphanum4(v) => v.write_xdr(w)?,
                Self::CreditAlphanum12(v) => v.write_xdr(w)?,
                Self::PoolShare(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
