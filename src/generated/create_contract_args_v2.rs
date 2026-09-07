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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

/// CreateContractArgsV2View is a borrowing equivalent of [`CreateContractArgsV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateContractArgsV2View<'a> {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutableView<'a>,
    pub constructor_args: VecMView<'a, ScValView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for CreateContractArgsV2View<'_> {
    type Owned = CreateContractArgsV2;
    fn into_owned(self) -> CreateContractArgsV2 {
        CreateContractArgsV2 {
            contract_id_preimage: self.contract_id_preimage.into_owned(),
            executable: self.executable.into_owned(),
            constructor_args: self.constructor_args.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&CreateContractArgsV2View<'_>> for CreateContractArgsV2 {
    #[must_use]
    fn from(v: &CreateContractArgsV2View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<CreateContractArgsV2View<'_>> for CreateContractArgsV2 {
    #[must_use]
    fn from(v: CreateContractArgsV2View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for CreateContractArgsV2View<'_> {
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
