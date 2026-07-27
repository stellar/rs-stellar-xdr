#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DiagnosticEvent is an XDR Struct defined as:
///
/// ```text
/// struct DiagnosticEvent
/// {
///     bool inSuccessfulContractCall;
///     ContractEvent event;
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
pub struct DiagnosticEvent {
    pub in_successful_contract_call: bool,
    pub event: ContractEvent,
}

impl ReadXdr for DiagnosticEvent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                in_successful_contract_call: bool::read_xdr(r)?,
                event: ContractEvent::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for DiagnosticEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.in_successful_contract_call.write_xdr(w)?;
            self.event.write_xdr(w)?;
            Ok(())
        })
    }
}

/// DiagnosticEventRef is a borrowing equivalent of [`DiagnosticEvent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagnosticEventRef<'a> {
    pub in_successful_contract_call: bool,
    pub event: ContractEventRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&DiagnosticEventRef<'_>> for DiagnosticEvent {
    #[must_use]
    fn from(v: &DiagnosticEventRef<'_>) -> Self {
        Self {
            in_successful_contract_call: v.in_successful_contract_call,
            event: (&v.event).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<DiagnosticEventRef<'_>> for DiagnosticEvent {
    #[must_use]
    fn from(v: DiagnosticEventRef<'_>) -> Self {
        Self::from(&v)
    }
}
