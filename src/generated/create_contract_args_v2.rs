#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreateContractArgsV2 is an XDR Struct defined as:
///
/// ```text
/// struct CreateContractArgsV2
/// {
///     ContractIDPreimage contractIDPreimage;
///     ContractExecutable executable;
///     // Arguments of the contract's constructor.
///     SCVal constructorArgs<>;
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
pub struct CreateContractArgsV2 {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutable,
    pub constructor_args: VecM<ScVal>,
}

impl ReadXdr for CreateContractArgsV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_id_preimage: ContractIdPreimage::read_xdr(r)?,
                executable: ContractExecutable::read_xdr(r)?,
                constructor_args: VecM::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreateContractArgsV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id_preimage.write_xdr(w)?;
            self.executable.write_xdr(w)?;
            self.constructor_args.write_xdr(w)?;
            Ok(())
        })
    }
}
