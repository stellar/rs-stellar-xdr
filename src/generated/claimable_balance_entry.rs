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

impl ClaimableBalanceEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.balance_id.const_to_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.claimants.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.asset.const_to_xdr(w);
        w.write_i64(self.amount);
        self.ext.const_to_xdr(w);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
