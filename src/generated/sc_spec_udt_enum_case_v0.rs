#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtEnumCaseV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTEnumCaseV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<60>;
///     uint32 value;
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
pub struct ScSpecUdtEnumCaseV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: StringM<60>,
    pub value: u32,
}

impl ReadXdr for ScSpecUdtEnumCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                value: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtEnumCaseV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtEnumCaseV0View is a borrowing equivalent of [`ScSpecUdtEnumCaseV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtEnumCaseV0View<'a> {
    pub doc: StringMView<'a, SC_SPEC_DOC_LIMIT>,
    pub name: StringMView<'a, 60>,
    pub value: u32,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecUdtEnumCaseV0View<'_> {
    type Owned = ScSpecUdtEnumCaseV0;
    fn into_owned(&self) -> ScSpecUdtEnumCaseV0 {
        ScSpecUdtEnumCaseV0 {
            doc: IntoOwned::into_owned(&self.doc),
            name: IntoOwned::into_owned(&self.name),
            value: IntoOwned::into_owned(&self.value),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtEnumCaseV0View<'_>> for ScSpecUdtEnumCaseV0 {
    #[must_use]
    fn from(v: &ScSpecUdtEnumCaseV0View<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtEnumCaseV0View<'_>> for ScSpecUdtEnumCaseV0 {
    #[must_use]
    fn from(v: ScSpecUdtEnumCaseV0View<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for ScSpecUdtEnumCaseV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}
