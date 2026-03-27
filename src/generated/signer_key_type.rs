#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignerKeyType is an XDR Enum defined as:
///
/// ```text
/// enum SignerKeyType
/// {
///     SIGNER_KEY_TYPE_ED25519 = KEY_TYPE_ED25519,
///     SIGNER_KEY_TYPE_PRE_AUTH_TX = KEY_TYPE_PRE_AUTH_TX,
///     SIGNER_KEY_TYPE_HASH_X = KEY_TYPE_HASH_X,
///     SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD = KEY_TYPE_ED25519_SIGNED_PAYLOAD
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum SignerKeyType {
    #[cfg_attr(feature = "alloc", default)]
    Ed25519 = 0,
    PreAuthTx = 1,
    HashX = 2,
    Ed25519SignedPayload = 3,
}

impl SignerKeyType {
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
            Self::Ed25519 => "Ed25519",
            Self::PreAuthTx => "PreAuthTx",
            Self::HashX => "HashX",
            Self::Ed25519SignedPayload => "Ed25519SignedPayload",
        }
    }

    #[must_use]
    pub const fn variants() -> [SignerKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SignerKeyType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<SignerKeyType> for SignerKeyType {
    fn variants() -> slice::Iter<'static, SignerKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for SignerKeyType {}

impl fmt::Display for SignerKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SignerKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => SignerKeyType::Ed25519,
            1 => SignerKeyType::PreAuthTx,
            2 => SignerKeyType::HashX,
            3 => SignerKeyType::Ed25519SignedPayload,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SignerKeyType> for i32 {
    #[must_use]
    fn from(e: SignerKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for SignerKeyType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for SignerKeyType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
