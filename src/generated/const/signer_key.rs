#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignerKey is a borrowing equivalent of [`SignerKey`](super::super::SignerKey)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayload),
}

impl SignerKey {
    #[must_use]
    pub const fn discriminant(&self) -> SignerKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => SignerKeyType::Ed25519,
            Self::PreAuthTx(_) => SignerKeyType::PreAuthTx,
            Self::HashX(_) => SignerKeyType::HashX,
            Self::Ed25519SignedPayload(_) => SignerKeyType::Ed25519SignedPayload,
        }
    }
}

impl SignerKey {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_signer_key(self);
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
        w.write_type_signer_key(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SignerKey`], mirroring `<SignerKey as WriteXdr>::write_xdr`.
    pub const fn write_type_signer_key(&mut self, v: &SignerKey) {
        let d = v.discriminant();
        self.write_type_signer_key_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            SignerKey::Ed25519(value) => {
                self.write_type_uint256(value);
            }
            SignerKey::PreAuthTx(value) => {
                self.write_type_uint256(value);
            }
            SignerKey::HashX(value) => {
                self.write_type_uint256(value);
            }
            SignerKey::Ed25519SignedPayload(value) => {
                self.write_type_signer_key_ed25519_signed_payload(value);
            }
        }
    }

    /// Serializes a variable-length array of [`SignerKey`], mirroring `<VecM<SignerKey, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_signer_key<const MAX: u32>(&mut self, v: &VecM<SignerKey, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_signer_key(&s[i]);
            i += 1;
        }
    }
}
