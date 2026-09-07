#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictSendResultSuccess is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         ClaimAtom offers<>;
///         SimplePaymentResult last;
///     }
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
pub struct PathPaymentStrictSendResultSuccess {
    pub offers: VecM<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXdr for PathPaymentStrictSendResultSuccess {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                offers: VecM::<ClaimAtom>::read_xdr(r)?,
                last: SimplePaymentResult::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PathPaymentStrictSendResultSuccess {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers.write_xdr(w)?;
            self.last.write_xdr(w)?;
            Ok(())
        })
    }
}

/// PathPaymentStrictSendResultSuccessView is a borrowing equivalent of [`PathPaymentStrictSendResultSuccess`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictSendResultSuccessView<'a> {
    pub offers: VecMView<'a, ClaimAtom>,
    pub last: SimplePaymentResult,
}

#[cfg(feature = "alloc")]
impl IntoOwned for PathPaymentStrictSendResultSuccessView<'_> {
    type Owned = PathPaymentStrictSendResultSuccess;
    fn into_owned(self) -> PathPaymentStrictSendResultSuccess {
        PathPaymentStrictSendResultSuccess {
            offers: self.offers.into_owned(),
            last: self.last.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&PathPaymentStrictSendResultSuccessView<'_>> for PathPaymentStrictSendResultSuccess {
    #[must_use]
    fn from(v: &PathPaymentStrictSendResultSuccessView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<PathPaymentStrictSendResultSuccessView<'_>> for PathPaymentStrictSendResultSuccess {
    #[must_use]
    fn from(v: PathPaymentStrictSendResultSuccessView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for PathPaymentStrictSendResultSuccessView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers.write_xdr(w)?;
            self.last.write_xdr(w)?;
            Ok(())
        })
    }
}
