#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationMeta is an XDR Struct defined as:
///
/// ```text
/// struct OperationMeta
/// {
///     LedgerEntryChanges changes;
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
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

impl ReadXdr for OperationMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                changes: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for OperationMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.changes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// OperationMetaRef is a borrowing equivalent of [`OperationMeta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationMetaRef<'a> {
    pub changes: LedgerEntryChangesRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for OperationMetaRef<'_> {
    type Owned = OperationMeta;
    fn into_owned(self) -> OperationMeta {
        OperationMeta {
            changes: self.changes.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&OperationMetaRef<'_>> for OperationMeta {
    #[must_use]
    fn from(v: &OperationMetaRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<OperationMetaRef<'_>> for OperationMeta {
    #[must_use]
    fn from(v: OperationMetaRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for OperationMetaRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.changes.write_xdr(w)?;
            Ok(())
        })
    }
}
