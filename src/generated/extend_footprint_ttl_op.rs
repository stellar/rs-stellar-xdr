#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ExtendFootprintTtlOp is an XDR Struct defined as:
///
/// ```text
/// struct ExtendFootprintTTLOp
/// {
///     ExtensionPoint ext;
///     uint32 extendTo;
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
pub struct ExtendFootprintTtlOp {
    pub ext: ExtensionPoint,
    pub extend_to: u32,
}

impl ReadXdr for ExtendFootprintTtlOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                extend_to: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ExtendFootprintTtlOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.extend_to.write_xdr(w)?;
            Ok(())
        })
    }
}
