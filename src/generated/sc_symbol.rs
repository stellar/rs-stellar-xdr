#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSymbol is an XDR Typedef defined as:
///
/// ```text
/// typedef string SCSymbol<SCSYMBOL_LIMIT>;
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
pub struct ScSymbol(pub StringM<SCSYMBOL_LIMIT>);

impl From<ScSymbol> for StringM<SCSYMBOL_LIMIT> {
    #[must_use]
    fn from(x: ScSymbol) -> Self {
        x.0
    }
}

impl From<StringM<SCSYMBOL_LIMIT>> for ScSymbol {
    #[must_use]
    fn from(x: StringM<SCSYMBOL_LIMIT>) -> Self {
        ScSymbol(x)
    }
}

impl AsRef<StringM<SCSYMBOL_LIMIT>> for ScSymbol {
    #[must_use]
    fn as_ref(&self) -> &StringM<SCSYMBOL_LIMIT> {
        &self.0
    }
}

impl ReadXdr for ScSymbol {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::<SCSYMBOL_LIMIT>::read_xdr(r)?;
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
    type Target = StringM<SCSYMBOL_LIMIT>;
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

/// ScSymbolView is a borrowing equivalent of [`ScSymbol`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSymbolView<'a>(pub StringMView<'a, SCSYMBOL_LIMIT>);

#[cfg(feature = "alloc")]
impl IntoOwned for ScSymbolView<'_> {
    type Owned = ScSymbol;
    fn into_owned(self) -> ScSymbol {
        ScSymbol(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSymbolView<'_>> for ScSymbol {
    #[must_use]
    fn from(v: &ScSymbolView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSymbolView<'_>> for ScSymbol {
    #[must_use]
    fn from(v: ScSymbolView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSymbolView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}
