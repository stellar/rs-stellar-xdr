#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TrustLineEntryV1 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             Liabilities liabilities;
///
///             union switch (int v)
///             {
///             case 0:
///                 void;
///             case 2:
///                 TrustLineEntryExtensionV2 v2;
///             }
///             ext;
///         }
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
pub struct TrustLineEntryV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryV1Ext,
}

impl ReadXdr for TrustLineEntryV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liabilities: Liabilities::read_xdr(r)?,
                ext: TrustLineEntryV1Ext::read_xdr(r)?,
            })
        })
    }
}

impl TrustLineEntryV1 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.liabilities.const_to_xdr(w);
        self.ext.const_to_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for TrustLineEntryV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liabilities.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
