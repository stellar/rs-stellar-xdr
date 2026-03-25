#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScSymbol is an XDR Typedef defined as:
///
/// ```text
/// typedef string SCSymbol<SCSYMBOL_LIMIT>;
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
pub struct ScSymbol(pub StringM<32>);

impl From<ScSymbol> for StringM<32> {
    #[must_use]
    fn from(x: ScSymbol) -> Self {
        x.0
    }
}

impl From<StringM<32>> for ScSymbol {
    #[must_use]
    fn from(x: StringM<32>) -> Self {
        ScSymbol(x)
    }
}

impl AsRef<StringM<32>> for ScSymbol {
    #[must_use]
    fn as_ref(&self) -> &StringM<32> {
        &self.0
    }
}

impl ReadXdr for ScSymbol {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::<32>::read_xdr(r)?;
            let v = ScSymbol(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ScSymbol {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ScSymbol {
    type Target = StringM<32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScSymbol> for Vec<u8> {
    #[must_use]
    fn from(x: ScSymbol) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for ScSymbol {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(ScSymbol(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScSymbol {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(ScSymbol(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for ScSymbol {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for ScSymbol {
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
