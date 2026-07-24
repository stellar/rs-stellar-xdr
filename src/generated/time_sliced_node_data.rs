#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimeSlicedNodeData is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedNodeData
/// {
///     uint32 addedAuthenticatedPeers;
///     uint32 droppedAuthenticatedPeers;
///     uint32 totalInboundPeerCount;
///     uint32 totalOutboundPeerCount;
///
///     // SCP stats
///     uint32 p75SCPFirstToSelfLatencyMs;
///     uint32 p75SCPSelfToOtherLatencyMs;
///
///     // How many times the node lost sync in the time slice
///     uint32 lostSyncCount;
///
///     // Config data
///     bool isValidator;
///     uint32 maxInboundPeerCount;
///     uint32 maxOutboundPeerCount;
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
pub struct TimeSlicedNodeData {
    pub added_authenticated_peers: u32,
    pub dropped_authenticated_peers: u32,
    pub total_inbound_peer_count: u32,
    pub total_outbound_peer_count: u32,
    pub p75_scp_first_to_self_latency_ms: u32,
    pub p75_scp_self_to_other_latency_ms: u32,
    pub lost_sync_count: u32,
    pub is_validator: bool,
    pub max_inbound_peer_count: u32,
    pub max_outbound_peer_count: u32,
}

impl ReadXdr for TimeSlicedNodeData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                added_authenticated_peers: u32::read_xdr(r)?,
                dropped_authenticated_peers: u32::read_xdr(r)?,
                total_inbound_peer_count: u32::read_xdr(r)?,
                total_outbound_peer_count: u32::read_xdr(r)?,
                p75_scp_first_to_self_latency_ms: u32::read_xdr(r)?,
                p75_scp_self_to_other_latency_ms: u32::read_xdr(r)?,
                lost_sync_count: u32::read_xdr(r)?,
                is_validator: bool::read_xdr(r)?,
                max_inbound_peer_count: u32::read_xdr(r)?,
                max_outbound_peer_count: u32::read_xdr(r)?,
            })
        })
    }
}

impl TimeSlicedNodeData {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.added_authenticated_peers);
        w.write_u32(self.dropped_authenticated_peers);
        w.write_u32(self.total_inbound_peer_count);
        w.write_u32(self.total_outbound_peer_count);
        w.write_u32(self.p75_scp_first_to_self_latency_ms);
        w.write_u32(self.p75_scp_self_to_other_latency_ms);
        w.write_u32(self.lost_sync_count);
        w.write_bool(self.is_validator);
        w.write_u32(self.max_inbound_peer_count);
        w.write_u32(self.max_outbound_peer_count);
        w.leave_depth();
    }
}

impl WriteXdr for TimeSlicedNodeData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.added_authenticated_peers.write_xdr(w)?;
            self.dropped_authenticated_peers.write_xdr(w)?;
            self.total_inbound_peer_count.write_xdr(w)?;
            self.total_outbound_peer_count.write_xdr(w)?;
            self.p75_scp_first_to_self_latency_ms.write_xdr(w)?;
            self.p75_scp_self_to_other_latency_ms.write_xdr(w)?;
            self.lost_sync_count.write_xdr(w)?;
            self.is_validator.write_xdr(w)?;
            self.max_inbound_peer_count.write_xdr(w)?;
            self.max_outbound_peer_count.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
