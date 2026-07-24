#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolEntry is an XDR Struct defined as:
///
/// ```text
/// struct LiquidityPoolEntry
/// {
///     PoolID liquidityPoolID;
///
///     union switch (LiquidityPoolType type)
///     {
///     case LIQUIDITY_POOL_CONSTANT_PRODUCT:
///         struct
///         {
///             LiquidityPoolConstantProductParameters params;
///
///             int64 reserveA;        // amount of A in the pool
///             int64 reserveB;        // amount of B in the pool
///             int64 totalPoolShares; // total number of pool shares issued
///             int64 poolSharesTrustLineCount; // number of trust lines for the
///                                             // associated pool shares
///         } constantProduct;
///     }
///     body;
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
pub struct LiquidityPoolEntry {
    pub liquidity_pool_id: PoolId,
    pub body: LiquidityPoolEntryBody,
}

impl ReadXdr for LiquidityPoolEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liquidity_pool_id: PoolId::read_xdr(r)?,
                body: LiquidityPoolEntryBody::read_xdr(r)?,
            })
        })
    }
}

impl LiquidityPoolEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.liquidity_pool_id.const_write_xdr(w);
        self.body.const_write_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for LiquidityPoolEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liquidity_pool_id.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
