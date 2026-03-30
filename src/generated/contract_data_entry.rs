#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractDataEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractDataEntry {
///     ExtensionPoint ext;
///
///     SCAddress contract;
///     SCVal key;
///     ContractDataDurability durability;
///     SCVal val;
/// };
/// ```
///
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
pub struct ContractDataEntry {
    pub ext: ExtensionPoint,
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
    pub val: ScVal,
}

impl ReadXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                contract: ScAddress::read_xdr(r)?,
                key: ScVal::read_xdr(r)?,
                durability: ContractDataDurability::read_xdr(r)?,
                val: ScVal::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract.write_xdr(w)?;
            self.key.write_xdr(w)?;
            self.durability.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}
