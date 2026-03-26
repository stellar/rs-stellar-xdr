#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCodeEntryV1 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///             {
///                 ExtensionPoint ext;
///                 ContractCodeCostInputs costInputs;
///             }
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
pub struct ContractCodeEntryV1 {
    pub ext: ExtensionPoint,
    pub cost_inputs: ContractCodeCostInputs,
}

impl ReadXdr for ContractCodeEntryV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                cost_inputs: ContractCodeCostInputs::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeEntryV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.cost_inputs.write_xdr(w)?;
            Ok(())
        })
    }
}
