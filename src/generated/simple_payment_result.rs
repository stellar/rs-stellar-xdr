#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SimplePaymentResult is an XDR Struct defined as:
///
/// ```text
/// struct SimplePaymentResult
/// {
///     AccountID destination;
///     Asset asset;
///     int64 amount;
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
pub struct SimplePaymentResult {
    pub destination: AccountId,
    pub asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
}

impl ReadXdr for SimplePaymentResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                destination: AccountId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
            })
        })
    }
}

impl SimplePaymentResult {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.destination.const_to_xdr(w);
        self.asset.const_to_xdr(w);
        w.write_i64(self.amount);
        w.leave_depth();
    }
}

impl WriteXdr for SimplePaymentResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.destination.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
