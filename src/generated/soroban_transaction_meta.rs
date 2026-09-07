#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMeta is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMeta
/// {
///     SorobanTransactionMetaExt ext;
///
///     ContractEvent events<>;             // custom events populated by the
///                                         // contracts themselves.
///     SCVal returnValue;                  // return value of the host fn invocation
///
///     // Diagnostics events that are not hashed.
///     // This will contain all contract and diagnostic events. Even ones
///     // that were emitted in a failed contract call.
///     DiagnosticEvent diagnosticEvents<>;
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
pub struct SorobanTransactionMeta {
    pub ext: SorobanTransactionMetaExt,
    pub events: VecM<ContractEvent>,
    pub return_value: ScVal,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl ReadXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionMetaExt::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
                return_value: ScVal::read_xdr(r)?,
                diagnostic_events: VecM::<DiagnosticEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanTransactionMetaRef is a borrowing equivalent of [`SorobanTransactionMeta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionMetaRef<'a> {
    pub ext: SorobanTransactionMetaExt,
    pub events: VecMRef<'a, ContractEventRef<'a>>,
    pub return_value: ScValRef<'a>,
    pub diagnostic_events: VecMRef<'a, DiagnosticEventRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanTransactionMetaRef<'_> {
    type Owned = SorobanTransactionMeta;
    fn into_owned(self) -> SorobanTransactionMeta {
        SorobanTransactionMeta {
            ext: self.ext.into_owned(),
            events: self.events.into_owned(),
            return_value: self.return_value.into_owned(),
            diagnostic_events: self.diagnostic_events.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanTransactionMetaRef<'_>> for SorobanTransactionMeta {
    #[must_use]
    fn from(v: &SorobanTransactionMetaRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanTransactionMetaRef<'_>> for SorobanTransactionMeta {
    #[must_use]
    fn from(v: SorobanTransactionMetaRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanTransactionMetaRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}
