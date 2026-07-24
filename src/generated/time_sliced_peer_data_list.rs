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
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for TimeSlicedPeerDataList {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
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
