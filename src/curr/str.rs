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

use super::{
    AccountId, Error, MuxedAccount, MuxedAccountMed25519, NodeId, PublicKey, SignerKey,
    SignerKeyEd25519SignedPayload, Uint256,
};

impl From<stellar_strkey::DecodeError> for Error {
    fn from(_: stellar_strkey::DecodeError) -> Self {
        // TODO: Add error type for strkeys.
        Error::Invalid
    }
}

impl core::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PublicKey::PublicKeyTypeEd25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey::from_payload(k)
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

impl core::fmt::Display for MuxedAccountMed25519 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let MuxedAccountMed25519 {
            ed25519: Uint256(ed25519),
            id,
        } = self;
        let k = stellar_strkey::ed25519::MuxedAccount {
            ed25519: *ed25519,
            id: *id,
        };
        let s = k.to_string();
        f.write_str(&s)?;
        Ok(())
    }
}

impl core::str::FromStr for MuxedAccountMed25519 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::MuxedAccount { ed25519, id } =
            stellar_strkey::ed25519::MuxedAccount::from_str(s)?;
        Ok(MuxedAccountMed25519 {
            ed25519: Uint256(ed25519),
            id,
        })
    }
}

impl core::str::FromStr for MuxedAccount {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => {
                Ok(MuxedAccount::Ed25519(Uint256(k)))
            }
            stellar_strkey::Strkey::MuxedAccountEd25519(
                stellar_strkey::ed25519::MuxedAccount { ed25519, id },
            ) => Ok(MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                ed25519: Uint256(ed25519),
                id,
            })),
            stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::PreAuthTx(_)
            | stellar_strkey::Strkey::HashX(_)
            | stellar_strkey::Strkey::SignedPayloadEd25519(_)
            | stellar_strkey::Strkey::Contract(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for MuxedAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MuxedAccount::Ed25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey(k.clone());
                let s = k.to_string();
                f.write_str(&s)?;
            }
            MuxedAccount::MuxedEd25519(m) => m.fmt(f)?,
        }
        Ok(())
    }
}

impl core::fmt::Display for NodeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for NodeId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(NodeId(PublicKey::from_str(s)?))
    }
}

impl core::fmt::Display for SignerKeyEd25519SignedPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let SignerKeyEd25519SignedPayload {
            ed25519: Uint256(ed25519),
            payload,
        } = self;
        let k = stellar_strkey::ed25519::SignedPayload {
            ed25519: *ed25519,
            payload: payload.into(),
        };
        let s = k.to_string();
        f.write_str(&s)?;
        Ok(())
    }
}

impl core::str::FromStr for SignerKeyEd25519SignedPayload {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let stellar_strkey::ed25519::SignedPayload { ed25519, payload } =
            stellar_strkey::ed25519::SignedPayload::from_str(s)?;
        Ok(SignerKeyEd25519SignedPayload {
            ed25519: Uint256(ed25519),
            payload: payload.try_into()?,
        })
    }
}

impl core::str::FromStr for SignerKey {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => {
                Ok(SignerKey::Ed25519(Uint256(k)))
            }
            stellar_strkey::Strkey::PreAuthTx(stellar_strkey::PreAuthTx(h)) => {
                Ok(SignerKey::PreAuthTx(Uint256(h)))
            }
            stellar_strkey::Strkey::HashX(stellar_strkey::HashX(h)) => {
                Ok(SignerKey::HashX(Uint256(h)))
            }
            stellar_strkey::Strkey::SignedPayloadEd25519(
                stellar_strkey::ed25519::SignedPayload { ed25519, payload },
            ) => Ok(SignerKey::Ed25519SignedPayload(
                SignerKeyEd25519SignedPayload {
                    ed25519: Uint256(ed25519),
                    payload: payload.try_into()?,
                },
            )),
            stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::Contract(_)
            | stellar_strkey::Strkey::MuxedAccountEd25519(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for SignerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SignerKey::Ed25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey(k.clone());
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::PreAuthTx(Uint256(h)) => {
                let k = stellar_strkey::PreAuthTx(h.clone());
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::HashX(Uint256(h)) => {
                let k = stellar_strkey::HashX(h.clone());
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::Ed25519SignedPayload(p) => p.fmt(f)?,
        }
        Ok(())
    }
}
