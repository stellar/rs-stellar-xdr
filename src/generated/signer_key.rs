#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignerKey is an XDR Union defined as:
///
/// ```text
/// union SignerKey switch (SignerKeyType type)
/// {
/// case SIGNER_KEY_TYPE_ED25519:
///     uint256 ed25519;
/// case SIGNER_KEY_TYPE_PRE_AUTH_TX:
///     /* SHA-256 Hash of TransactionSignaturePayload structure */
///     uint256 preAuthTx;
/// case SIGNER_KEY_TYPE_HASH_X:
///     /* Hash of random 256 bit preimage X */
///     uint256 hashX;
/// case SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD:
///     struct
///     {
///         /* Public key that must sign the payload. */
///         uint256 ed25519;
///         /* Payload to be raw signed by ed25519. */
///         opaque payload<64>;
///     } ed25519SignedPayload;
/// };
/// ```
///
// union with discriminant SignerKeyType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayload),
}

#[cfg(feature = "alloc")]
impl Default for SignerKey {
    fn default() -> Self {
        Self::Ed25519(Uint256::default())
    }
}

impl SignerKey {
    const _VARIANTS: &[SignerKeyType] = &[
        SignerKeyType::Ed25519,
        SignerKeyType::PreAuthTx,
        SignerKeyType::HashX,
        SignerKeyType::Ed25519SignedPayload,
    ];
    pub const VARIANTS: [SignerKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Ed25519", "PreAuthTx", "HashX", "Ed25519SignedPayload"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Ed25519(_) => "Ed25519",
            Self::PreAuthTx(_) => "PreAuthTx",
            Self::HashX(_) => "HashX",
            Self::Ed25519SignedPayload(_) => "Ed25519SignedPayload",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [SignerKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SignerKey {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SignerKeyType> for SignerKey {
    #[must_use]
    fn discriminant(&self) -> SignerKeyType {
        Self::discriminant(self)
    }
}

impl Variants<SignerKeyType> for SignerKey {
    fn variants() -> slice::Iter<'static, SignerKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SignerKeyType> for SignerKey {}

impl ReadXdr for SignerKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SignerKeyType = <SignerKeyType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SignerKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
                SignerKeyType::PreAuthTx => Self::PreAuthTx(Uint256::read_xdr(r)?),
                SignerKeyType::HashX => Self::HashX(Uint256::read_xdr(r)?),
                SignerKeyType::Ed25519SignedPayload => {
                    Self::Ed25519SignedPayload(SignerKeyEd25519SignedPayload::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SignerKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Ed25519(v) => v.write_xdr(w)?,
                Self::PreAuthTx(v) => v.write_xdr(w)?,
                Self::HashX(v) => v.write_xdr(w)?,
                Self::Ed25519SignedPayload(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// SignerKeyView is a borrowing equivalent of [`SignerKey`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum SignerKeyView<'a> {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayloadView<'a>),
}

#[cfg(feature = "alloc")]
impl From<&SignerKeyView<'_>> for SignerKey {
    #[must_use]
    fn from(v: &SignerKeyView<'_>) -> Self {
        #[allow(clippy::match_same_arms)]
        match v {
            SignerKeyView::Ed25519(value) => Self::Ed25519(value.clone()),
            SignerKeyView::PreAuthTx(value) => Self::PreAuthTx(value.clone()),
            SignerKeyView::HashX(value) => Self::HashX(value.clone()),
            SignerKeyView::Ed25519SignedPayload(value) => Self::Ed25519SignedPayload(value.into()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SignerKeyView<'_>> for SignerKey {
    #[must_use]
    fn from(v: SignerKeyView<'_>) -> Self {
        Self::from(&v)
    }
}

impl SignerKeyView<'_> {
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

impl WriteXdr for SignerKeyView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Ed25519(v) => v.write_xdr(w)?,
                Self::PreAuthTx(v) => v.write_xdr(w)?,
                Self::HashX(v) => v.write_xdr(w)?,
                Self::Ed25519SignedPayload(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SignerKeyView<'_> {
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
        w.write_type_signer_key(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SignerKey`], mirroring `<SignerKey as WriteXdr>::write_xdr`.
    pub const fn write_type_signer_key(&mut self, v: &SignerKeyView<'_>) {
        let d = v.discriminant();
        self.write_type_signer_key_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            SignerKeyView::Ed25519(value) => {
                self.write_type_uint256(value);
            }
            SignerKeyView::PreAuthTx(value) => {
                self.write_type_uint256(value);
            }
            SignerKeyView::HashX(value) => {
                self.write_type_uint256(value);
            }
            SignerKeyView::Ed25519SignedPayload(value) => {
                self.write_type_signer_key_ed25519_signed_payload(value);
            }
        }
    }

    /// Serializes a variable-length array of [`SignerKey`], mirroring `<VecM<SignerKey, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_signer_key<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, SignerKeyView<'_>, MAX>,
    ) {
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
