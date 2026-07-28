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

/// InvokeHostFunctionSuccessPreImageRef is a borrowing equivalent of [`InvokeHostFunctionSuccessPreImage`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeHostFunctionSuccessPreImageRef<'a> {
    pub return_value: ScValRef<'a>,
    pub events: VecMRef<'a, ContractEventRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&InvokeHostFunctionSuccessPreImageRef<'_>> for InvokeHostFunctionSuccessPreImage {
    #[must_use]
    fn from(v: &InvokeHostFunctionSuccessPreImageRef<'_>) -> Self {
        Self {
            return_value: (&v.return_value).into(),
            events: v.events.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<InvokeHostFunctionSuccessPreImageRef<'_>> for InvokeHostFunctionSuccessPreImage {
    #[must_use]
    fn from(v: InvokeHostFunctionSuccessPreImageRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for InvokeHostFunctionSuccessPreImageRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.return_value.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}
