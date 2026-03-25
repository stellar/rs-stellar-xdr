#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// TimeBounds is an XDR Struct defined as:
///
/// ```text
/// struct TimeBounds
/// {
///     TimePoint minTime;
///     TimePoint maxTime; // 0 here means no maxTime
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TimeBounds {
    pub min_time: TimePoint,
    pub max_time: TimePoint,
}

impl ReadXdr for TimeBounds {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                min_time: TimePoint::read_xdr(r)?,
                max_time: TimePoint::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeBounds {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.min_time.write_xdr(w)?;
            self.max_time.write_xdr(w)?;
            Ok(())
        })
    }
}
