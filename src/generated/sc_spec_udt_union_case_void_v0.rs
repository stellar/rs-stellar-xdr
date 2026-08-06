#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtUnionCaseVoidV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTUnionCaseVoidV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<60>;
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
pub struct ScSpecUdtUnionCaseVoidV0 {
    pub doc: StringM<1024>,
    pub name: StringM<60>,
}

impl ReadXdr for ScSpecUdtUnionCaseVoidV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtUnionCaseVoidV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtUnionCaseVoidV0View is a borrowing equivalent of [`ScSpecUdtUnionCaseVoidV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtUnionCaseVoidV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub name: StringMView<'a, 60>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtUnionCaseVoidV0View<'_>> for ScSpecUdtUnionCaseVoidV0 {
    #[must_use]
    fn from(v: &ScSpecUdtUnionCaseVoidV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            name: v.name.to_stringm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtUnionCaseVoidV0View<'_>> for ScSpecUdtUnionCaseVoidV0 {
    #[must_use]
    fn from(v: ScSpecUdtUnionCaseVoidV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecUdtUnionCaseVoidV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}
