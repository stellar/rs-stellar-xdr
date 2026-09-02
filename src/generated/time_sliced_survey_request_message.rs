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

#[cfg(feature = "alloc")]
impl IntoOwned for TimeSlicedSurveyRequestMessage {
    type Owned = TimeSlicedSurveyRequestMessage;
    fn into_owned(self) -> TimeSlicedSurveyRequestMessage {
        self
    }
}

#[cfg(feature = "const")]
impl TimeSlicedSurveyRequestMessage {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_time_sliced_survey_request_message(self);
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
        w.write_type_time_sliced_survey_request_message(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TimeSlicedSurveyRequestMessage`], mirroring `<TimeSlicedSurveyRequestMessage as WriteXdr>::write_xdr`.
    pub const fn write_type_time_sliced_survey_request_message(
        &mut self,
        v: &TimeSlicedSurveyRequestMessage,
    ) {
        self.write_type_survey_request_message(&v.request);
        self.write_u32(v.nonce);
        self.write_u32(v.inbound_peers_index);
        self.write_u32(v.outbound_peers_index);
    }
}
