#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtEnumV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTEnumV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTEnumCaseV0 cases<>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScSpecUdtEnumV0 {
    pub doc: StringM::<1024>,
    pub lib: StringM::<80>,
    pub name: StringM::<60>,
    pub cases: VecM<ScSpecUdtEnumCaseV0>,
}

impl ReadXdr for ScSpecUdtEnumV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtEnumCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtEnumV0 {
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
