#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryExtensionV1 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerEntryExtensionV1
/// {
///     SponsorshipDescriptor sponsoringID;
/// 
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
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
pub struct LedgerEntryExtensionV1 {
    pub sponsoring_id: SponsorshipDescriptor,
    pub ext: LedgerEntryExtensionV1Ext,
}

impl ReadXdr for LedgerEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                sponsoring_id: SponsorshipDescriptor::read_xdr(r)?,
                ext: LedgerEntryExtensionV1Ext::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.sponsoring_id.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
