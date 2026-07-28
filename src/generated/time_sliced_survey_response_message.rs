#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TimeSlicedSurveyResponseMessage is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedSurveyResponseMessage
/// {
///     SurveyResponseMessage response;
///     uint32 nonce;
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
pub struct TimeSlicedSurveyResponseMessage {
    pub response: SurveyResponseMessage,
    pub nonce: u32,
}

impl ReadXdr for TimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                response: SurveyResponseMessage::read_xdr(r)?,
                nonce: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeSlicedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.response.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TimeSlicedSurveyResponseMessageRef is a borrowing equivalent of [`TimeSlicedSurveyResponseMessage`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSlicedSurveyResponseMessageRef<'a> {
    pub response: SurveyResponseMessageRef<'a>,
    pub nonce: u32,
}

#[cfg(feature = "alloc")]
impl From<&TimeSlicedSurveyResponseMessageRef<'_>> for TimeSlicedSurveyResponseMessage {
    #[must_use]
    fn from(v: &TimeSlicedSurveyResponseMessageRef<'_>) -> Self {
        Self {
            response: (&v.response).into(),
            nonce: v.nonce,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TimeSlicedSurveyResponseMessageRef<'_>> for TimeSlicedSurveyResponseMessage {
    #[must_use]
    fn from(v: TimeSlicedSurveyResponseMessageRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TimeSlicedSurveyResponseMessageRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.response.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            Ok(())
        })
    }
}
