#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageOfferSuccessResult is an XDR Struct defined as:
///
/// ```text
/// struct ManageOfferSuccessResult
/// {
///     // offers that got claimed while creating this offer
///     ClaimAtom offersClaimed<>;
///
///     union switch (ManageOfferEffect effect)
///     {
///     case MANAGE_OFFER_CREATED:
///     case MANAGE_OFFER_UPDATED:
///         OfferEntry offer;
///     case MANAGE_OFFER_DELETED:
///         void;
///     }
///     offer;
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
pub struct ManageOfferSuccessResult {
    pub offers_claimed: VecM<ClaimAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

impl ReadXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                offers_claimed: VecM::<ClaimAtom>::read_xdr(r)?,
                offer: ManageOfferSuccessResultOffer::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers_claimed.write_xdr(w)?;
            self.offer.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ManageOfferSuccessResultView is a borrowing equivalent of [`ManageOfferSuccessResult`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageOfferSuccessResultView<'a> {
    pub offers_claimed: VecMView<'a, ClaimAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ManageOfferSuccessResultView<'_> {
    type Owned = ManageOfferSuccessResult;
    fn into_owned(self) -> ManageOfferSuccessResult {
        ManageOfferSuccessResult {
            offers_claimed: self.offers_claimed.into_owned(),
            offer: self.offer.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ManageOfferSuccessResultView<'_>> for ManageOfferSuccessResult {
    #[must_use]
    fn from(v: &ManageOfferSuccessResultView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ManageOfferSuccessResultView<'_>> for ManageOfferSuccessResult {
    #[must_use]
    fn from(v: ManageOfferSuccessResultView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ManageOfferSuccessResultView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers_claimed.write_xdr(w)?;
            self.offer.write_xdr(w)?;
            Ok(())
        })
    }
}
