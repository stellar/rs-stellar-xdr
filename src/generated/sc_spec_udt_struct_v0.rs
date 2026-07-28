#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtStructV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTStructV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTStructFieldV0 fields<>;
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
pub struct ScSpecUdtStructV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub fields: VecM<ScSpecUdtStructFieldV0>,
}

impl ReadXdr for ScSpecUdtStructV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                fields: VecM::<ScSpecUdtStructFieldV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtStructV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.fields.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtStructV0Ref is a borrowing equivalent of [`ScSpecUdtStructV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtStructV0Ref<'a> {
    pub doc: StringMRef<'a, 1024>,
    pub lib: StringMRef<'a, 80>,
    pub name: StringMRef<'a, 60>,
    pub fields: VecMRef<'a, ScSpecUdtStructFieldV0Ref<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtStructV0Ref<'_>> for ScSpecUdtStructV0 {
    #[must_use]
    fn from(v: &ScSpecUdtStructV0Ref<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            lib: v.lib.to_stringm(),
            name: v.name.to_stringm(),
            fields: v.fields.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtStructV0Ref<'_>> for ScSpecUdtStructV0 {
    #[must_use]
    fn from(v: ScSpecUdtStructV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecUdtStructV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.fields.write_xdr(w)?;
            Ok(())
        })
    }
}
