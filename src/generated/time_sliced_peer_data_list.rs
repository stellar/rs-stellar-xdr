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

impl TimeSlicedPeerDataList {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
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
