#[allow(unused_imports)]
use super::*;
/// ContractIdPreimageFromAddress is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         SCAddress address;
///         uint256 salt;
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
pub struct ContractIdPreimageFromAddress {
    pub address: ScAddress,
    pub salt: Uint256,
}

impl ReadXdr for ContractIdPreimageFromAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                salt: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractIdPreimageFromAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.salt.write_xdr(w)?;
            Ok(())
        })
    }
}
