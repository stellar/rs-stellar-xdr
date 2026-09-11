#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StellarMessage is a borrowing equivalent of [`StellarMessage`](super::super::StellarMessage)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum StellarMessage {
    ErrorMsg(SError),
    Hello(Hello),
    Auth(Auth),
    DontHave(DontHave),
    Peers(VecM<PeerAddress, 100>),
    GetTxSet(Uint256),
    TxSet(TransactionSet),
    GeneralizedTxSet(GeneralizedTransactionSet),
    Transaction(TransactionEnvelope),
    TimeSlicedSurveyRequest(SignedTimeSlicedSurveyRequestMessage),
    TimeSlicedSurveyResponse(SignedTimeSlicedSurveyResponseMessage),
    TimeSlicedSurveyStartCollecting(SignedTimeSlicedSurveyStartCollectingMessage),
    TimeSlicedSurveyStopCollecting(SignedTimeSlicedSurveyStopCollectingMessage),
    GetScpQuorumset(Uint256),
    ScpQuorumset(ScpQuorumSet),
    ScpMessage(ScpEnvelope),
    GetScpState(u32),
    SendMore(SendMore),
    SendMoreExtended(SendMoreExtended),
    FloodAdvert(FloodAdvert),
    FloodDemand(FloodDemand),
}

impl StellarMessage {
    #[must_use]
    pub const fn discriminant(&self) -> MessageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg(_) => MessageType::ErrorMsg,
            Self::Hello(_) => MessageType::Hello,
            Self::Auth(_) => MessageType::Auth,
            Self::DontHave(_) => MessageType::DontHave,
            Self::Peers(_) => MessageType::Peers,
            Self::GetTxSet(_) => MessageType::GetTxSet,
            Self::TxSet(_) => MessageType::TxSet,
            Self::GeneralizedTxSet(_) => MessageType::GeneralizedTxSet,
            Self::Transaction(_) => MessageType::Transaction,
            Self::TimeSlicedSurveyRequest(_) => MessageType::TimeSlicedSurveyRequest,
            Self::TimeSlicedSurveyResponse(_) => MessageType::TimeSlicedSurveyResponse,
            Self::TimeSlicedSurveyStartCollecting(_) => {
                MessageType::TimeSlicedSurveyStartCollecting
            }
            Self::TimeSlicedSurveyStopCollecting(_) => MessageType::TimeSlicedSurveyStopCollecting,
            Self::GetScpQuorumset(_) => MessageType::GetScpQuorumset,
            Self::ScpQuorumset(_) => MessageType::ScpQuorumset,
            Self::ScpMessage(_) => MessageType::ScpMessage,
            Self::GetScpState(_) => MessageType::GetScpState,
            Self::SendMore(_) => MessageType::SendMore,
            Self::SendMoreExtended(_) => MessageType::SendMoreExtended,
            Self::FloodAdvert(_) => MessageType::FloodAdvert,
            Self::FloodDemand(_) => MessageType::FloodDemand,
        }
    }
}

impl StellarMessage {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_stellar_message(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_stellar_message(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`StellarMessage`], mirroring `<StellarMessage as WriteXdr>::write_xdr`.
    pub const fn write_type_stellar_message(&mut self, v: &StellarMessage) {
        let d = v.discriminant();
        self.write_type_message_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            StellarMessage::ErrorMsg(value) => {
                self.write_type_s_error(value);
            }
            StellarMessage::Hello(value) => {
                self.write_type_hello(value);
            }
            StellarMessage::Auth(value) => {
                self.write_type_auth(value);
            }
            StellarMessage::DontHave(value) => {
                self.write_type_dont_have(value);
            }
            StellarMessage::Peers(value) => {
                self.write_type_vec_peer_address(value);
            }
            StellarMessage::GetTxSet(value) => {
                self.write_type_uint256(value);
            }
            StellarMessage::TxSet(value) => {
                self.write_type_transaction_set(value);
            }
            StellarMessage::GeneralizedTxSet(value) => {
                self.write_type_generalized_transaction_set(value);
            }
            StellarMessage::Transaction(value) => {
                self.write_type_transaction_envelope(value);
            }
            StellarMessage::TimeSlicedSurveyRequest(value) => {
                self.write_type_signed_time_sliced_survey_request_message(value);
            }
            StellarMessage::TimeSlicedSurveyResponse(value) => {
                self.write_type_signed_time_sliced_survey_response_message(value);
            }
            StellarMessage::TimeSlicedSurveyStartCollecting(value) => {
                self.write_type_signed_time_sliced_survey_start_collecting_message(value);
            }
            StellarMessage::TimeSlicedSurveyStopCollecting(value) => {
                self.write_type_signed_time_sliced_survey_stop_collecting_message(value);
            }
            StellarMessage::GetScpQuorumset(value) => {
                self.write_type_uint256(value);
            }
            StellarMessage::ScpQuorumset(value) => {
                self.write_type_scp_quorum_set(value);
            }
            StellarMessage::ScpMessage(value) => {
                self.write_type_scp_envelope(value);
            }
            StellarMessage::GetScpState(value) => {
                self.write_u32(*value);
            }
            StellarMessage::SendMore(value) => {
                self.write_type_send_more(value);
            }
            StellarMessage::SendMoreExtended(value) => {
                self.write_type_send_more_extended(value);
            }
            StellarMessage::FloodAdvert(value) => {
                self.write_type_flood_advert(value);
            }
            StellarMessage::FloodDemand(value) => {
                self.write_type_flood_demand(value);
            }
        }
    }
}
