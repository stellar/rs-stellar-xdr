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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

#[cfg(feature = "const")]
impl ClaimLiquidityAtom {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_claim_liquidity_atom(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_claim_liquidity_atom(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ClaimLiquidityAtom`], mirroring `<ClaimLiquidityAtom as WriteXdr>::write_xdr`.
    pub const fn write_type_claim_liquidity_atom(&mut self, v: &ClaimLiquidityAtom) {
        self.write_type_pool_id(&v.liquidity_pool_id);
        self.write_type_asset(&v.asset_sold);
        self.write_i64(v.amount_sold);
        self.write_type_asset(&v.asset_bought);
        self.write_i64(v.amount_bought);
    }
}
