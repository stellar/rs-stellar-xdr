#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerKeyConfigSetting is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         ConfigSettingID configSettingID;
///     }
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
pub struct LedgerKeyConfigSetting {
    pub config_setting_id: ConfigSettingId,
}

impl ReadXdr for LedgerKeyConfigSetting {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                config_setting_id: ConfigSettingId::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyConfigSetting {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.config_setting_id.write_xdr(w)?;
            Ok(())
        })
    }
}
