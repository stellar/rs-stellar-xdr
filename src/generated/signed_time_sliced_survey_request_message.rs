#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignedTimeSlicedSurveyRequestMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyRequestMessage
/// {
///     Signature requestSignature;
///     TimeSlicedSurveyRequestMessage request;
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
pub struct SignedTimeSlicedSurveyRequestMessage {
    pub request_signature: Signature,
    pub request: TimeSlicedSurveyRequestMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                request_signature: Signature::read_xdr(r)?,
                request: TimeSlicedSurveyRequestMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.request_signature.write_xdr(w)?;
            self.request.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SignedTimeSlicedSurveyRequestMessageRef is a borrowing equivalent of [`SignedTimeSlicedSurveyRequestMessage`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedTimeSlicedSurveyRequestMessageRef<'a> {
    pub request_signature: SignatureRef<'a>,
    pub request: TimeSlicedSurveyRequestMessage,
}

#[cfg(feature = "alloc")]
impl From<&SignedTimeSlicedSurveyRequestMessageRef<'_>> for SignedTimeSlicedSurveyRequestMessage {
    #[must_use]
    fn from(v: &SignedTimeSlicedSurveyRequestMessageRef<'_>) -> Self {
        Self {
            request_signature: (&v.request_signature).into(),
            request: v.request.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SignedTimeSlicedSurveyRequestMessageRef<'_>> for SignedTimeSlicedSurveyRequestMessage {
    #[must_use]
    fn from(v: SignedTimeSlicedSurveyRequestMessageRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SignedTimeSlicedSurveyRequestMessageRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.request_signature.write_xdr(w)?;
            self.request.write_xdr(w)?;
            Ok(())
        })
    }
}
