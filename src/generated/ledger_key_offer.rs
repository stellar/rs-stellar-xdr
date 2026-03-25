#[allow(unused_imports)]
use super::*;
/// LedgerKeyOffer is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID sellerID;
///         int64 offerID;
///     }
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
pub struct LedgerKeyOffer {
    pub seller_id: AccountId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub offer_id: i64,
}

impl ReadXdr for LedgerKeyOffer {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                seller_id: AccountId::read_xdr(r)?,
                offer_id: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyOffer {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.seller_id.write_xdr(w)?;
            self.offer_id.write_xdr(w)?;
            Ok(())
        })
    }
}
