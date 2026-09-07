#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreateContractArgs is an XDR Struct defined as:
///
/// ```text
/// struct CreateContractArgs
/// {
///     ContractIDPreimage contractIDPreimage;
///     ContractExecutable executable;
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
pub struct CreateContractArgs {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutable,
}

impl ReadXdr for CreateContractArgs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_id_preimage: ContractIdPreimage::read_xdr(r)?,
                executable: ContractExecutable::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreateContractArgs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id_preimage.write_xdr(w)?;
            self.executable.write_xdr(w)?;
            Ok(())
        })
    }
}

/// CreateContractArgsView is a borrowing equivalent of [`CreateContractArgs`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateContractArgsView<'a> {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutableView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for CreateContractArgsView<'_> {
    type Owned = CreateContractArgs;
    fn into_owned(&self) -> CreateContractArgs {
        CreateContractArgs {
            contract_id_preimage: self.contract_id_preimage.into_owned(),
            executable: self.executable.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&CreateContractArgsView<'_>> for CreateContractArgs {
    #[must_use]
    fn from(v: &CreateContractArgsView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<CreateContractArgsView<'_>> for CreateContractArgs {
    #[must_use]
    fn from(v: CreateContractArgsView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for CreateContractArgsView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id_preimage.write_xdr(w)?;
            self.executable.write_xdr(w)?;
            Ok(())
        })
    }
}
