#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AllowTrustOp is an XDR Struct defined as:
///
/// ```text
/// struct AllowTrustOp
/// {
///     AccountID trustor;
///     AssetCode asset;
///
///     // One of 0, AUTHORIZED_FLAG, or AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG
///     uint32 authorize;
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
pub struct AllowTrustOp {
    pub trustor: AccountId,
    pub asset: AssetCode,
    pub authorize: u32,
}

impl ReadXdr for AllowTrustOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                trustor: AccountId::read_xdr(r)?,
                asset: AssetCode::read_xdr(r)?,
                authorize: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AllowTrustOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.trustor.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.authorize.write_xdr(w)?;
            Ok(())
        })
    }
}
