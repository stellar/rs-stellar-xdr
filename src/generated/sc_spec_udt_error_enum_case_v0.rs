#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtErrorEnumCaseV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTErrorEnumCaseV0
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
pub struct ScSpecUdtErrorEnumCaseV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: StringM<60>,
    pub value: u32,
}

impl ReadXdr for ScSpecUdtErrorEnumCaseV0 {
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

impl WriteXdr for ScSpecUdtErrorEnumCaseV0 {
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

/// ScSpecUdtErrorEnumCaseV0View is a borrowing equivalent of [`ScSpecUdtErrorEnumCaseV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtErrorEnumCaseV0View<'a> {
    pub doc: StringMView<'a, SC_SPEC_DOC_LIMIT>,
    pub name: StringMView<'a, 60>,
    pub value: u32,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecUdtErrorEnumCaseV0View<'_> {
    type Owned = ScSpecUdtErrorEnumCaseV0;
    fn into_owned(&self) -> ScSpecUdtErrorEnumCaseV0 {
        ScSpecUdtErrorEnumCaseV0 {
            doc: self.doc.into_owned(),
            name: self.name.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtErrorEnumCaseV0View<'_>> for ScSpecUdtErrorEnumCaseV0 {
    #[must_use]
    fn from(v: &ScSpecUdtErrorEnumCaseV0View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtErrorEnumCaseV0View<'_>> for ScSpecUdtErrorEnumCaseV0 {
    #[must_use]
    fn from(v: ScSpecUdtErrorEnumCaseV0View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecUdtErrorEnumCaseV0View<'_> {
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
