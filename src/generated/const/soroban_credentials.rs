#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanCredentials is a borrowing equivalent of [`SorobanCredentials`](super::super::SorobanCredentials)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum SorobanCredentials {
    SourceAccount,
    Address(SorobanAddressCredentials),
    AddressV2(SorobanAddressCredentials),
    AddressWithDelegates(SorobanAddressCredentialsWithDelegates),
}

impl SorobanCredentials {
    #[must_use]
    pub const fn discriminant(&self) -> SorobanCredentialsType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SourceAccount => SorobanCredentialsType::SourceAccount,
            Self::Address(_) => SorobanCredentialsType::Address,
            Self::AddressV2(_) => SorobanCredentialsType::AddressV2,
            Self::AddressWithDelegates(_) => SorobanCredentialsType::AddressWithDelegates,
        }
    }
}

impl SorobanCredentials {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_credentials(self);
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
        w.write_type_soroban_credentials(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SorobanCredentials`], mirroring `<SorobanCredentials as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_credentials(&mut self, v: &SorobanCredentials) {
        let d = v.discriminant();
        self.write_type_soroban_credentials_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            SorobanCredentials::SourceAccount => {}
            SorobanCredentials::Address(value) => {
                self.write_type_soroban_address_credentials(value);
            }
            SorobanCredentials::AddressV2(value) => {
                self.write_type_soroban_address_credentials(value);
            }
            SorobanCredentials::AddressWithDelegates(value) => {
                self.write_type_soroban_address_credentials_with_delegates(value);
            }
        }
    }
}
