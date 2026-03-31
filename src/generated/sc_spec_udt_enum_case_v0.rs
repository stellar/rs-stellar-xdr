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
    pub doc: StringM<1024>,
    pub name: StringM<60>,
    pub value: u32,
}

impl ReadXdr for ScSpecUdtEnumCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
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
