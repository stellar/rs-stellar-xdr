#[allow(unused_imports, clippy::wildcard_imports)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl TimeSlicedSurveyStartCollectingMessage {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.surveyor_id.const_write_xdr(w);
        w.write_u32(self.nonce);
        w.write_u32(self.ledger_num);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
