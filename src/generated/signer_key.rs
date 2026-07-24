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

impl SignerKey {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(v) => {
                v.const_write_xdr(w);
            }
            Self::PreAuthTx(v) => {
                v.const_write_xdr(w);
            }
            Self::HashX(v) => {
                v.const_write_xdr(w);
            }
            Self::Ed25519SignedPayload(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for SignerKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
