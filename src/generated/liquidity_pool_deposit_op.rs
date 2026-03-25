#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// LiquidityPoolDepositOp is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolDepositOp
/// {
///     PoolID liquidityPoolID;
///     int64 maxAmountA; // maximum amount of first asset to deposit
///     int64 maxAmountB; // maximum amount of second asset to deposit
///     Price minPrice;   // minimum depositA/depositB
///     Price maxPrice;   // maximum depositA/depositB
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
pub struct LiquidityPoolDepositOp {
    pub liquidity_pool_id: PoolId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub max_amount_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub max_amount_b: i64,
    pub min_price: Price,
    pub max_price: Price,
}

impl ReadXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                max_amount_a: i64::read_xdr(r)?,
                max_amount_b: i64::read_xdr(r)?,
                min_price: Price::read_xdr(r)?,
                max_price: Price::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.max_amount_a.write_xdr(w)?;
            self.max_amount_b.write_xdr(w)?;
            self.min_price.write_xdr(w)?;
            self.max_price.write_xdr(w)?;
            Ok(())
        })
    }
}
