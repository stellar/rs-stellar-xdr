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

/// TimeSlicedPeerDataListConst is a borrowing equivalent of [`TimeSlicedPeerDataList`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSlicedPeerDataListConst(pub VecMConst<TimeSlicedPeerDataConst, 25>);

#[cfg(feature = "const")]
impl TimeSlicedPeerDataListConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_time_sliced_peer_data_list(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_time_sliced_peer_data_list(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TimeSlicedPeerDataList`], mirroring `<TimeSlicedPeerDataList as WriteXdr>::write_xdr`.
    pub const fn write_type_time_sliced_peer_data_list(&mut self, v: &TimeSlicedPeerDataListConst) {
        self.write_type_vec_time_sliced_peer_data(&v.0);
    }
}
