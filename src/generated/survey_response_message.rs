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
impl IntoOwned for SurveyResponseMessageView<'_> {
    type Owned = SurveyResponseMessage;
    fn into_owned(&self) -> SurveyResponseMessage {
        SurveyResponseMessage {
            surveyor_peer_id: IntoOwned::into_owned(&self.surveyor_peer_id),
            surveyed_peer_id: IntoOwned::into_owned(&self.surveyed_peer_id),
            ledger_num: IntoOwned::into_owned(&self.ledger_num),
            command_type: IntoOwned::into_owned(&self.command_type),
            encrypted_body: IntoOwned::into_owned(&self.encrypted_body),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SurveyResponseMessageView<'_>> for SurveyResponseMessage {
    #[must_use]
    fn from(v: &SurveyResponseMessageView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SurveyResponseMessageView<'_>> for SurveyResponseMessage {
    #[must_use]
    fn from(v: SurveyResponseMessageView<'_>) -> Self {
        IntoOwned::into_owned(&v)
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
