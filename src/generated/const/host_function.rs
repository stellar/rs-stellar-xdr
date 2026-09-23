#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HostFunction is a borrowing equivalent of [`HostFunction`](super::super::HostFunction)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum HostFunction {
    InvokeContract(InvokeContractArgs),
    CreateContract(CreateContractArgs),
    UploadContractWasm(BytesM),
    CreateContractV2(CreateContractArgsV2),
}

impl HostFunction {
    #[must_use]
    pub const fn discriminant(&self) -> HostFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::InvokeContract(_) => HostFunctionType::InvokeContract,
            Self::CreateContract(_) => HostFunctionType::CreateContract,
            Self::UploadContractWasm(_) => HostFunctionType::UploadContractWasm,
            Self::CreateContractV2(_) => HostFunctionType::CreateContractV2,
        }
    }
}

impl HostFunction {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_host_function(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_host_function(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`HostFunction`], mirroring `<HostFunction as WriteXdr>::write_xdr`.
    pub const fn write_type_host_function(&mut self, v: &HostFunction) {
        let d = v.discriminant();
        self.write_type_host_function_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            HostFunction::InvokeContract(value) => {
                self.write_type_invoke_contract_args(value);
            }
            HostFunction::CreateContract(value) => {
                self.write_type_create_contract_args(value);
            }
            HostFunction::UploadContractWasm(value) => {
                self.write_var_opaque(value.as_slice());
            }
            HostFunction::CreateContractV2(value) => {
                self.write_type_create_contract_args_v2(value);
            }
        }
    }
}
