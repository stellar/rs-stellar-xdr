#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OfferEntry is an XDR Struct defined as:
///
/// ```text
/// struct OfferEntry
/// {
///     AccountID sellerID;
///     int64 offerID;
///     Asset selling; // A
///     Asset buying;  // B
///     int64 amount;  // amount of A
/// 
///     /* price for this offer:
///         price of A in terms of B
///         price=AmountB/AmountA=priceNumerator/priceDenominator
///         price is after fees
///     */
///     Price price;
///     uint32 flags; // see OfferEntryFlags
/// 
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
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
pub struct OfferEntry {
    pub seller_id: AccountId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub offer_id: i64,
    pub selling: Asset,
    pub buying: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub price: Price,
    pub flags: u32,
    pub ext: OfferEntryExt,
}

impl ReadXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                seller_id: AccountId::read_xdr(r)?,
                offer_id: i64::read_xdr(r)?,
                selling: Asset::read_xdr(r)?,
                buying: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                price: Price::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                ext: OfferEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.seller_id.write_xdr(w)?;
            self.offer_id.write_xdr(w)?;
            self.selling.write_xdr(w)?;
            self.buying.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.price.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
