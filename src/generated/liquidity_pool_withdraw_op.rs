#[allow(unused_imports, clippy::wildcard_imports)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

#[cfg(feature = "const")]
impl LiquidityPoolWithdrawOp {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_liquidity_pool_withdraw_op(self);
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
        w.write_type_liquidity_pool_withdraw_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LiquidityPoolWithdrawOp`], mirroring `<LiquidityPoolWithdrawOp as WriteXdr>::write_xdr`.
    pub const fn write_type_liquidity_pool_withdraw_op(&mut self, v: &LiquidityPoolWithdrawOp) {
        self.write_type_pool_id(&v.liquidity_pool_id);
        self.write_i64(v.amount);
        self.write_i64(v.min_amount_a);
        self.write_i64(v.min_amount_b);
    }
}
