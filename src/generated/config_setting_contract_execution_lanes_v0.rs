#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractExecutionLanesV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractExecutionLanesV0
/// {
///     // maximum number of Soroban transactions per ledger
///     uint32 ledgerMaxTxCount;
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
pub struct ConfigSettingContractExecutionLanesV0 {
    pub ledger_max_tx_count: u32,
}

impl ReadXdr for ConfigSettingContractExecutionLanesV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_tx_count: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractExecutionLanesV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_tx_count.write_xdr(w)?;
            Ok(())
        })
    }
}
