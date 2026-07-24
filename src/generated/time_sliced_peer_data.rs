#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimeSlicedPeerData is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedPeerData
/// {
///     PeerStats peerStats;
///     uint32 averageLatencyMs;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TimeSlicedPeerData {
    pub peer_stats: PeerStats,
    pub average_latency_ms: u32,
}

impl ReadXdr for TimeSlicedPeerData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                peer_stats: PeerStats::read_xdr(r)?,
                average_latency_ms: u32::read_xdr(r)?,
            })
        })
    }
}

impl TimeSlicedPeerData {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.peer_stats.const_to_xdr(w);
        w.write_u32(self.average_latency_ms);
        w.leave_depth();
    }
}

impl WriteXdr for TimeSlicedPeerData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.peer_stats.write_xdr(w)?;
            self.average_latency_ms.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
