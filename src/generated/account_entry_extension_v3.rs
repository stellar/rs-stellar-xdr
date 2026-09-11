#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountEntryExtensionV3 is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntryExtensionV3
/// {
///     // We can use this to add more fields, or because it is first, to
///     // change AccountEntryExtensionV3 into a union.
///     ExtensionPoint ext;
///
///     // Ledger number at which `seqNum` took on its present value.
///     uint32 seqLedger;
///
///     // Time at which `seqNum` took on its present value.
///     TimePoint seqTime;
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
pub struct AccountEntryExtensionV3 {
    pub ext: ExtensionPoint,
    pub seq_ledger: u32,
    pub seq_time: TimePoint,
}

impl ReadXdr for AccountEntryExtensionV3 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                seq_ledger: u32::read_xdr(r)?,
                seq_time: TimePoint::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AccountEntryExtensionV3 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.seq_ledger.write_xdr(w)?;
            self.seq_time.write_xdr(w)?;
            Ok(())
        })
    }
}
