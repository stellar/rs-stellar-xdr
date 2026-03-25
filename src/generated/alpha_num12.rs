#[allow(unused_imports)]
use super::*;
/// AlphaNum12 is an XDR Struct defined as:
///
/// ```text
/// struct AlphaNum12
/// {
///     AssetCode12 assetCode;
///     AccountID issuer;
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
pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}

impl ReadXdr for AlphaNum12 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                asset_code: AssetCode12::read_xdr(r)?,
                issuer: AccountId::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AlphaNum12 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.asset_code.write_xdr(w)?;
            self.issuer.write_xdr(w)?;
            Ok(())
        })
    }
}
