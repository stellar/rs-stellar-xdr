#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SurveyResponseMessage is an XDR Struct defined as:
///
/// ```text
/// struct SurveyResponseMessage
/// {
///     NodeID surveyorPeerID;
///     NodeID surveyedPeerID;
///     uint32 ledgerNum;
///     SurveyMessageCommandType commandType;
///     EncryptedBody encryptedBody;
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
pub struct SurveyResponseMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub command_type: SurveyMessageCommandType,
    pub encrypted_body: EncryptedBody,
}

impl ReadXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                surveyor_peer_id: NodeId::read_xdr(r)?,
                surveyed_peer_id: NodeId::read_xdr(r)?,
                ledger_num: u32::read_xdr(r)?,
                command_type: SurveyMessageCommandType::read_xdr(r)?,
                encrypted_body: EncryptedBody::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.surveyor_peer_id.write_xdr(w)?;
            self.surveyed_peer_id.write_xdr(w)?;
            self.ledger_num.write_xdr(w)?;
            self.command_type.write_xdr(w)?;
            self.encrypted_body.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SurveyResponseMessageView is a borrowing equivalent of [`SurveyResponseMessage`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyResponseMessageView<'a> {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub command_type: SurveyMessageCommandType,
    pub encrypted_body: EncryptedBodyView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&SurveyResponseMessageView<'_>> for SurveyResponseMessage {
    #[must_use]
    fn from(v: &SurveyResponseMessageView<'_>) -> Self {
        Self {
            surveyor_peer_id: v.surveyor_peer_id.clone(),
            surveyed_peer_id: v.surveyed_peer_id.clone(),
            ledger_num: v.ledger_num,
            command_type: v.command_type,
            encrypted_body: (&v.encrypted_body).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SurveyResponseMessageView<'_>> for SurveyResponseMessage {
    #[must_use]
    fn from(v: SurveyResponseMessageView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SurveyResponseMessageView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.surveyor_peer_id.write_xdr(w)?;
            self.surveyed_peer_id.write_xdr(w)?;
            self.ledger_num.write_xdr(w)?;
            self.command_type.write_xdr(w)?;
            self.encrypted_body.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SurveyResponseMessageView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_survey_response_message(self);
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
        w.write_type_survey_response_message(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SurveyResponseMessage`], mirroring `<SurveyResponseMessage as WriteXdr>::write_xdr`.
    pub const fn write_type_survey_response_message(&mut self, v: &SurveyResponseMessageView<'_>) {
        self.write_type_node_id(&v.surveyor_peer_id);
        self.write_type_node_id(&v.surveyed_peer_id);
        self.write_u32(v.ledger_num);
        self.write_type_survey_message_command_type(&v.command_type);
        self.write_type_encrypted_body(&v.encrypted_body);
    }
}
