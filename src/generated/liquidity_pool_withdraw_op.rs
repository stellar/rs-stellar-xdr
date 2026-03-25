#[allow(unused_imports)]
use super::*;
/// LiquidityPoolWithdrawOp is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolWithdrawOp
/// {
///     PoolID liquidityPoolID;
///     int64 amount;     // amount of pool shares to withdraw
///     int64 minAmountA; // minimum amount of first asset to withdraw
///     int64 minAmountB; // minimum amount of second asset to withdraw
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
pub struct LiquidityPoolWithdrawOp {
    pub liquidity_pool_id: PoolId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub min_amount_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub min_amount_b: i64,
}

impl ReadXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                min_amount_a: i64::read_xdr(r)?,
                min_amount_b: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.min_amount_a.write_xdr(w)?;
            self.min_amount_b.write_xdr(w)?;
            Ok(())
        })
    }
}
