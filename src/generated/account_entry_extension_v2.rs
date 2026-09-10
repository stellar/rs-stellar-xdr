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

/// AccountEntryExtensionV2Const is a borrowing equivalent of [`AccountEntryExtensionV2`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV2Const {
    pub num_sponsored: u32,
    pub num_sponsoring: u32,
    pub signer_sponsoring_i_ds: VecMConst<SponsorshipDescriptor, MAX_SIGNERS>,
    pub ext: AccountEntryExtensionV2Ext,
}

#[cfg(feature = "const")]
impl AccountEntryExtensionV2Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_account_entry_extension_v2(self);
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
        w.write_type_account_entry_extension_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`AccountEntryExtensionV2`], mirroring `<AccountEntryExtensionV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_account_entry_extension_v2(
        &mut self,
        v: &AccountEntryExtensionV2Const,
    ) {
        self.write_u32(v.num_sponsored);
        self.write_u32(v.num_sponsoring);
        self.write_type_vec_sponsorship_descriptor(&v.signer_sponsoring_i_ds);
        self.write_type_account_entry_extension_v2_ext(&v.ext);
    }
}
