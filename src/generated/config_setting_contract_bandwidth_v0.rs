#[allow(unused_imports)]
use super::*;
/// ConfigSettingContractBandwidthV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractBandwidthV0
/// {
///     // Maximum sum of all transaction sizes in the ledger in bytes
///     uint32 ledgerMaxTxsSizeBytes;
///     // Maximum size in bytes for a transaction
///     uint32 txMaxSizeBytes;
///
///     // Fee for 1 KB of transaction size
///     int64 feeTxSize1KB;
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
pub struct ConfigSettingContractBandwidthV0 {
    pub ledger_max_txs_size_bytes: u32,
    pub tx_max_size_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_tx_size1_kb: i64,
}

impl ReadXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_txs_size_bytes: u32::read_xdr(r)?,
                tx_max_size_bytes: u32::read_xdr(r)?,
                fee_tx_size1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_txs_size_bytes.write_xdr(w)?;
            self.tx_max_size_bytes.write_xdr(w)?;
            self.fee_tx_size1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}
