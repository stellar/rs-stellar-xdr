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

/// InvokeHostFunctionSuccessPreImageConst is a borrowing equivalent of [`InvokeHostFunctionSuccessPreImage`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeHostFunctionSuccessPreImageConst {
    pub return_value: ScValConst,
    pub events: VecMConst<ContractEventConst>,
}

#[cfg(feature = "const")]
impl InvokeHostFunctionSuccessPreImageConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_invoke_host_function_success_pre_image(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_invoke_host_function_success_pre_image(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`InvokeHostFunctionSuccessPreImage`], mirroring `<InvokeHostFunctionSuccessPreImage as WriteXdr>::write_xdr`.
    pub const fn write_type_invoke_host_function_success_pre_image(
        &mut self,
        v: &InvokeHostFunctionSuccessPreImageConst,
    ) {
        self.write_type_sc_val(&v.return_value);
        self.write_type_vec_contract_event(&v.events);
    }
}
