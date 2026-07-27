#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// String32 is an XDR Typedef defined as:
///
/// ```text
/// typedef string string32<32>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct String32(pub StringM<32>);

impl From<String32> for StringM<32> {
    #[must_use]
    fn from(x: String32) -> Self {
        x.0
    }
}

impl From<StringM<32>> for String32 {
    #[must_use]
    fn from(x: StringM<32>) -> Self {
        String32(x)
    }
}

impl AsRef<StringM<32>> for String32 {
    #[must_use]
    fn as_ref(&self) -> &StringM<32> {
        &self.0
    }
}

impl ReadXdr for String32 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::<32>::read_xdr(r)?;
            let v = String32(i);
            Ok(v)
        })
    }
}

impl WriteXdr for String32 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for String32 {
    type Target = StringM<32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String32> for Vec<u8> {
    #[must_use]
    fn from(x: String32) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for String32 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(String32(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for String32 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(String32(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for String32 {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for String32 {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

/// String32Ref is a borrowing equivalent of [`String32`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct String32Ref<'a>(pub StringMRef<'a, 32>);

#[cfg(feature = "alloc")]
impl From<&String32Ref<'_>> for String32 {
    #[must_use]
    fn from(v: &String32Ref<'_>) -> Self {
        Self(v.0.to_stringm())
    }
}

#[cfg(feature = "alloc")]
impl From<String32Ref<'_>> for String32 {
    #[must_use]
    fn from(v: String32Ref<'_>) -> Self {
        Self::from(&v)
    }
}
