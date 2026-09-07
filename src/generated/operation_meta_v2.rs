#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct OperationMetaV2
/// {
///     ExtensionPoint ext;
///
///     LedgerEntryChanges changes;
///
///     ContractEvent events<>;
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
pub struct OperationMetaV2 {
    pub ext: ExtensionPoint,
    pub changes: LedgerEntryChanges,
    pub events: VecM<ContractEvent>,
}

impl ReadXdr for OperationMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                changes: LedgerEntryChanges::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for OperationMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}

/// OperationMetaV2View is a borrowing equivalent of [`OperationMetaV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationMetaV2View<'a> {
    pub ext: ExtensionPoint,
    pub changes: LedgerEntryChangesView<'a>,
    pub events: VecMView<'a, ContractEventView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for OperationMetaV2View<'_> {
    type Owned = OperationMetaV2;
    fn into_owned(self) -> OperationMetaV2 {
        OperationMetaV2 {
            ext: self.ext.into_owned(),
            changes: self.changes.into_owned(),
            events: self.events.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&OperationMetaV2View<'_>> for OperationMetaV2 {
    #[must_use]
    fn from(v: &OperationMetaV2View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<OperationMetaV2View<'_>> for OperationMetaV2 {
    #[must_use]
    fn from(v: OperationMetaV2View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for OperationMetaV2View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}
