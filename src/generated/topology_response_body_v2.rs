#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TopologyResponseBodyV2 is an XDR Struct defined as:
///
/// ```text
/// struct TopologyResponseBodyV2
/// {
///     TimeSlicedPeerDataList inboundPeers;
///     TimeSlicedPeerDataList outboundPeers;
///     TimeSlicedNodeData nodeData;
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
pub struct TopologyResponseBodyV2 {
    pub inbound_peers: TimeSlicedPeerDataList,
    pub outbound_peers: TimeSlicedPeerDataList,
    pub node_data: TimeSlicedNodeData,
}

impl ReadXdr for TopologyResponseBodyV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                inbound_peers: TimeSlicedPeerDataList::read_xdr(r)?,
                outbound_peers: TimeSlicedPeerDataList::read_xdr(r)?,
                node_data: TimeSlicedNodeData::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TopologyResponseBodyV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.inbound_peers.write_xdr(w)?;
            self.outbound_peers.write_xdr(w)?;
            self.node_data.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TopologyResponseBodyV2Const is a borrowing equivalent of [`TopologyResponseBodyV2`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyResponseBodyV2Const {
    pub inbound_peers: TimeSlicedPeerDataListConst,
    pub outbound_peers: TimeSlicedPeerDataListConst,
    pub node_data: TimeSlicedNodeData,
}

#[cfg(feature = "const")]
impl TopologyResponseBodyV2Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_topology_response_body_v2(self);
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
        w.write_type_topology_response_body_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TopologyResponseBodyV2`], mirroring `<TopologyResponseBodyV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_topology_response_body_v2(&mut self, v: &TopologyResponseBodyV2Const) {
        self.write_type_time_sliced_peer_data_list(&v.inbound_peers);
        self.write_type_time_sliced_peer_data_list(&v.outbound_peers);
        self.write_type_time_sliced_node_data(&v.node_data);
    }
}
