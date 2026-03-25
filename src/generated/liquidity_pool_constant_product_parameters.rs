#[allow(unused_imports)]
use super::*;
/// LiquidityPoolConstantProductParameters is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolConstantProductParameters
/// {
///     Asset assetA; // assetA < assetB
///     Asset assetB;
///     int32 fee; // Fee is in basis points, so the actual rate is (fee/100)%
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct LiquidityPoolConstantProductParameters {
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub fee: i32,
}

impl ReadXdr for LiquidityPoolConstantProductParameters {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                asset_a: Asset::read_xdr(r)?,
                asset_b: Asset::read_xdr(r)?,
                fee: i32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolConstantProductParameters {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.asset_a.write_xdr(w)?;
            self.asset_b.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            Ok(())
        })
    }
}
