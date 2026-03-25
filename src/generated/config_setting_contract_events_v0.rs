#[allow(unused_imports)]
use super::*;
/// ConfigSettingContractEventsV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractEventsV0
/// {
///     // Maximum size of events that a contract call can emit.
///     uint32 txMaxContractEventsSizeBytes;
///     // Fee for generating 1KB of contract events.
///     int64 feeContractEvents1KB;
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
pub struct ConfigSettingContractEventsV0 {
    pub tx_max_contract_events_size_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_contract_events1_kb: i64,
}

impl ReadXdr for ConfigSettingContractEventsV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_max_contract_events_size_bytes: u32::read_xdr(r)?,
                fee_contract_events1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractEventsV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_max_contract_events_size_bytes.write_xdr(w)?;
            self.fee_contract_events1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}
