#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignedTimeSlicedSurveyStartCollectingMessage is an XDR Struct defined as:
///
/// ```text
/// struct SignedTimeSlicedSurveyStartCollectingMessage
/// {
///     Signature signature;
///     TimeSlicedSurveyStartCollectingMessage startCollecting;
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
pub struct SignedTimeSlicedSurveyStartCollectingMessage {
    pub signature: Signature,
    pub start_collecting: TimeSlicedSurveyStartCollectingMessage,
}

impl ReadXdr for SignedTimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                signature: Signature::read_xdr(r)?,
                start_collecting: TimeSlicedSurveyStartCollectingMessage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignedTimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.signature.write_xdr(w)?;
            self.start_collecting.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SignedTimeSlicedSurveyStartCollectingMessageRef is a borrowing equivalent of [`SignedTimeSlicedSurveyStartCollectingMessage`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedTimeSlicedSurveyStartCollectingMessageRef<'a> {
    pub signature: SignatureRef<'a>,
    pub start_collecting: TimeSlicedSurveyStartCollectingMessage,
}

#[cfg(feature = "alloc")]
impl From<&SignedTimeSlicedSurveyStartCollectingMessageRef<'_>>
    for SignedTimeSlicedSurveyStartCollectingMessage
{
    #[must_use]
    fn from(v: &SignedTimeSlicedSurveyStartCollectingMessageRef<'_>) -> Self {
        Self {
            signature: (&v.signature).into(),
            start_collecting: v.start_collecting.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SignedTimeSlicedSurveyStartCollectingMessageRef<'_>>
    for SignedTimeSlicedSurveyStartCollectingMessage
{
    #[must_use]
    fn from(v: SignedTimeSlicedSurveyStartCollectingMessageRef<'_>) -> Self {
        Self::from(&v)
    }
}
