#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanResourcesExtV0 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanResourcesExtV0
/// {
///     // Vector of indices representing what Soroban
///     // entries in the footprint are archived, based on the
///     // order of keys provided in the readWrite footprint.
///     uint32 archivedSorobanEntries<>;
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
pub struct SorobanResourcesExtV0 {
    pub archived_soroban_entries: VecM<u32>,
}

impl ReadXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                archived_soroban_entries: VecM::<u32>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.archived_soroban_entries.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanResourcesExtV0Ref is a borrowing equivalent of [`SorobanResourcesExtV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanResourcesExtV0Ref<'a> {
    pub archived_soroban_entries: VecMRef<'a, u32>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanResourcesExtV0Ref<'_>> for SorobanResourcesExtV0 {
    #[must_use]
    fn from(v: &SorobanResourcesExtV0Ref<'_>) -> Self {
        Self {
            archived_soroban_entries: v.archived_soroban_entries.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanResourcesExtV0Ref<'_>> for SorobanResourcesExtV0 {
    #[must_use]
    fn from(v: SorobanResourcesExtV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanResourcesExtV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.archived_soroban_entries.write_xdr(w)?;
            Ok(())
        })
    }
}
