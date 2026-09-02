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
impl From<&InvokeContractArgsView<'_>> for InvokeContractArgs {
    #[must_use]
    fn from(v: &InvokeContractArgsView<'_>) -> Self {
        Self {
            contract_address: v.contract_address.clone(),
            function_name: (&v.function_name).into(),
            args: v.args.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<InvokeContractArgsView<'_>> for InvokeContractArgs {
    #[must_use]
    fn from(v: InvokeContractArgsView<'_>) -> Self {
        Self::from(&v)
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

#[cfg(feature = "const")]
impl InvokeContractArgsView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_invoke_contract_args(self);
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
        w.write_type_invoke_contract_args(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`InvokeContractArgs`], mirroring `<InvokeContractArgs as WriteXdr>::write_xdr`.
    pub const fn write_type_invoke_contract_args(&mut self, v: &InvokeContractArgsView<'_>) {
        self.write_type_sc_address(&v.contract_address);
        self.write_type_sc_symbol(&v.function_name);
        self.write_type_vec_sc_val(&v.args);
    }
}
