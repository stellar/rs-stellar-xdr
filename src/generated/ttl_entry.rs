#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// TtlEntry is an XDR Struct defined as:
///
/// ```text
/// struct TTLEntry {
///     // Hash of the LedgerKey that is associated with this TTLEntry
///     Hash keyHash;
///     uint32 liveUntilLedgerSeq;
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
pub struct TtlEntry {
    pub key_hash: Hash,
    pub live_until_ledger_seq: u32,
}

impl ReadXdr for TtlEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key_hash: Hash::read_xdr(r)?,
                live_until_ledger_seq: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TtlEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key_hash.write_xdr(w)?;
            self.live_until_ledger_seq.write_xdr(w)?;
            Ok(())
        })
    }
}
