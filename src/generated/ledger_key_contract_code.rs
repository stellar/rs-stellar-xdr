#[allow(unused_imports)]
use super::*;
/// LedgerKeyContractCode is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash hash;
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
pub struct LedgerKeyContractCode {
    pub hash: Hash,
}

impl ReadXdr for LedgerKeyContractCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyContractCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hash.write_xdr(w)?;
            Ok(())
        })
    }
}
