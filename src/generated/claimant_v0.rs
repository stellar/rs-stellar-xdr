#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClaimantV0 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID destination;    // The account that can use this condition
///         ClaimPredicate predicate; // Claimable if predicate is true
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
pub struct ClaimantV0 {
    pub destination: AccountId,
    pub predicate: ClaimPredicate,
}

impl ReadXdr for ClaimantV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                destination: AccountId::read_xdr(r)?,
                predicate: ClaimPredicate::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimantV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.destination.write_xdr(w)?;
            self.predicate.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ClaimantV0View is a borrowing equivalent of [`ClaimantV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimantV0View<'a> {
    pub destination: AccountId,
    pub predicate: ClaimPredicateView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ClaimantV0View<'_> {
    type Owned = ClaimantV0;
    fn into_owned(self) -> ClaimantV0 {
        ClaimantV0 {
            destination: self.destination.into_owned(),
            predicate: self.predicate.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ClaimantV0View<'_>> for ClaimantV0 {
    #[must_use]
    fn from(v: &ClaimantV0View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ClaimantV0View<'_>> for ClaimantV0 {
    #[must_use]
    fn from(v: ClaimantV0View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ClaimantV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.destination.write_xdr(w)?;
            self.predicate.write_xdr(w)?;
            Ok(())
        })
    }
}
