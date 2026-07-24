#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScEnvMetaEntryInterfaceVersion is an XDR NestedStruct defined as:
///
/// ```text
/// struct {
///         uint32 protocol;
///         uint32 preRelease;
///     }
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
pub struct ScEnvMetaEntryInterfaceVersion {
    pub protocol: u32,
    pub pre_release: u32,
}

impl ReadXdr for ScEnvMetaEntryInterfaceVersion {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                protocol: u32::read_xdr(r)?,
                pre_release: u32::read_xdr(r)?,
            })
        })
    }
}

impl ScEnvMetaEntryInterfaceVersion {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.protocol);
        w.write_u32(self.pre_release);
        w.leave_depth();
    }
}

impl WriteXdr for ScEnvMetaEntryInterfaceVersion {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.protocol.write_xdr(w)?;
            self.pre_release.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
