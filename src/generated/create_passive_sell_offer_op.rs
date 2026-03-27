#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreatePassiveSellOfferOp is an XDR Struct defined as:
///
/// ```text
/// struct CreatePassiveSellOfferOp
/// {
///     Asset selling; // A
///     Asset buying;  // B
///     int64 amount;  // amount taker gets
///     Price price;   // cost of A in terms of B
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
pub struct CreatePassiveSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub price: Price,
}

impl ReadXdr for CreatePassiveSellOfferOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                selling: Asset::read_xdr(r)?,
                buying: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                price: Price::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreatePassiveSellOfferOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.selling.write_xdr(w)?;
            self.buying.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.price.write_xdr(w)?;
            Ok(())
        })
    }
}
