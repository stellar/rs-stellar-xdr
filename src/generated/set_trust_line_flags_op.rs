#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SetTrustLineFlagsOp is an XDR Struct defined as:
///
/// ```text
/// struct SetTrustLineFlagsOp
/// {
///     AccountID trustor;
///     Asset asset;
///
///     uint32 clearFlags; // which flags to clear
///     uint32 setFlags;   // which flags to set
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
pub struct SetTrustLineFlagsOp {
    pub trustor: AccountId,
    pub asset: Asset,
    pub clear_flags: u32,
    pub set_flags: u32,
}

impl ReadXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                trustor: AccountId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
                clear_flags: u32::read_xdr(r)?,
                set_flags: u32::read_xdr(r)?,
            })
        })
    }
}

impl SetTrustLineFlagsOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.trustor.const_write_xdr(w);
        self.asset.const_write_xdr(w);
        w.write_u32(self.clear_flags);
        w.write_u32(self.set_flags);
        w.leave_depth();
    }
}

impl WriteXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.trustor.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.clear_flags.write_xdr(w)?;
            self.set_flags.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
