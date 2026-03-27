#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpHistoryEntryV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCPHistoryEntryV0
/// {
///     SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
///     LedgerSCPMessages ledgerMessages;
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
pub struct ScpHistoryEntryV0 {
    pub quorum_sets: VecM<ScpQuorumSet>,
    pub ledger_messages: LedgerScpMessages,
}

impl ReadXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
                ledger_messages: LedgerScpMessages::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_sets.write_xdr(w)?;
            self.ledger_messages.write_xdr(w)?;
            Ok(())
        })
    }
}
