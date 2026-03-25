#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScSpecTypeVec is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeVec
/// {
///     SCSpecTypeDef elementType;
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
pub struct ScSpecTypeVec {
    pub element_type: Box<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecTypeVec {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                element_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeVec {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.element_type.write_xdr(w)?;
            Ok(())
        })
    }
}
