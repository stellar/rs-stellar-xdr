#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InvokeContractArgs is an XDR Struct defined as:
///
/// ```text
/// struct InvokeContractArgs {
///     SCAddress contractAddress;
///     SCSymbol functionName;
///     SCVal args<>;
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
pub struct InvokeContractArgs {
    pub contract_address: ScAddress,
    pub function_name: ScSymbol,
    pub args: VecM<ScVal>,
}

impl ReadXdr for InvokeContractArgs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_address: ScAddress::read_xdr(r)?,
                function_name: ScSymbol::read_xdr(r)?,
                args: VecM::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InvokeContractArgs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_address.write_xdr(w)?;
            self.function_name.write_xdr(w)?;
            self.args.write_xdr(w)?;
            Ok(())
        })
    }
}

/// InvokeContractArgsView is a borrowing equivalent of [`InvokeContractArgs`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeContractArgsView<'a> {
    pub contract_address: ScAddress,
    pub function_name: ScSymbolView<'a>,
    pub args: VecMView<'a, ScValView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for InvokeContractArgsView<'_> {
    type Owned = InvokeContractArgs;
    fn into_owned(&self) -> InvokeContractArgs {
        InvokeContractArgs {
            contract_address: self.contract_address.into_owned(),
            function_name: self.function_name.into_owned(),
            args: self.args.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&InvokeContractArgsView<'_>> for InvokeContractArgs {
    #[must_use]
    fn from(v: &InvokeContractArgsView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<InvokeContractArgsView<'_>> for InvokeContractArgs {
    #[must_use]
    fn from(v: InvokeContractArgsView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for InvokeContractArgsView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_address.write_xdr(w)?;
            self.function_name.write_xdr(w)?;
            self.args.write_xdr(w)?;
            Ok(())
        })
    }
}
