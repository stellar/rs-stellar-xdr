#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// LedgerKeyTtl is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         // Hash of the LedgerKey that is associated with this TTLEntry
///         Hash keyHash;
///     }
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
pub struct LedgerKeyTtl {
    pub key_hash: Hash,
}

impl ReadXdr for LedgerKeyTtl {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyTtl {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key_hash.write_xdr(w)?;
            Ok(())
        })
    }
}
