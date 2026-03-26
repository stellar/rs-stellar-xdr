#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageContractId is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash networkID;
///         ContractIDPreimage contractIDPreimage;
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
pub struct HashIdPreimageContractId {
    pub network_id: Hash,
    pub contract_id_preimage: ContractIdPreimage,
}

impl ReadXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                contract_id_preimage: ContractIdPreimage::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.contract_id_preimage.write_xdr(w)?;
            Ok(())
        })
    }
}
