#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InvokeHostFunctionSuccessPreImage is an XDR Struct defined as:
///
/// ```text
/// struct InvokeHostFunctionSuccessPreImage
/// {
///     SCVal returnValue;
///     ContractEvent events<>;
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
pub struct InvokeHostFunctionSuccessPreImage {
    pub return_value: ScVal,
    pub events: VecM<ContractEvent>,
}

impl ReadXdr for InvokeHostFunctionSuccessPreImage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                return_value: ScVal::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InvokeHostFunctionSuccessPreImage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.return_value.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}
