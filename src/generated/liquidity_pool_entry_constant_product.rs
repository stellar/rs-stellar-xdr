#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LiquidityPoolEntryConstantProduct is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             LiquidityPoolConstantProductParameters params;
///
///             int64 reserveA;        // amount of A in the pool
///             int64 reserveB;        // amount of B in the pool
///             int64 totalPoolShares; // total number of pool shares issued
///             int64 poolSharesTrustLineCount; // number of trust lines for the
///                                             // associated pool shares
///         }
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
pub struct LiquidityPoolEntryConstantProduct {
    pub params: LiquidityPoolConstantProductParameters,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub reserve_a: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub reserve_b: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_pool_shares: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub pool_shares_trust_line_count: i64,
}

impl ReadXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                params: LiquidityPoolConstantProductParameters::read_xdr(r)?,
                reserve_a: i64::read_xdr(r)?,
                reserve_b: i64::read_xdr(r)?,
                total_pool_shares: i64::read_xdr(r)?,
                pool_shares_trust_line_count: i64::read_xdr(r)?,
            })
        })
    }
}

impl LiquidityPoolEntryConstantProduct {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.params.const_write_xdr(w);
        w.write_i64(self.reserve_a);
        w.write_i64(self.reserve_b);
        w.write_i64(self.total_pool_shares);
        w.write_i64(self.pool_shares_trust_line_count);
        w.leave_depth();
    }
}

impl WriteXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.params.write_xdr(w)?;
            self.reserve_a.write_xdr(w)?;
            self.reserve_b.write_xdr(w)?;
            self.total_pool_shares.write_xdr(w)?;
            self.pool_shares_trust_line_count.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
