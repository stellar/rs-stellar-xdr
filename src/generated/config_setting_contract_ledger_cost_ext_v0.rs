#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ConfigSettingContractLedgerCostExtV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractLedgerCostExtV0
/// {
///     // Maximum number of RO+RW entries in the transaction footprint.
///     uint32 txMaxFootprintEntries;
///     // Fee per 1 KB of data written to the ledger.
///     // Unlike the rent fee, this is a flat fee that is charged for any ledger
///     // write, independent of the type of the entry being written.
///     int64 feeWrite1KB;
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
pub struct ConfigSettingContractLedgerCostExtV0 {
    pub tx_max_footprint_entries: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_write1_kb: i64,
}

impl ReadXdr for ConfigSettingContractLedgerCostExtV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_max_footprint_entries: u32::read_xdr(r)?,
                fee_write1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractLedgerCostExtV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_max_footprint_entries.write_xdr(w)?;
            self.fee_write1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}
