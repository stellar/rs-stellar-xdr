#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountEntryExtensionV2 is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntryExtensionV2
/// {
///     uint32 numSponsored;
///     uint32 numSponsoring;
///     SponsorshipDescriptor signerSponsoringIDs<MAX_SIGNERS>;
///
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 3:
///         AccountEntryExtensionV3 v3;
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
pub struct AccountEntryExtensionV2 {
    pub num_sponsored: u32,
    pub num_sponsoring: u32,
    pub signer_sponsoring_i_ds: VecM<SponsorshipDescriptor, 20>,
    pub ext: AccountEntryExtensionV2Ext,
}

impl ReadXdr for AccountEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                num_sponsored: u32::read_xdr(r)?,
                num_sponsoring: u32::read_xdr(r)?,
                signer_sponsoring_i_ds: VecM::<SponsorshipDescriptor, 20>::read_xdr(r)?,
                ext: AccountEntryExtensionV2Ext::read_xdr(r)?,
            })
        })
    }
}

impl AccountEntryExtensionV2 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.num_sponsored);
        w.write_u32(self.num_sponsoring);
        {
            w.enter_depth();
            let __s0 = self.signer_sponsoring_i_ds.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.ext.const_write_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for AccountEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.num_sponsored.write_xdr(w)?;
            self.num_sponsoring.write_xdr(w)?;
            self.signer_sponsoring_i_ds.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
