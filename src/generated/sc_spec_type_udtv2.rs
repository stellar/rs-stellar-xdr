#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeUdtv2 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeUDTV2
/// {
///     opaque hash[8];
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
pub struct ScSpecTypeUdtv2 {
    pub hash: [u8; 8],
    pub name: StringM<60>,
}

impl ReadXdr for ScSpecTypeUdtv2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hash: <[u8; 8]>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeUdtv2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hash.write_xdr(w)?;
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}
