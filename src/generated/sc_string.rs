#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScString is an XDR Typedef defined as:
///
/// ```text
/// typedef string SCString<>;
/// ```
///
#[cfg_eval::cfg_eval]
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
pub struct ScString(pub StringM);


impl From<ScString> for StringM {
    #[must_use]
    fn from(x: ScString) -> Self {
        x.0
    }
}

impl From<StringM> for ScString {
    #[must_use]
    fn from(x: StringM) -> Self {
        ScString(x)
    }
}

impl AsRef<StringM> for ScString {
    #[must_use]
    fn as_ref(&self) -> &StringM {
        &self.0
    }
}

impl ReadXdr for ScString {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::read_xdr(r)?;
            let v = ScString(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ScString {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| { self.0.write_xdr(w) })
    }
}

impl Deref for ScString {
    type Target = StringM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScString> for Vec<u8> {
    #[must_use]
    fn from(x: ScString) -> Self {
        x.0.0
    }
}

impl TryFrom<Vec<u8>> for ScString {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(ScString(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScString {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(ScString(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for ScString {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0.0
    }
}

impl AsRef<[u8]> for ScString {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0.0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.0
    }
}
