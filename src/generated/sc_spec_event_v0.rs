#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEventV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEventV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     SCSymbol name;
///     SCSymbol prefixTopics<2>;
///     SCSpecEventParamV0 params<>;
///     SCSpecEventDataFormat dataFormat;
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
pub struct ScSpecEventV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub lib: StringM<80>,
    pub name: ScSymbol,
    pub prefix_topics: VecM<ScSymbol, 2>,
    pub params: VecM<ScSpecEventParamV0>,
    pub data_format: ScSpecEventDataFormat,
}

impl ReadXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: ScSymbol::read_xdr(r)?,
                prefix_topics: VecM::<ScSymbol, 2>::read_xdr(r)?,
                params: VecM::<ScSpecEventParamV0>::read_xdr(r)?,
                data_format: ScSpecEventDataFormat::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.prefix_topics.write_xdr(w)?;
            self.params.write_xdr(w)?;
            self.data_format.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecEventV0Ref is a borrowing equivalent of [`ScSpecEventV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecEventV0Ref<'a> {
    pub doc: StringMRef<'a, 1024>,
    pub lib: StringMRef<'a, 80>,
    pub name: ScSymbolRef<'a>,
    pub prefix_topics: VecMRef<'a, ScSymbolRef<'a>, 2>,
    pub params: VecMRef<'a, ScSpecEventParamV0Ref<'a>>,
    pub data_format: ScSpecEventDataFormat,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecEventV0Ref<'_>> for ScSpecEventV0 {
    #[must_use]
    fn from(v: &ScSpecEventV0Ref<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            lib: v.lib.to_stringm(),
            name: (&v.name).into(),
            prefix_topics: v.prefix_topics.to_vecm_from(),
            params: v.params.to_vecm_from(),
            data_format: v.data_format,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecEventV0Ref<'_>> for ScSpecEventV0 {
    #[must_use]
    fn from(v: ScSpecEventV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecEventV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.prefix_topics.write_xdr(w)?;
            self.params.write_xdr(w)?;
            self.data_format.write_xdr(w)?;
            Ok(())
        })
    }
}
