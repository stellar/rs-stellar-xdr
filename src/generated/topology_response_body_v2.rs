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

/// TopologyResponseBodyV2Ref is a borrowing equivalent of [`TopologyResponseBodyV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyResponseBodyV2Ref<'a> {
    pub inbound_peers: TimeSlicedPeerDataListRef<'a>,
    pub outbound_peers: TimeSlicedPeerDataListRef<'a>,
    pub node_data: TimeSlicedNodeData,
}

#[cfg(feature = "alloc")]
impl From<&TopologyResponseBodyV2Ref<'_>> for TopologyResponseBodyV2 {
    #[must_use]
    fn from(v: &TopologyResponseBodyV2Ref<'_>) -> Self {
        Self {
            inbound_peers: (&v.inbound_peers).into(),
            outbound_peers: (&v.outbound_peers).into(),
            node_data: v.node_data.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TopologyResponseBodyV2Ref<'_>> for TopologyResponseBodyV2 {
    #[must_use]
    fn from(v: TopologyResponseBodyV2Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TopologyResponseBodyV2Ref<'_> {
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
