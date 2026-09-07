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

/// RevokeSponsorshipOpSignerView is a borrowing equivalent of [`RevokeSponsorshipOpSigner`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevokeSponsorshipOpSignerView<'a> {
    pub account_id: AccountId,
    pub signer_key: SignerKeyView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for RevokeSponsorshipOpSignerView<'_> {
    type Owned = RevokeSponsorshipOpSigner;
    fn into_owned(&self) -> RevokeSponsorshipOpSigner {
        RevokeSponsorshipOpSigner {
            account_id: self.account_id.into_owned(),
            signer_key: self.signer_key.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&RevokeSponsorshipOpSignerView<'_>> for RevokeSponsorshipOpSigner {
    #[must_use]
    fn from(v: &RevokeSponsorshipOpSignerView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<RevokeSponsorshipOpSignerView<'_>> for RevokeSponsorshipOpSigner {
    #[must_use]
    fn from(v: RevokeSponsorshipOpSignerView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for RevokeSponsorshipOpSignerView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.signer_key.write_xdr(w)?;
            Ok(())
        })
    }
}
