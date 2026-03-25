#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// StellarMessage is an XDR Union defined as:
///
/// ```text
/// union StellarMessage switch (MessageType type)
/// {
/// case ERROR_MSG:
///     Error error;
/// case HELLO:
///     Hello hello;
/// case AUTH:
///     Auth auth;
/// case DONT_HAVE:
///     DontHave dontHave;
/// case PEERS:
///     PeerAddress peers<100>;
///
/// case GET_TX_SET:
///     uint256 txSetHash;
/// case TX_SET:
///     TransactionSet txSet;
/// case GENERALIZED_TX_SET:
///     GeneralizedTransactionSet generalizedTxSet;
///
/// case TRANSACTION:
///     TransactionEnvelope transaction;
///
/// case TIME_SLICED_SURVEY_REQUEST:
///     SignedTimeSlicedSurveyRequestMessage signedTimeSlicedSurveyRequestMessage;
///
/// case TIME_SLICED_SURVEY_RESPONSE:
///     SignedTimeSlicedSurveyResponseMessage signedTimeSlicedSurveyResponseMessage;
///
/// case TIME_SLICED_SURVEY_START_COLLECTING:
///     SignedTimeSlicedSurveyStartCollectingMessage
///         signedTimeSlicedSurveyStartCollectingMessage;
///
/// case TIME_SLICED_SURVEY_STOP_COLLECTING:
///     SignedTimeSlicedSurveyStopCollectingMessage
///         signedTimeSlicedSurveyStopCollectingMessage;
///
/// // SCP
/// case GET_SCP_QUORUMSET:
///     uint256 qSetHash;
/// case SCP_QUORUMSET:
///     SCPQuorumSet qSet;
/// case SCP_MESSAGE:
///     SCPEnvelope envelope;
/// case GET_SCP_STATE:
///     uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
/// case SEND_MORE:
///     SendMore sendMoreMessage;
/// case SEND_MORE_EXTENDED:
///     SendMoreExtended sendMoreExtendedMessage;
/// // Pull mode
/// case FLOOD_ADVERT:
///      FloodAdvert floodAdvert;
/// case FLOOD_DEMAND:
///      FloodDemand floodDemand;
/// };
/// ```
///
// union with discriminant MessageType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

#[cfg(feature = "alloc")]
impl Default for StellarMessage {
    fn default() -> Self {
        Self::ErrorMsg(SError::default())
    }
}

impl StellarMessage {
    const _VARIANTS: &[MessageType] = &[
        MessageType::ErrorMsg,
        MessageType::Hello,
        MessageType::Auth,
        MessageType::DontHave,
        MessageType::Peers,
        MessageType::GetTxSet,
        MessageType::TxSet,
        MessageType::GeneralizedTxSet,
        MessageType::Transaction,
        MessageType::TimeSlicedSurveyRequest,
        MessageType::TimeSlicedSurveyResponse,
        MessageType::TimeSlicedSurveyStartCollecting,
        MessageType::TimeSlicedSurveyStopCollecting,
        MessageType::GetScpQuorumset,
        MessageType::ScpQuorumset,
        MessageType::ScpMessage,
        MessageType::GetScpState,
        MessageType::SendMore,
        MessageType::SendMoreExtended,
        MessageType::FloodAdvert,
        MessageType::FloodDemand,
    ];
    pub const VARIANTS: [MessageType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ErrorMsg",
        "Hello",
        "Auth",
        "DontHave",
        "Peers",
        "GetTxSet",
        "TxSet",
        "GeneralizedTxSet",
        "Transaction",
        "TimeSlicedSurveyRequest",
        "TimeSlicedSurveyResponse",
        "TimeSlicedSurveyStartCollecting",
        "TimeSlicedSurveyStopCollecting",
        "GetScpQuorumset",
        "ScpQuorumset",
        "ScpMessage",
        "GetScpState",
        "SendMore",
        "SendMoreExtended",
        "FloodAdvert",
        "FloodDemand",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ErrorMsg(_) => "ErrorMsg",
            Self::Hello(_) => "Hello",
            Self::Auth(_) => "Auth",
            Self::DontHave(_) => "DontHave",
            Self::Peers(_) => "Peers",
            Self::GetTxSet(_) => "GetTxSet",
            Self::TxSet(_) => "TxSet",
            Self::GeneralizedTxSet(_) => "GeneralizedTxSet",
            Self::Transaction(_) => "Transaction",
            Self::TimeSlicedSurveyRequest(_) => "TimeSlicedSurveyRequest",
            Self::TimeSlicedSurveyResponse(_) => "TimeSlicedSurveyResponse",
            Self::TimeSlicedSurveyStartCollecting(_) => "TimeSlicedSurveyStartCollecting",
            Self::TimeSlicedSurveyStopCollecting(_) => "TimeSlicedSurveyStopCollecting",
            Self::GetScpQuorumset(_) => "GetScpQuorumset",
            Self::ScpQuorumset(_) => "ScpQuorumset",
            Self::ScpMessage(_) => "ScpMessage",
            Self::GetScpState(_) => "GetScpState",
            Self::SendMore(_) => "SendMore",
            Self::SendMoreExtended(_) => "SendMoreExtended",
            Self::FloodAdvert(_) => "FloodAdvert",
            Self::FloodDemand(_) => "FloodDemand",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [MessageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for StellarMessage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<MessageType> for StellarMessage {
    #[must_use]
    fn discriminant(&self) -> MessageType {
        Self::discriminant(self)
    }
}

impl Variants<MessageType> for StellarMessage {
    fn variants() -> slice::Iter<'static, MessageType> {
        Self::VARIANTS.iter()
    }
}

impl Union<MessageType> for StellarMessage {}

impl ReadXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: MessageType = <MessageType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                MessageType::ErrorMsg => Self::ErrorMsg(SError::read_xdr(r)?),
                MessageType::Hello => Self::Hello(Hello::read_xdr(r)?),
                MessageType::Auth => Self::Auth(Auth::read_xdr(r)?),
                MessageType::DontHave => Self::DontHave(DontHave::read_xdr(r)?),
                MessageType::Peers => Self::Peers(VecM::<PeerAddress, 100>::read_xdr(r)?),
                MessageType::GetTxSet => Self::GetTxSet(Uint256::read_xdr(r)?),
                MessageType::TxSet => Self::TxSet(TransactionSet::read_xdr(r)?),
                MessageType::GeneralizedTxSet => {
                    Self::GeneralizedTxSet(GeneralizedTransactionSet::read_xdr(r)?)
                }
                MessageType::Transaction => Self::Transaction(TransactionEnvelope::read_xdr(r)?),
                MessageType::TimeSlicedSurveyRequest => Self::TimeSlicedSurveyRequest(
                    SignedTimeSlicedSurveyRequestMessage::read_xdr(r)?,
                ),
                MessageType::TimeSlicedSurveyResponse => Self::TimeSlicedSurveyResponse(
                    SignedTimeSlicedSurveyResponseMessage::read_xdr(r)?,
                ),
                MessageType::TimeSlicedSurveyStartCollecting => {
                    Self::TimeSlicedSurveyStartCollecting(
                        SignedTimeSlicedSurveyStartCollectingMessage::read_xdr(r)?,
                    )
                }
                MessageType::TimeSlicedSurveyStopCollecting => {
                    Self::TimeSlicedSurveyStopCollecting(
                        SignedTimeSlicedSurveyStopCollectingMessage::read_xdr(r)?,
                    )
                }
                MessageType::GetScpQuorumset => Self::GetScpQuorumset(Uint256::read_xdr(r)?),
                MessageType::ScpQuorumset => Self::ScpQuorumset(ScpQuorumSet::read_xdr(r)?),
                MessageType::ScpMessage => Self::ScpMessage(ScpEnvelope::read_xdr(r)?),
                MessageType::GetScpState => Self::GetScpState(u32::read_xdr(r)?),
                MessageType::SendMore => Self::SendMore(SendMore::read_xdr(r)?),
                MessageType::SendMoreExtended => {
                    Self::SendMoreExtended(SendMoreExtended::read_xdr(r)?)
                }
                MessageType::FloodAdvert => Self::FloodAdvert(FloodAdvert::read_xdr(r)?),
                MessageType::FloodDemand => Self::FloodDemand(FloodDemand::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ErrorMsg(v) => v.write_xdr(w)?,
                Self::Hello(v) => v.write_xdr(w)?,
                Self::Auth(v) => v.write_xdr(w)?,
                Self::DontHave(v) => v.write_xdr(w)?,
                Self::Peers(v) => v.write_xdr(w)?,
                Self::GetTxSet(v) => v.write_xdr(w)?,
                Self::TxSet(v) => v.write_xdr(w)?,
                Self::GeneralizedTxSet(v) => v.write_xdr(w)?,
                Self::Transaction(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyRequest(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyResponse(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyStartCollecting(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyStopCollecting(v) => v.write_xdr(w)?,
                Self::GetScpQuorumset(v) => v.write_xdr(w)?,
                Self::ScpQuorumset(v) => v.write_xdr(w)?,
                Self::ScpMessage(v) => v.write_xdr(w)?,
                Self::GetScpState(v) => v.write_xdr(w)?,
                Self::SendMore(v) => v.write_xdr(w)?,
                Self::SendMoreExtended(v) => v.write_xdr(w)?,
                Self::FloodAdvert(v) => v.write_xdr(w)?,
                Self::FloodDemand(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
