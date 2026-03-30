#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Duration is an XDR Typedef defined as:
///
/// ```text
/// typedef uint64 Duration;
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
pub struct Duration(
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub u64,
);


impl From<Duration> for u64 {
    #[must_use]
    fn from(x: Duration) -> Self {
        x.0
    }
}

impl From<u64> for Duration {
    #[must_use]
    fn from(x: u64) -> Self {
        Duration(x)
    }
}

impl AsRef<u64> for Duration {
    #[must_use]
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl ReadXdr for Duration {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = u64::read_xdr(r)?;
            let v = Duration(i);
            Ok(v)
        })
    }
}

impl WriteXdr for Duration {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| { self.0.write_xdr(w) })
    }
}
