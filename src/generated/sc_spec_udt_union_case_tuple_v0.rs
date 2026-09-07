#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtUnionCaseTupleV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTUnionCaseTupleV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<60>;
///     SCSpecTypeDef type<>;
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
pub struct ScSpecUdtUnionCaseTupleV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: StringM<60>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde(rename = "type", alias = "type_")
    )]
    #[cfg_attr(feature = "schemars", schemars(rename = "type"))]
    pub type_: VecM<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecUdtUnionCaseTupleV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                type_: VecM::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtUnionCaseTupleV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtUnionCaseTupleV0Ref is a borrowing equivalent of [`ScSpecUdtUnionCaseTupleV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtUnionCaseTupleV0Ref<'a> {
    pub doc: StringMRef<'a, SC_SPEC_DOC_LIMIT>,
    pub name: StringMRef<'a, 60>,
    pub type_: VecMRef<'a, ScSpecTypeDefRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecUdtUnionCaseTupleV0Ref<'_> {
    type Owned = ScSpecUdtUnionCaseTupleV0;
    fn into_owned(self) -> ScSpecUdtUnionCaseTupleV0 {
        ScSpecUdtUnionCaseTupleV0 {
            doc: self.doc.into_owned(),
            name: self.name.into_owned(),
            type_: self.type_.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtUnionCaseTupleV0Ref<'_>> for ScSpecUdtUnionCaseTupleV0 {
    #[must_use]
    fn from(v: &ScSpecUdtUnionCaseTupleV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtUnionCaseTupleV0Ref<'_>> for ScSpecUdtUnionCaseTupleV0 {
    #[must_use]
    fn from(v: ScSpecUdtUnionCaseTupleV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecUdtUnionCaseTupleV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            Ok(())
        })
    }
}
