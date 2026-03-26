#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimeSlicedSurveyRequestMessage is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedSurveyRequestMessage
/// {
///     SurveyRequestMessage request;
///     uint32 nonce;
///     uint32 inboundPeersIndex;
///     uint32 outboundPeersIndex;
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
pub struct TimeSlicedSurveyRequestMessage {
    pub request: SurveyRequestMessage,
    pub nonce: u32,
    pub inbound_peers_index: u32,
    pub outbound_peers_index: u32,
}

impl ReadXdr for TimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                request: SurveyRequestMessage::read_xdr(r)?,
                nonce: u32::read_xdr(r)?,
                inbound_peers_index: u32::read_xdr(r)?,
                outbound_peers_index: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.request.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.inbound_peers_index.write_xdr(w)?;
            self.outbound_peers_index.write_xdr(w)?;
            Ok(())
        })
    }
}
