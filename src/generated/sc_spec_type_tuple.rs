#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScSpecTypeTuple is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeTuple
/// {
///     SCSpecTypeDef valueTypes<12>;
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
pub struct ScSpecTypeTuple {
    pub value_types: VecM<ScSpecTypeDef, 12>,
}

impl ReadXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                value_types: VecM::<ScSpecTypeDef, 12>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_types.write_xdr(w)?;
            Ok(())
        })
    }
}
