#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StellarValueProposedMsValue is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             TimePointMilliseconds closeTimeMs; // closeTime == closeTimeMs / 1000
///             Hash txSetHash;
///             Hash previousLedgerHash;
///             uint32 previousLedgerVersion;
///             LedgerCloseValueSignature lcValueSignature;
///         }
/// ```
///
#[cfg(feature = "ms_close_time")]
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
pub struct StellarValueProposedMsValue {
    pub close_time_ms: TimePointMilliseconds,
    pub tx_set_hash: Hash,
    pub previous_ledger_hash: Hash,
    pub previous_ledger_version: u32,
    pub lc_value_signature: LedgerCloseValueSignature,
}

#[cfg(feature = "ms_close_time")]
impl ReadXdr for StellarValueProposedMsValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                close_time_ms: TimePointMilliseconds::read_xdr(r)?,
                tx_set_hash: Hash::read_xdr(r)?,
                previous_ledger_hash: Hash::read_xdr(r)?,
                previous_ledger_version: u32::read_xdr(r)?,
                lc_value_signature: LedgerCloseValueSignature::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "ms_close_time")]
impl WriteXdr for StellarValueProposedMsValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.close_time_ms.write_xdr(w)?;
            self.tx_set_hash.write_xdr(w)?;
            self.previous_ledger_hash.write_xdr(w)?;
            self.previous_ledger_version.write_xdr(w)?;
            self.lc_value_signature.write_xdr(w)?;
            Ok(())
        })
    }
}
