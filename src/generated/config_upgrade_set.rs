#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigUpgradeSet is an XDR Struct defined as:
///
/// ```text
/// struct ConfigUpgradeSet {
///     ConfigSettingEntry updatedEntry<>;
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
pub struct ConfigUpgradeSet {
    pub updated_entry: VecM<ConfigSettingEntry>,
}

impl ReadXdr for ConfigUpgradeSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                updated_entry: VecM::<ConfigSettingEntry>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigUpgradeSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.updated_entry.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ConfigUpgradeSetView is a borrowing equivalent of [`ConfigUpgradeSet`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConfigUpgradeSetView<'a> {
    pub updated_entry: VecMView<'a, ConfigSettingEntryView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ConfigUpgradeSetView<'_>> for ConfigUpgradeSet {
    #[must_use]
    fn from(v: &ConfigUpgradeSetView<'_>) -> Self {
        Self {
            updated_entry: v.updated_entry.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ConfigUpgradeSetView<'_>> for ConfigUpgradeSet {
    #[must_use]
    fn from(v: ConfigUpgradeSetView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ConfigUpgradeSetView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.updated_entry.write_xdr(w)?;
            Ok(())
        })
    }
}
