#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// RevokeSponsorshipOpSigner is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID accountID;
///         SignerKey signerKey;
///     }
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
pub struct RevokeSponsorshipOpSigner {
    pub account_id: AccountId,
    pub signer_key: SignerKey,
}

impl ReadXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                signer_key: SignerKey::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.signer_key.write_xdr(w)?;
            Ok(())
        })
    }
}

/// RevokeSponsorshipOpSignerRef is a borrowing equivalent of [`RevokeSponsorshipOpSigner`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevokeSponsorshipOpSignerRef<'a> {
    pub account_id: AccountId,
    pub signer_key: SignerKeyRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for RevokeSponsorshipOpSignerRef<'_> {
    type Owned = RevokeSponsorshipOpSigner;
    fn into_owned(self) -> RevokeSponsorshipOpSigner {
        RevokeSponsorshipOpSigner {
            account_id: self.account_id.into_owned(),
            signer_key: self.signer_key.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&RevokeSponsorshipOpSignerRef<'_>> for RevokeSponsorshipOpSigner {
    #[must_use]
    fn from(v: &RevokeSponsorshipOpSignerRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<RevokeSponsorshipOpSignerRef<'_>> for RevokeSponsorshipOpSigner {
    #[must_use]
    fn from(v: RevokeSponsorshipOpSignerRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for RevokeSponsorshipOpSignerRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.signer_key.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl RevokeSponsorshipOpSignerView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_revoke_sponsorship_op_signer(self);
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
        w.write_type_revoke_sponsorship_op_signer(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`RevokeSponsorshipOpSigner`], mirroring `<RevokeSponsorshipOpSigner as WriteXdr>::write_xdr`.
    pub const fn write_type_revoke_sponsorship_op_signer(
        &mut self,
        v: &RevokeSponsorshipOpSignerView<'_>,
    ) {
        self.write_type_account_id(&v.account_id);
        self.write_type_signer_key(&v.signer_key);
    }
}
