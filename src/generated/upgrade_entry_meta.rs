#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// UpgradeEntryMeta is an XDR Struct defined as:
///
/// ```text
/// struct UpgradeEntryMeta
/// {
///     LedgerUpgrade upgrade;
///     LedgerEntryChanges changes;
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
pub struct UpgradeEntryMeta {
    pub upgrade: LedgerUpgrade,
    pub changes: LedgerEntryChanges,
}

impl ReadXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                upgrade: LedgerUpgrade::read_xdr(r)?,
                changes: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.upgrade.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            Ok(())
        })
    }
}
