#[allow(unused_imports)]
use super::*;
/// MessageType is an XDR Enum defined as:
///
/// ```text
/// enum MessageType
/// {
///     ERROR_MSG = 0,
///     AUTH = 2,
///     DONT_HAVE = 3,
///     // GET_PEERS (4) is deprecated
///
///     PEERS = 5,
///
///     GET_TX_SET = 6, // gets a particular txset by hash
///     TX_SET = 7,
///     GENERALIZED_TX_SET = 17,
///
///     TRANSACTION = 8, // pass on a tx you have heard about
///
///     // SCP
///     GET_SCP_QUORUMSET = 9,
///     SCP_QUORUMSET = 10,
///     SCP_MESSAGE = 11,
///     GET_SCP_STATE = 12,
///
///     // new messages
///     HELLO = 13,
///
///     // SURVEY_REQUEST (14) removed and replaced by TIME_SLICED_SURVEY_REQUEST
///     // SURVEY_RESPONSE (15) removed and replaced by TIME_SLICED_SURVEY_RESPONSE
///
///     SEND_MORE = 16,
///     SEND_MORE_EXTENDED = 20,
///
///     FLOOD_ADVERT = 18,
///     FLOOD_DEMAND = 19,
///
///     TIME_SLICED_SURVEY_REQUEST = 21,
///     TIME_SLICED_SURVEY_RESPONSE = 22,
///     TIME_SLICED_SURVEY_START_COLLECTING = 23,
///     TIME_SLICED_SURVEY_STOP_COLLECTING = 24
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum MessageType {
    #[cfg_attr(feature = "alloc", default)]
    ErrorMsg = 0,
    Auth = 2,
    DontHave = 3,
    Peers = 5,
    GetTxSet = 6,
    TxSet = 7,
    GeneralizedTxSet = 17,
    Transaction = 8,
    GetScpQuorumset = 9,
    ScpQuorumset = 10,
    ScpMessage = 11,
    GetScpState = 12,
    Hello = 13,
    SendMore = 16,
    SendMoreExtended = 20,
    FloodAdvert = 18,
    FloodDemand = 19,
    TimeSlicedSurveyRequest = 21,
    TimeSlicedSurveyResponse = 22,
    TimeSlicedSurveyStartCollecting = 23,
    TimeSlicedSurveyStopCollecting = 24,
}

impl MessageType {
    const _VARIANTS: &[MessageType] = &[
        MessageType::ErrorMsg,
        MessageType::Auth,
        MessageType::DontHave,
        MessageType::Peers,
        MessageType::GetTxSet,
        MessageType::TxSet,
        MessageType::GeneralizedTxSet,
        MessageType::Transaction,
        MessageType::GetScpQuorumset,
        MessageType::ScpQuorumset,
        MessageType::ScpMessage,
        MessageType::GetScpState,
        MessageType::Hello,
        MessageType::SendMore,
        MessageType::SendMoreExtended,
        MessageType::FloodAdvert,
        MessageType::FloodDemand,
        MessageType::TimeSlicedSurveyRequest,
        MessageType::TimeSlicedSurveyResponse,
        MessageType::TimeSlicedSurveyStartCollecting,
        MessageType::TimeSlicedSurveyStopCollecting,
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
        "Auth",
        "DontHave",
        "Peers",
        "GetTxSet",
        "TxSet",
        "GeneralizedTxSet",
        "Transaction",
        "GetScpQuorumset",
        "ScpQuorumset",
        "ScpMessage",
        "GetScpState",
        "Hello",
        "SendMore",
        "SendMoreExtended",
        "FloodAdvert",
        "FloodDemand",
        "TimeSlicedSurveyRequest",
        "TimeSlicedSurveyResponse",
        "TimeSlicedSurveyStartCollecting",
        "TimeSlicedSurveyStopCollecting",
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
            Self::ErrorMsg => "ErrorMsg",
            Self::Auth => "Auth",
            Self::DontHave => "DontHave",
            Self::Peers => "Peers",
            Self::GetTxSet => "GetTxSet",
            Self::TxSet => "TxSet",
            Self::GeneralizedTxSet => "GeneralizedTxSet",
            Self::Transaction => "Transaction",
            Self::GetScpQuorumset => "GetScpQuorumset",
            Self::ScpQuorumset => "ScpQuorumset",
            Self::ScpMessage => "ScpMessage",
            Self::GetScpState => "GetScpState",
            Self::Hello => "Hello",
            Self::SendMore => "SendMore",
            Self::SendMoreExtended => "SendMoreExtended",
            Self::FloodAdvert => "FloodAdvert",
            Self::FloodDemand => "FloodDemand",
            Self::TimeSlicedSurveyRequest => "TimeSlicedSurveyRequest",
            Self::TimeSlicedSurveyResponse => "TimeSlicedSurveyResponse",
            Self::TimeSlicedSurveyStartCollecting => "TimeSlicedSurveyStartCollecting",
            Self::TimeSlicedSurveyStopCollecting => "TimeSlicedSurveyStopCollecting",
        }
    }

    #[must_use]
    pub const fn variants() -> [MessageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for MessageType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<MessageType> for MessageType {
    fn variants() -> slice::Iter<'static, MessageType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for MessageType {}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for MessageType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => MessageType::ErrorMsg,
            2 => MessageType::Auth,
            3 => MessageType::DontHave,
            5 => MessageType::Peers,
            6 => MessageType::GetTxSet,
            7 => MessageType::TxSet,
            17 => MessageType::GeneralizedTxSet,
            8 => MessageType::Transaction,
            9 => MessageType::GetScpQuorumset,
            10 => MessageType::ScpQuorumset,
            11 => MessageType::ScpMessage,
            12 => MessageType::GetScpState,
            13 => MessageType::Hello,
            16 => MessageType::SendMore,
            20 => MessageType::SendMoreExtended,
            18 => MessageType::FloodAdvert,
            19 => MessageType::FloodDemand,
            21 => MessageType::TimeSlicedSurveyRequest,
            22 => MessageType::TimeSlicedSurveyResponse,
            23 => MessageType::TimeSlicedSurveyStartCollecting,
            24 => MessageType::TimeSlicedSurveyStopCollecting,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MessageType> for i32 {
    #[must_use]
    fn from(e: MessageType) -> Self {
        e as Self
    }
}

impl ReadXdr for MessageType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for MessageType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
