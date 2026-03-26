#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractComputeV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractComputeV0
/// {
///     // Maximum instructions per ledger
///     int64 ledgerMaxInstructions;
///     // Maximum instructions per transaction
///     int64 txMaxInstructions;
///     // Cost of 10000 instructions
///     int64 feeRatePerInstructionsIncrement;
///
///     // Memory limit per transaction. Unlike instructions, there is no fee
///     // for memory, just the limit.
///     uint32 txMemoryLimit;
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
pub struct ConfigSettingContractComputeV0 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub ledger_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub tx_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_rate_per_instructions_increment: i64,
    pub tx_memory_limit: u32,
}

impl ReadXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_instructions: i64::read_xdr(r)?,
                tx_max_instructions: i64::read_xdr(r)?,
                fee_rate_per_instructions_increment: i64::read_xdr(r)?,
                tx_memory_limit: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_instructions.write_xdr(w)?;
            self.tx_max_instructions.write_xdr(w)?;
            self.fee_rate_per_instructions_increment.write_xdr(w)?;
            self.tx_memory_limit.write_xdr(w)?;
            Ok(())
        })
    }
}
