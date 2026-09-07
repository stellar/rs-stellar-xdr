#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanResources is an XDR Struct defined as:
///
/// ```text
/// struct SorobanResources
/// {   
///     // The ledger footprint of the transaction.
///     LedgerFootprint footprint;
///     // The maximum number of instructions this transaction can use
///     uint32 instructions;
///
///     // The maximum number of bytes this transaction can read from disk backed entries
///     uint32 diskReadBytes;
///     // The maximum number of bytes this transaction can write to ledger
///     uint32 writeBytes;
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
pub struct SorobanResources {
    pub footprint: LedgerFootprint,
    pub instructions: u32,
    pub disk_read_bytes: u32,
    pub write_bytes: u32,
}

impl ReadXdr for SorobanResources {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                footprint: LedgerFootprint::read_xdr(r)?,
                instructions: u32::read_xdr(r)?,
                disk_read_bytes: u32::read_xdr(r)?,
                write_bytes: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanResources {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.footprint.write_xdr(w)?;
            self.instructions.write_xdr(w)?;
            self.disk_read_bytes.write_xdr(w)?;
            self.write_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanResourcesRef is a borrowing equivalent of [`SorobanResources`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanResourcesRef<'a> {
    pub footprint: LedgerFootprintRef<'a>,
    pub instructions: u32,
    pub disk_read_bytes: u32,
    pub write_bytes: u32,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanResourcesRef<'_> {
    type Owned = SorobanResources;
    fn into_owned(self) -> SorobanResources {
        SorobanResources {
            footprint: self.footprint.into_owned(),
            instructions: self.instructions.into_owned(),
            disk_read_bytes: self.disk_read_bytes.into_owned(),
            write_bytes: self.write_bytes.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanResourcesRef<'_>> for SorobanResources {
    #[must_use]
    fn from(v: &SorobanResourcesRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanResourcesRef<'_>> for SorobanResources {
    #[must_use]
    fn from(v: SorobanResourcesRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanResourcesRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.footprint.write_xdr(w)?;
            self.instructions.write_xdr(w)?;
            self.disk_read_bytes.write_xdr(w)?;
            self.write_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}
