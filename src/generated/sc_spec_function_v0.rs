#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecFunctionV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecFunctionV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     SCSymbol name;
///     SCSpecFunctionInputV0 inputs<>;
///     SCSpecTypeDef outputs<1>;
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
pub struct ScSpecFunctionV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: ScSymbol,
    pub inputs: VecM<ScSpecFunctionInputV0>,
    pub outputs: VecM<ScSpecTypeDef, 1>,
}

impl ReadXdr for ScSpecFunctionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                name: ScSymbol::read_xdr(r)?,
                inputs: VecM::<ScSpecFunctionInputV0>::read_xdr(r)?,
                outputs: VecM::<ScSpecTypeDef, 1>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecFunctionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.inputs.write_xdr(w)?;
            self.outputs.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecFunctionV0Ref is a borrowing equivalent of [`ScSpecFunctionV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecFunctionV0Ref<'a> {
    pub doc: StringMRef<'a, SC_SPEC_DOC_LIMIT>,
    pub name: ScSymbolRef<'a>,
    pub inputs: VecMRef<'a, ScSpecFunctionInputV0Ref<'a>>,
    pub outputs: VecMRef<'a, ScSpecTypeDefRef<'a>, 1>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecFunctionV0Ref<'_> {
    type Owned = ScSpecFunctionV0;
    fn into_owned(self) -> ScSpecFunctionV0 {
        ScSpecFunctionV0 {
            doc: self.doc.into_owned(),
            name: self.name.into_owned(),
            inputs: self.inputs.into_owned(),
            outputs: self.outputs.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecFunctionV0Ref<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: &ScSpecFunctionV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecFunctionV0Ref<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: ScSpecFunctionV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecFunctionV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.inputs.write_xdr(w)?;
            self.outputs.write_xdr(w)?;
            Ok(())
        })
    }
}
