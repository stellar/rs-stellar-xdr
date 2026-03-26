#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StoredDebugTransactionSet is an XDR Struct defined as:
///
/// ```text
/// struct StoredDebugTransactionSet
/// {
/// 	StoredTransactionSet txSet;
/// 	uint32 ledgerSeq;
/// 	StellarValue scpValue;
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
pub struct StoredDebugTransactionSet {
    pub tx_set: StoredTransactionSet,
    pub ledger_seq: u32,
    pub scp_value: StellarValue,
}

impl ReadXdr for StoredDebugTransactionSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set: StoredTransactionSet::read_xdr(r)?,
                ledger_seq: u32::read_xdr(r)?,
                scp_value: StellarValue::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for StoredDebugTransactionSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set.write_xdr(w)?;
            self.ledger_seq.write_xdr(w)?;
            self.scp_value.write_xdr(w)?;
            Ok(())
        })
    }
}
