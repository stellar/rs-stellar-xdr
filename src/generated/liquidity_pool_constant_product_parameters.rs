#[allow(unused_imports, clippy::wildcard_imports)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl LiquidityPoolConstantProductParameters {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.asset_a.const_write_xdr(w);
        self.asset_b.const_write_xdr(w);
        w.write_i32(self.fee);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
