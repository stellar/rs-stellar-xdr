#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimeSlicedPeerDataList is an XDR Typedef defined as:
///
/// ```text
/// typedef TimeSlicedPeerData TimeSlicedPeerDataList<25>;
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
pub struct TimeSlicedPeerDataList(pub VecM<TimeSlicedPeerData, 25>);

impl From<TimeSlicedPeerDataList> for VecM<TimeSlicedPeerData, 25> {
    #[must_use]
    fn from(x: TimeSlicedPeerDataList) -> Self {
        x.0
    }
}

impl From<VecM<TimeSlicedPeerData, 25>> for TimeSlicedPeerDataList {
    #[must_use]
    fn from(x: VecM<TimeSlicedPeerData, 25>) -> Self {
        TimeSlicedPeerDataList(x)
    }
}

impl AsRef<VecM<TimeSlicedPeerData, 25>> for TimeSlicedPeerDataList {
    #[must_use]
    fn as_ref(&self) -> &VecM<TimeSlicedPeerData, 25> {
        &self.0
    }
}

impl ReadXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<TimeSlicedPeerData, 25>::read_xdr(r)?;
            let v = TimeSlicedPeerDataList(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for TimeSlicedPeerDataList {
    type Target = VecM<TimeSlicedPeerData, 25>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TimeSlicedPeerDataList> for Vec<TimeSlicedPeerData> {
    #[must_use]
    fn from(x: TimeSlicedPeerDataList) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(x: Vec<TimeSlicedPeerData>) -> Result<Self, Error> {
        Ok(TimeSlicedPeerDataList(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    type Error = Error;
    fn try_from(x: &Vec<TimeSlicedPeerData>) -> Result<Self, Error> {
        Ok(TimeSlicedPeerDataList(x.try_into()?))
    }
}

impl AsRef<Vec<TimeSlicedPeerData>> for TimeSlicedPeerDataList {
    #[must_use]
    fn as_ref(&self) -> &Vec<TimeSlicedPeerData> {
        &self.0 .0
    }
}

impl AsRef<[TimeSlicedPeerData]> for TimeSlicedPeerDataList {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[TimeSlicedPeerData] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[TimeSlicedPeerData] {
        self.0 .0
    }
}
