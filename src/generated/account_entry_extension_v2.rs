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
    pub signer_sponsoring_i_ds: VecM<SponsorshipDescriptor, MAX_SIGNERS>,
    pub ext: AccountEntryExtensionV2Ext,
}

impl ReadXdr for AccountEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                num_sponsored: u32::read_xdr(r)?,
                num_sponsoring: u32::read_xdr(r)?,
                signer_sponsoring_i_ds: VecM::<SponsorshipDescriptor, MAX_SIGNERS>::read_xdr(r)?,
                ext: AccountEntryExtensionV2Ext::read_xdr(r)?,
            })
        })
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
}

/// AccountEntryExtensionV2View is a borrowing equivalent of [`AccountEntryExtensionV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV2View<'a> {
    pub num_sponsored: u32,
    pub num_sponsoring: u32,
    pub signer_sponsoring_i_ds: VecMView<'a, SponsorshipDescriptor, MAX_SIGNERS>,
    pub ext: AccountEntryExtensionV2Ext,
}

#[cfg(feature = "alloc")]
impl IntoOwned for AccountEntryExtensionV2View<'_> {
    type Owned = AccountEntryExtensionV2;
    fn into_owned(self) -> AccountEntryExtensionV2 {
        AccountEntryExtensionV2 {
            num_sponsored: self.num_sponsored.into_owned(),
            num_sponsoring: self.num_sponsoring.into_owned(),
            signer_sponsoring_i_ds: self.signer_sponsoring_i_ds.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&AccountEntryExtensionV2View<'_>> for AccountEntryExtensionV2 {
    #[must_use]
    fn from(v: &AccountEntryExtensionV2View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<AccountEntryExtensionV2View<'_>> for AccountEntryExtensionV2 {
    #[must_use]
    fn from(v: AccountEntryExtensionV2View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for AccountEntryExtensionV2View<'_> {
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
}
