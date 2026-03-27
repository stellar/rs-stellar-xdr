#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizationEntries is an XDR Typedef defined as:
///
/// ```text
/// typedef SorobanAuthorizationEntry SorobanAuthorizationEntries<>;
/// ```
///
#[cfg_eval::cfg_eval]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct SorobanAuthorizationEntries(pub VecM<SorobanAuthorizationEntry>);

impl From<SorobanAuthorizationEntries> for VecM<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0
    }
}

impl From<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn from(x: VecM<SorobanAuthorizationEntry>) -> Self {
        SorobanAuthorizationEntries(x)
    }
}

impl AsRef<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &VecM<SorobanAuthorizationEntry> {
        &self.0
    }
}

impl ReadXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<SorobanAuthorizationEntry>::read_xdr(r)?;
            let v = SorobanAuthorizationEntries(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for SorobanAuthorizationEntries {
    type Target = VecM<SorobanAuthorizationEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SorobanAuthorizationEntries> for Vec<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: &Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

impl AsRef<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &Vec<SorobanAuthorizationEntry> {
        &self.0 .0
    }
}

impl AsRef<[SorobanAuthorizationEntry]> for SorobanAuthorizationEntries {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        self.0 .0
    }
}
