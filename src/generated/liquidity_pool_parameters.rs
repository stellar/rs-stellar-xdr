#[allow(unused_imports)]
use super::*;
/// LiquidityPoolParameters is an XDR Union defined as:
///
/// ```text
/// union LiquidityPoolParameters switch (LiquidityPoolType type)
/// {
/// case LIQUIDITY_POOL_CONSTANT_PRODUCT:
///     LiquidityPoolConstantProductParameters constantProduct;
/// };
/// ```
///
// union with discriminant LiquidityPoolType
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
pub enum LiquidityPoolParameters {
    LiquidityPoolConstantProduct(LiquidityPoolConstantProductParameters),
}

#[cfg(feature = "alloc")]
impl Default for LiquidityPoolParameters {
    fn default() -> Self {
        Self::LiquidityPoolConstantProduct(LiquidityPoolConstantProductParameters::default())
    }
}

impl LiquidityPoolParameters {
    const _VARIANTS: &[LiquidityPoolType] = &[LiquidityPoolType::LiquidityPoolConstantProduct];
    pub const VARIANTS: [LiquidityPoolType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["LiquidityPoolConstantProduct"];
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
            Self::LiquidityPoolConstantProduct(_) => "LiquidityPoolConstantProduct",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LiquidityPoolType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(_) => {
                LiquidityPoolType::LiquidityPoolConstantProduct
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolParameters {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LiquidityPoolType> for LiquidityPoolParameters {
    #[must_use]
    fn discriminant(&self) -> LiquidityPoolType {
        Self::discriminant(self)
    }
}

impl Variants<LiquidityPoolType> for LiquidityPoolParameters {
    fn variants() -> slice::Iter<'static, LiquidityPoolType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LiquidityPoolType> for LiquidityPoolParameters {}

impl ReadXdr for LiquidityPoolParameters {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LiquidityPoolType = <LiquidityPoolType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LiquidityPoolType::LiquidityPoolConstantProduct => {
                    Self::LiquidityPoolConstantProduct(
                        LiquidityPoolConstantProductParameters::read_xdr(r)?,
                    )
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LiquidityPoolParameters {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::LiquidityPoolConstantProduct(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
