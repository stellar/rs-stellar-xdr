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

impl WriteXdr for TransactionEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.stage.write_xdr(w)?;
            self.event.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionEventView is a borrowing equivalent of [`TransactionEvent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionEventView<'a> {
    pub stage: TransactionEventStage,
    pub event: ContractEventView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionEventView<'_> {
    type Owned = TransactionEvent;
    fn into_owned(self) -> TransactionEvent {
        TransactionEvent {
            stage: self.stage.into_owned(),
            event: self.event.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionEventView<'_>> for TransactionEvent {
    #[must_use]
    fn from(v: &TransactionEventView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionEventView<'_>> for TransactionEvent {
    #[must_use]
    fn from(v: TransactionEventView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionEventView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.stage.write_xdr(w)?;
            self.event.write_xdr(w)?;
            Ok(())
        })
    }
}
