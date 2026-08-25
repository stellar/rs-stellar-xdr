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

/// ScSpecFunctionV0View is a borrowing equivalent of [`ScSpecFunctionV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecFunctionV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub name: ScSymbolView<'a>,
    pub inputs: VecMView<'a, ScSpecFunctionInputV0View<'a>>,
    pub outputs: VecMView<'a, ScSpecTypeDefView<'a>, 1>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecFunctionV0View<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: &ScSpecFunctionV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            name: (&v.name).into(),
            inputs: v.inputs.to_vecm(),
            outputs: v.outputs.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecFunctionV0View<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: ScSpecFunctionV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecFunctionV0View<'_> {
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
