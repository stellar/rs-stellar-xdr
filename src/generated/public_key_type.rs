#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PublicKeyType is an XDR Enum defined as:
///
/// ```text
/// enum PublicKeyType
/// {
///     PUBLIC_KEY_TYPE_ED25519 = KEY_TYPE_ED25519
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
pub enum PublicKeyType {
    #[cfg_attr(feature = "alloc", default)]
    PublicKeyTypeEd25519 = 0,
}

impl PublicKeyType {
    const _VARIANTS: &[PublicKeyType] = &[PublicKeyType::PublicKeyTypeEd25519];
    pub const VARIANTS: [PublicKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["PublicKeyTypeEd25519"];
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
            Self::PublicKeyTypeEd25519 => "PublicKeyTypeEd25519",
        }
    }

    #[must_use]
    pub const fn variants() -> [PublicKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PublicKeyType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<PublicKeyType> for PublicKeyType {
    fn variants() -> slice::Iter<'static, PublicKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for PublicKeyType {}

impl fmt::Display for PublicKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PublicKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => PublicKeyType::PublicKeyTypeEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PublicKeyType> for i32 {
    #[must_use]
    fn from(e: PublicKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for PublicKeyType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for PublicKeyType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
