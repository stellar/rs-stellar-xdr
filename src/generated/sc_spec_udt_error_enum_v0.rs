#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtErrorEnumV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTErrorEnumV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTErrorEnumCaseV0 cases<>;
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
pub struct ScSpecUdtErrorEnumV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub cases: VecM<ScSpecUdtErrorEnumCaseV0>,
}

impl ReadXdr for ScSpecUdtErrorEnumV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtErrorEnumCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtErrorEnumV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtErrorEnumV0Ref is a borrowing equivalent of [`ScSpecUdtErrorEnumV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtErrorEnumV0Ref<'a> {
    pub doc: StringMRef<'a, 1024>,
    pub lib: StringMRef<'a, 80>,
    pub name: StringMRef<'a, 60>,
    pub cases: VecMRef<'a, ScSpecUdtErrorEnumCaseV0Ref<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtErrorEnumV0Ref<'_>> for ScSpecUdtErrorEnumV0 {
    #[must_use]
    fn from(v: &ScSpecUdtErrorEnumV0Ref<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            lib: v.lib.to_stringm(),
            name: v.name.to_stringm(),
            cases: v.cases.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtErrorEnumV0Ref<'_>> for ScSpecUdtErrorEnumV0 {
    #[must_use]
    fn from(v: ScSpecUdtErrorEnumV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}
