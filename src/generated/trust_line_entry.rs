#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TrustLineEntry is an XDR Struct defined as:
///
/// ```text
/// struct TrustLineEntry
/// {
///     AccountID accountID;  // account this trustline belongs to
///     TrustLineAsset asset; // type of asset (with issuer)
///     int64 balance;        // how much of this asset the user has.
///                           // Asset defines the unit for this;
///
///     int64 limit;  // balance cannot be above this
///     uint32 flags; // see TrustLineFlags
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         struct
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
///         } v1;
///     }
///     ext;
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
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub balance: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub limit: i64,
    pub flags: u32,
    pub ext: TrustLineEntryExt,
}

impl ReadXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                asset: TrustLineAsset::read_xdr(r)?,
                balance: i64::read_xdr(r)?,
                limit: i64::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                ext: TrustLineEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.limit.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
