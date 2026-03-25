#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// LedgerKeyContractData is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         SCAddress contract;
///         SCVal key;
///         ContractDataDurability durability;
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
pub struct LedgerKeyContractData {
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
}

impl ReadXdr for LedgerKeyContractData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract: ScAddress::read_xdr(r)?,
                key: ScVal::read_xdr(r)?,
                durability: ContractDataDurability::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyContractData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract.write_xdr(w)?;
            self.key.write_xdr(w)?;
            self.durability.write_xdr(w)?;
            Ok(())
        })
    }
}
