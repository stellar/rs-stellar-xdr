#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimePointMilliseconds is an XDR Typedef defined as:
///
/// ```text
/// typedef uint64 TimePointMilliseconds;
/// ```
///
#[cfg(feature = "ms_close_time")]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
pub struct TimePointMilliseconds(
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub u64,
);

#[cfg(feature = "ms_close_time")]
impl From<TimePointMilliseconds> for u64 {
    #[must_use]
    fn from(x: TimePointMilliseconds) -> Self {
        x.0
    }
}

#[cfg(feature = "ms_close_time")]
impl From<u64> for TimePointMilliseconds {
    #[must_use]
    fn from(x: u64) -> Self {
        TimePointMilliseconds(x)
    }
}

#[cfg(feature = "ms_close_time")]
impl AsRef<u64> for TimePointMilliseconds {
    #[must_use]
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

#[cfg(feature = "ms_close_time")]
impl ReadXdr for TimePointMilliseconds {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = u64::read_xdr(r)?;
            let v = TimePointMilliseconds(i);
            Ok(v)
        })
    }
}

#[cfg(feature = "ms_close_time")]
impl WriteXdr for TimePointMilliseconds {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}
