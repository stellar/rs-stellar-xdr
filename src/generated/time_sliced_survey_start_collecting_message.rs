#[allow(unused_imports)]
use super::*;
/// TimeSlicedSurveyStartCollectingMessage is an XDR Struct defined as:
///
/// ```text
/// struct TimeSlicedSurveyStartCollectingMessage
/// {
///     NodeID surveyorID;
///     uint32 nonce;
///     uint32 ledgerNum;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TimeSlicedSurveyStartCollectingMessage {
    pub surveyor_id: NodeId,
    pub nonce: u32,
    pub ledger_num: u32,
}

impl ReadXdr for TimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                surveyor_id: NodeId::read_xdr(r)?,
                nonce: u32::read_xdr(r)?,
                ledger_num: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TimeSlicedSurveyStartCollectingMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.surveyor_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.ledger_num.write_xdr(w)?;
            Ok(())
        })
    }
}
