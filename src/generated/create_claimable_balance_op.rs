#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreateClaimableBalanceOp is an XDR Struct defined as:
///
/// ```text
/// struct CreateClaimableBalanceOp
/// {
///     Asset asset;
///     int64 amount;
///     Claimant claimants<10>;
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
pub struct CreateClaimableBalanceOp {
    pub asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub amount: i64,
    pub claimants: VecM<Claimant, 10>,
}

impl ReadXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                asset: Asset::read_xdr(r)?,
                amount: i64::read_xdr(r)?,
                claimants: VecM::<Claimant, 10>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.asset.write_xdr(w)?;
            self.amount.write_xdr(w)?;
            self.claimants.write_xdr(w)?;
            Ok(())
        })
    }
}

/// CreateClaimableBalanceOpRef is a borrowing equivalent of [`CreateClaimableBalanceOp`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateClaimableBalanceOpRef<'a> {
    pub asset: Asset,
    pub amount: i64,
    pub claimants: VecMRef<'a, ClaimantRef<'a>, 10>,
}

#[cfg(feature = "alloc")]
impl From<&CreateClaimableBalanceOpRef<'_>> for CreateClaimableBalanceOp {
    #[must_use]
    fn from(v: &CreateClaimableBalanceOpRef<'_>) -> Self {
        Self {
            asset: v.asset.clone(),
            amount: v.amount,
            claimants: v.claimants.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<CreateClaimableBalanceOpRef<'_>> for CreateClaimableBalanceOp {
    #[must_use]
    fn from(v: CreateClaimableBalanceOpRef<'_>) -> Self {
        Self::from(&v)
    }
}
