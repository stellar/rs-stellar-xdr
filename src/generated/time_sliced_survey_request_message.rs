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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl TimeSlicedSurveyRequestMessage {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.request.const_write_xdr(w);
        w.write_u32(self.nonce);
        w.write_u32(self.inbound_peers_index);
        w.write_u32(self.outbound_peers_index);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
