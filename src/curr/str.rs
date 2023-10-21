//# Custom string representations of the following types, also used for JSON
//# formatting.
//#
//# ## Strkey Types
//# - PublicKey
//# - AccountId
//# - MuxedAccount
//# - MuxedAccountMed25519
//# - SignerKey
//# - SignerKeyEd25519SignedPayload
//# - NodeId
//#
//# ## Asset Types
//# - Asset
//# - AlphaNum4
//# - AlphaNum12
//#
//# ## ASCII Types
//# - AssetCode
//# - AssetCode4
//# - AssetCode12
#![cfg(feature = "std")]
use super::{AccountId, Error, PublicKey, Uint256};

impl From<stellar_strkey::DecodeError> for Error {
    fn from(_: stellar_strkey::DecodeError) -> Self {
        // TODO: Add error type for strkeys.
        Error::Invalid
    }
}

impl core::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PublicKey::PublicKeyTypeEd25519(k) => {
                let k = stellar_strkey::ed25519::PublicKey::from_payload(&k.0)
                    .map_err(|_| std::fmt::Error)?;
                let s = k.to_string();
                f.write_str(&s)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for PublicKey {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::PublicKey(k) =
            stellar_strkey::ed25519::PublicKey::from_str(s)?;
        Ok(PublicKey::PublicKeyTypeEd25519(Uint256(k)))
    }
}

impl core::fmt::Display for AccountId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for AccountId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(AccountId(PublicKey::from_str(s)?))
    }
}
