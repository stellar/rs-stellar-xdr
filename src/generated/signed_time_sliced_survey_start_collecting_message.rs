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
impl IntoOwned for SignedTimeSlicedSurveyStartCollectingMessageRef<'_> {
    type Owned = SignedTimeSlicedSurveyStartCollectingMessage;
    fn into_owned(self) -> SignedTimeSlicedSurveyStartCollectingMessage {
        SignedTimeSlicedSurveyStartCollectingMessage {
            signature: self.signature.into_owned(),
            start_collecting: self.start_collecting.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SignedTimeSlicedSurveyStartCollectingMessageRef<'_>>
    for SignedTimeSlicedSurveyStartCollectingMessage
{
    #[must_use]
    fn from(v: &SignedTimeSlicedSurveyStartCollectingMessageRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SignedTimeSlicedSurveyStartCollectingMessageRef<'_>>
    for SignedTimeSlicedSurveyStartCollectingMessage
{
    #[must_use]
    fn from(v: SignedTimeSlicedSurveyStartCollectingMessageRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SignedTimeSlicedSurveyStartCollectingMessageRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.signature.write_xdr(w)?;
            self.start_collecting.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SignedTimeSlicedSurveyStartCollectingMessageRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_signed_time_sliced_survey_start_collecting_message(self);
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
        w.write_type_signed_time_sliced_survey_start_collecting_message(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SignedTimeSlicedSurveyStartCollectingMessage`], mirroring `<SignedTimeSlicedSurveyStartCollectingMessage as WriteXdr>::write_xdr`.
    pub const fn write_type_signed_time_sliced_survey_start_collecting_message(
        &mut self,
        v: &SignedTimeSlicedSurveyStartCollectingMessageRef<'_>,
    ) {
        self.write_type_signature(&v.signature);
        self.write_type_time_sliced_survey_start_collecting_message(&v.start_collecting);
    }
}
