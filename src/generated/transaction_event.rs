#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionEvent is an XDR Struct defined as:
///
/// ```text
/// struct TransactionEvent {    
///     TransactionEventStage stage;  // Stage at which an event has occurred.
///     ContractEvent event;  // The contract event that has occurred.
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
pub struct TransactionEvent {
    pub stage: TransactionEventStage,
    pub event: ContractEvent,
}

impl ReadXdr for TransactionEvent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                stage: TransactionEventStage::read_xdr(r)?,
                event: ContractEvent::read_xdr(r)?,
            })
        })
    }
}

impl TransactionEvent {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.stage.const_to_xdr(w);
        self.event.const_to_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for TransactionEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.stage.write_xdr(w)?;
            self.event.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
