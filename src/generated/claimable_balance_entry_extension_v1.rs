#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClaimableBalanceEntryExtensionV1 is an XDR Struct defined as:
///
/// ```text
/// struct ClaimableBalanceEntryExtensionV1
/// {
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
///
///     uint32 flags; // see ClaimableBalanceFlags
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
pub struct ClaimableBalanceEntryExtensionV1 {
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    pub flags: u32,
}

impl ReadXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ClaimableBalanceEntryExtensionV1Ext::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            Ok(())
        })
    }
}
