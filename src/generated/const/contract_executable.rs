#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractExecutable is a borrowing equivalent of [`ContractExecutable`](super::super::ContractExecutable)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum ContractExecutable {
    Wasm(Hash),
    StellarAsset,
    ExternalRef(ContractExecutableExternalRef),
}

impl ContractExecutable {
    #[must_use]
    pub const fn discriminant(&self) -> ContractExecutableType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Wasm(_) => ContractExecutableType::Wasm,
            Self::StellarAsset => ContractExecutableType::StellarAsset,
            Self::ExternalRef(_) => ContractExecutableType::ExternalRef,
        }
    }
}

impl ContractExecutable {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_contract_executable(self);
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
        w.write_type_contract_executable(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ContractExecutable`], mirroring `<ContractExecutable as WriteXdr>::write_xdr`.
    pub const fn write_type_contract_executable(&mut self, v: &ContractExecutable) {
        let d = v.discriminant();
        self.write_type_contract_executable_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ContractExecutable::Wasm(value) => {
                self.write_type_hash(value);
            }
            ContractExecutable::StellarAsset => {}
            ContractExecutable::ExternalRef(value) => {
                self.write_type_contract_executable_external_ref(value);
            }
        }
    }
}
