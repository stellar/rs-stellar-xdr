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
impl IntoOwned for SignedTimeSlicedSurveyRequestMessageRef<'_> {
    type Owned = SignedTimeSlicedSurveyRequestMessage;
    fn into_owned(self) -> SignedTimeSlicedSurveyRequestMessage {
        SignedTimeSlicedSurveyRequestMessage {
            request_signature: self.request_signature.into_owned(),
            request: self.request.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SignedTimeSlicedSurveyRequestMessageRef<'_>> for SignedTimeSlicedSurveyRequestMessage {
    #[must_use]
    fn from(v: &SignedTimeSlicedSurveyRequestMessageRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SignedTimeSlicedSurveyRequestMessageRef<'_>> for SignedTimeSlicedSurveyRequestMessage {
    #[must_use]
    fn from(v: SignedTimeSlicedSurveyRequestMessageRef<'_>) -> Self {
        v.into_owned()
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

#[cfg(feature = "const")]
impl SignedTimeSlicedSurveyRequestMessageRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_signed_time_sliced_survey_request_message(self);
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
        w.write_type_signed_time_sliced_survey_request_message(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SignedTimeSlicedSurveyRequestMessage`], mirroring `<SignedTimeSlicedSurveyRequestMessage as WriteXdr>::write_xdr`.
    pub const fn write_type_signed_time_sliced_survey_request_message(
        &mut self,
        v: &SignedTimeSlicedSurveyRequestMessageRef<'_>,
    ) {
        self.write_type_signature(&v.request_signature);
        self.write_type_time_sliced_survey_request_message(&v.request);
    }
}
