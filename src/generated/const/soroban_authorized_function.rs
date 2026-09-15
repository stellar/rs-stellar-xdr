#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizedFunction is a borrowing equivalent of [`SorobanAuthorizedFunction`](super::super::SorobanAuthorizedFunction)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum SorobanAuthorizedFunction {
    ContractFn(InvokeContractArgs),
    CreateContractHostFn(CreateContractArgs),
    CreateContractV2HostFn(CreateContractArgsV2),
}

impl SorobanAuthorizedFunction {
    #[must_use]
    pub const fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ContractFn(_) => SorobanAuthorizedFunctionType::ContractFn,
            Self::CreateContractHostFn(_) => SorobanAuthorizedFunctionType::CreateContractHostFn,
            Self::CreateContractV2HostFn(_) => {
                SorobanAuthorizedFunctionType::CreateContractV2HostFn
            }
        }
    }
}

impl SorobanAuthorizedFunction {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_authorized_function(self);
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
        w.write_type_soroban_authorized_function(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SorobanAuthorizedFunction`], mirroring `<SorobanAuthorizedFunction as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_authorized_function(&mut self, v: &SorobanAuthorizedFunction) {
        let d = v.discriminant();
        self.write_type_soroban_authorized_function_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            SorobanAuthorizedFunction::ContractFn(value) => {
                self.write_type_invoke_contract_args(value);
            }
            SorobanAuthorizedFunction::CreateContractHostFn(value) => {
                self.write_type_create_contract_args(value);
            }
            SorobanAuthorizedFunction::CreateContractV2HostFn(value) => {
                self.write_type_create_contract_args_v2(value);
            }
        }
    }
}
