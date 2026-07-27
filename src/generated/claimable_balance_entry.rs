#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClaimableBalanceEntry is an XDR Struct defined as:
///
/// ```text
/// struct ClaimableBalanceEntry
/// {
///     // Unique identifier for this ClaimableBalanceEntry
///     ClaimableBalanceID balanceID;
///
///     // List of claimants with associated predicate
///     Claimant claimants<10>;
///
///     // Any asset including native
///     Asset asset;
///
///     // Amount of asset
///     int64 amount;
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         ClaimableBalanceEntryExtensionV1 v1;
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
pub struct ClaimableBalanceEntry {
    pub balance_id: ClaimableBalanceId,
    pub claimants: VecM<Claimant, 10>,
    pub asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub ext: ClaimableBalanceEntryExt,
}

impl ReadXdr for ClaimableBalanceEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                balance_id: ClaimableBalanceId::read_xdr(r)?,
                claimants: VecM::<Claimant, 10>::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                ext: ClaimableBalanceEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ClaimableBalanceEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.balance_id.write_xdr(w)?;
            self.claimants.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ClaimableBalanceEntryRef is a borrowing equivalent of [`ClaimableBalanceEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimableBalanceEntryRef<'a> {
    pub balance_id: ClaimableBalanceId,
    pub claimants: VecMRef<'a, ClaimantRef<'a>, 10>,
    pub asset: Asset,
    pub amount: i64,
    pub ext: ClaimableBalanceEntryExt,
}

#[cfg(feature = "alloc")]
impl From<&ClaimableBalanceEntryRef<'_>> for ClaimableBalanceEntry {
    #[must_use]
    fn from(v: &ClaimableBalanceEntryRef<'_>) -> Self {
        Self {
            balance_id: v.balance_id.clone(),
            claimants: v.claimants.to_vecm_from(),
            asset: v.asset.clone(),
            amount: v.amount,
            ext: v.ext.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ClaimableBalanceEntryRef<'_>> for ClaimableBalanceEntry {
    #[must_use]
    fn from(v: ClaimableBalanceEntryRef<'_>) -> Self {
        Self::from(&v)
    }
}
