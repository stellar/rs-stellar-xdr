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

impl RevokeSponsorshipOpSigner {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.account_id.const_write_xdr(w);
        self.signer_key.const_write_xdr(w);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
