#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEventParamV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEventParamV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<30>;
///     SCSpecTypeDef type;
///     SCSpecEventParamLocationV0 location;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScSpecEventParamV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: StringM<30>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde(rename = "type", alias = "type_")
    )]
    #[cfg_attr(feature = "schemars", schemars(rename = "type"))]
    pub type_: ScSpecTypeDef,
    pub location: ScSpecEventParamLocationV0,
}

impl ReadXdr for ScSpecEventParamV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                name: StringM::<30>::read_xdr(r)?,
                type_: ScSpecTypeDef::read_xdr(r)?,
                location: ScSpecEventParamLocationV0::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEventParamV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.location.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecEventParamV0View is a borrowing equivalent of [`ScSpecEventParamV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecEventParamV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub name: StringMView<'a, 30>,
    pub type_: ScSpecTypeDefView<'a>,
    pub location: ScSpecEventParamLocationV0,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecEventParamV0View<'_>> for ScSpecEventParamV0 {
    #[must_use]
    fn from(v: &ScSpecEventParamV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            name: v.name.to_stringm(),
            type_: (&v.type_).into(),
            location: v.location,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecEventParamV0View<'_>> for ScSpecEventParamV0 {
    #[must_use]
    fn from(v: ScSpecEventParamV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecEventParamV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.location.write_xdr(w)?;
            Ok(())
        })
    }
}
