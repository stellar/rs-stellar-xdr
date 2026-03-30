#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SponsorshipDescriptor is an XDR Typedef defined as:
///
/// ```text
/// typedef AccountID* SponsorshipDescriptor;
/// ```
///
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct SponsorshipDescriptor(pub Option<AccountId>);


impl From<SponsorshipDescriptor> for Option<AccountId> {
    #[must_use]
    fn from(x: SponsorshipDescriptor) -> Self {
        x.0
    }
}

impl From<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn from(x: Option<AccountId>) -> Self {
        SponsorshipDescriptor(x)
    }
}

impl AsRef<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn as_ref(&self) -> &Option<AccountId> {
        &self.0
    }
}

impl ReadXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Option::<AccountId>::read_xdr(r)?;
            let v = SponsorshipDescriptor(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| { self.0.write_xdr(w) })
    }
}
