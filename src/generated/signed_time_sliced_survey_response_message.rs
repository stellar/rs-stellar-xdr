#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignedTimeSlicedSurveyResponseMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyResponseMessage
/// {
///     Signature responseSignature;
///     TimeSlicedSurveyResponseMessage response;
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
pub struct SignedTimeSlicedSurveyResponseMessage {
    pub response_signature: Signature,
    pub response: TimeSlicedSurveyResponseMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                response_signature: Signature::read_xdr(r)?,
                response: TimeSlicedSurveyResponseMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.response_signature.write_xdr(w)?;
            self.response.write_xdr(w)?;
            Ok(())
        })
    }
}
