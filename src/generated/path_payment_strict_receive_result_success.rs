#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictReceiveResultSuccess is an XDR NestedStruct defined as:
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
pub struct PathPaymentStrictReceiveResultSuccess {
    pub offers: VecM<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXdr for PathPaymentStrictReceiveResultSuccess {
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

impl WriteXdr for PathPaymentStrictReceiveResultSuccess {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.offers.write_xdr(w)?;
            self.last.write_xdr(w)?;
            Ok(())
        })
    }
}

/// PathPaymentStrictReceiveResultSuccessRef is a borrowing equivalent of [`PathPaymentStrictReceiveResultSuccess`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictReceiveResultSuccessRef<'a> {
    pub offers: VecMRef<'a, ClaimAtom>,
    pub last: SimplePaymentResult,
}

#[cfg(feature = "alloc")]
impl From<&PathPaymentStrictReceiveResultSuccessRef<'_>> for PathPaymentStrictReceiveResultSuccess {
    #[must_use]
    fn from(v: &PathPaymentStrictReceiveResultSuccessRef<'_>) -> Self {
        Self {
            offers: v.offers.to_vecm(),
            last: v.last.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<PathPaymentStrictReceiveResultSuccessRef<'_>> for PathPaymentStrictReceiveResultSuccess {
    #[must_use]
    fn from(v: PathPaymentStrictReceiveResultSuccessRef<'_>) -> Self {
        Self::from(&v)
    }
}
