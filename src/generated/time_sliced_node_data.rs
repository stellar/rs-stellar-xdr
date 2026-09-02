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
}

#[cfg(feature = "alloc")]
impl IntoOwned for TimeSlicedNodeData {
    type Owned = TimeSlicedNodeData;
    fn into_owned(self) -> TimeSlicedNodeData {
        self
    }
}

#[cfg(feature = "const")]
impl TimeSlicedNodeData {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_time_sliced_node_data(self);
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
        w.write_type_time_sliced_node_data(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TimeSlicedNodeData`], mirroring `<TimeSlicedNodeData as WriteXdr>::write_xdr`.
    pub const fn write_type_time_sliced_node_data(&mut self, v: &TimeSlicedNodeData) {
        self.write_u32(v.added_authenticated_peers);
        self.write_u32(v.dropped_authenticated_peers);
        self.write_u32(v.total_inbound_peer_count);
        self.write_u32(v.total_outbound_peer_count);
        self.write_u32(v.p75_scp_first_to_self_latency_ms);
        self.write_u32(v.p75_scp_self_to_other_latency_ms);
        self.write_u32(v.lost_sync_count);
        self.write_bool(v.is_validator);
        self.write_u32(v.max_inbound_peer_count);
        self.write_u32(v.max_outbound_peer_count);
    }
}
