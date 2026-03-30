#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractParallelComputeV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractParallelComputeV0
/// {
///     // Maximum number of clusters with dependent transactions allowed in a
///     // stage of parallel tx set component.
///     // This effectively sets the lower bound on the number of physical threads
///     // necessary to effectively apply transaction sets in parallel.
///     uint32 ledgerMaxDependentTxClusters;
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
pub struct ConfigSettingContractParallelComputeV0 {
    pub ledger_max_dependent_tx_clusters: u32,
}

impl ReadXdr for ConfigSettingContractParallelComputeV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_dependent_tx_clusters: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractParallelComputeV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_dependent_tx_clusters.write_xdr(w)?;
            Ok(())
        })
    }
}
