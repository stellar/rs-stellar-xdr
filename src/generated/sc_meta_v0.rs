#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScMetaV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCMetaV0
/// {
///     string key<>;
///     string val<>;
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
pub struct ScMetaV0 {
    pub key: StringM,
    pub val: StringM,
}

impl ReadXdr for ScMetaV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key: StringM::read_xdr(r)?,
                val: StringM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScMetaV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}
