#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClaimLiquidityAtom is an XDR Struct defined as:
///
/// ```text
/// struct ClaimLiquidityAtom
/// {
///     PoolID liquidityPoolID;
/// 
///     // amount and asset taken from the pool
///     Asset assetSold;
///     int64 amountSold;
/// 
///     // amount and asset sent to the pool
///     Asset assetBought;
///     int64 amountBought;
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
pub struct ClaimLiquidityAtom {
    pub liquidity_pool_id: PoolId,
    pub asset_sold: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount_sold: i64,
    pub asset_bought: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount_bought: i64,
}

impl ReadXdr for ClaimLiquidityAtom {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                asset_sold: Asset::read_xdr(r)?,
                amount_sold: i64::read_xdr(r)?,
                asset_bought: Asset::read_xdr(r)?,
                amount_bought: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimLiquidityAtom {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.asset_sold.write_xdr(w)?;
            self.amount_sold.write_xdr(w)?;
            self.asset_bought.write_xdr(w)?;
            self.amount_bought.write_xdr(w)?;
            Ok(())
        })
    }
}
