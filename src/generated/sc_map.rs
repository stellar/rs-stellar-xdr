#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScMap is an XDR Typedef defined as:
///
/// ```text
/// typedef SCMapEntry SCMap<>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
pub struct ScMap(pub VecM<ScMapEntry>);

impl From<ScMap> for VecM<ScMapEntry> {
    #[must_use]
    fn from(x: ScMap) -> Self {
        x.0
    }
}

impl From<VecM<ScMapEntry>> for ScMap {
    #[must_use]
    fn from(x: VecM<ScMapEntry>) -> Self {
        ScMap(x)
    }
}

impl AsRef<VecM<ScMapEntry>> for ScMap {
    #[must_use]
    fn as_ref(&self) -> &VecM<ScMapEntry> {
        &self.0
    }
}

impl ReadXdr for ScMap {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ScMapEntry>::read_xdr(r)?;
            let v = ScMap(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ScMap {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ScMap {
    type Target = VecM<ScMapEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScMap> for Vec<ScMapEntry> {
    #[must_use]
    fn from(x: ScMap) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ScMapEntry>> for ScMap {
    type Error = Error;
    fn try_from(x: Vec<ScMapEntry>) -> Result<Self, Error> {
        Ok(ScMap(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<ScMapEntry>> for ScMap {
    type Error = Error;
    fn try_from(x: &Vec<ScMapEntry>) -> Result<Self, Error> {
        Ok(ScMap(x.try_into()?))
    }
}

impl AsRef<Vec<ScMapEntry>> for ScMap {
    #[must_use]
    fn as_ref(&self) -> &Vec<ScMapEntry> {
        &self.0 .0
    }
}

impl AsRef<[ScMapEntry]> for ScMap {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ScMapEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ScMapEntry] {
        self.0 .0
    }
}
