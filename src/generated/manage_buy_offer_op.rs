#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageBuyOfferOp is an XDR Struct defined as:
///
/// ```text
/// struct ManageBuyOfferOp
/// {
///     Asset selling;
///     Asset buying;
///     int64 buyAmount; // amount being bought. if set to 0, delete the offer
///     Price price;     // price of thing being bought in terms of what you are
///                      // selling
/// 
///     // 0=create a new offer, otherwise edit an existing offer
///     int64 offerID;
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
pub struct ManageBuyOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub buy_amount: i64,
    pub price: Price,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub offer_id: i64,
}

impl ReadXdr for ManageBuyOfferOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                selling: Asset::read_xdr(r)?,
                buying: Asset::read_xdr(r)?,
                buy_amount: i64::read_xdr(r)?,
                price: Price::read_xdr(r)?,
                offer_id: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ManageBuyOfferOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.selling.write_xdr(w)?;
            self.buying.write_xdr(w)?;
            self.buy_amount.write_xdr(w)?;
            self.price.write_xdr(w)?;
            self.offer_id.write_xdr(w)?;
            Ok(())
        })
    }
}
