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

/// CreateContractArgsV2Const is a borrowing equivalent of [`CreateContractArgsV2`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateContractArgsV2Const {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutableConst,
    pub constructor_args: VecMConst<ScValConst>,
}

#[cfg(feature = "const")]
impl CreateContractArgsV2Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_create_contract_args_v2(self);
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
        w.write_type_create_contract_args_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`CreateContractArgsV2`], mirroring `<CreateContractArgsV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_create_contract_args_v2(&mut self, v: &CreateContractArgsV2Const) {
        self.write_type_contract_id_preimage(&v.contract_id_preimage);
        self.write_type_contract_executable(&v.executable);
        self.write_type_vec_sc_val(&v.constructor_args);
    }
}
