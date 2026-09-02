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
impl IntoOwned for CreateClaimableBalanceOpRef<'_> {
    type Owned = CreateClaimableBalanceOp;
    fn into_owned(self) -> CreateClaimableBalanceOp {
        CreateClaimableBalanceOp {
            asset: self.asset.into_owned(),
            amount: self.amount.into_owned(),
            claimants: self.claimants.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&CreateClaimableBalanceOpRef<'_>> for CreateClaimableBalanceOp {
    #[must_use]
    fn from(v: &CreateClaimableBalanceOpRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<CreateClaimableBalanceOpRef<'_>> for CreateClaimableBalanceOp {
    #[must_use]
    fn from(v: CreateClaimableBalanceOpRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for CreateClaimableBalanceOpRef<'_> {
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

#[cfg(feature = "const")]
impl CreateClaimableBalanceOpView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_create_claimable_balance_op(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_create_claimable_balance_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`CreateClaimableBalanceOp`], mirroring `<CreateClaimableBalanceOp as WriteXdr>::write_xdr`.
    pub const fn write_type_create_claimable_balance_op(
        &mut self,
        v: &CreateClaimableBalanceOpView<'_>,
    ) {
        self.write_type_asset(&v.asset);
        self.write_i64(v.amount);
        self.write_type_vec_claimant(&v.claimants);
    }
}
