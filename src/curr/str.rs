//# Custom string representations of the following types, also used for JSON
//# formatting.
//#
//# The types that has impls in this file are given to the xdrgen
//# --rust-types-custom-str-impl cli option, so that xdrgen does not generate
//# FromStr and Display impls for them.
//#
//# ## Strkey Types (SEP-23)
//# - PublicKey
//# - AccountId
//# - MuxedAccount
//# - MuxedAccountMed25519
//# - SignerKey
//# - SignerKeyEd25519SignedPayload
//# - NodeId
//#
//# ## Asset Codes
//# - AssetCode
//# - AssetCode4
//# - AssetCode12
//#
//# ## Other
//# - ClaimableBalanceId
#![cfg(feature = "alloc")]

use super::{
    AccountId, AssetCode, AssetCode12, AssetCode4, ClaimableBalanceId, Error, Hash, MuxedAccount,
    MuxedAccountMed25519, NodeId, PublicKey, ScAddress, SignerKey, SignerKeyEd25519SignedPayload,
    Uint256,
};

impl From<stellar_strkey::DecodeError> for Error {
    fn from(_: stellar_strkey::DecodeError) -> Self {
        Error::Invalid
    }
}

impl core::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PublicKey::PublicKeyTypeEd25519(Uint256(k)) => {
                let k = stellar_strkey::ed25519::PublicKey::from_payload(k)
                    .map_err(|_| core::fmt::Error)?;
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
                let k = stellar_strkey::ed25519::PublicKey(*k);
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
                let k = stellar_strkey::ed25519::PublicKey(*k);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::PreAuthTx(Uint256(h)) => {
                let k = stellar_strkey::PreAuthTx(*h);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::HashX(Uint256(h)) => {
                let k = stellar_strkey::HashX(*h);
                let s = k.to_string();
                f.write_str(&s)?;
            }
            SignerKey::Ed25519SignedPayload(p) => p.fmt(f)?,
        }
        Ok(())
    }
}

impl core::str::FromStr for ScAddress {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let strkey = stellar_strkey::Strkey::from_str(s)?;
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(k)) => Ok(
                ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(k)))),
            ),
            stellar_strkey::Strkey::Contract(stellar_strkey::Contract(h)) => {
                Ok(ScAddress::Contract(Hash(h)))
            }
            stellar_strkey::Strkey::MuxedAccountEd25519(_)
            | stellar_strkey::Strkey::PrivateKeyEd25519(_)
            | stellar_strkey::Strkey::PreAuthTx(_)
            | stellar_strkey::Strkey::HashX(_)
            | stellar_strkey::Strkey::SignedPayloadEd25519(_) => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for ScAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ScAddress::Account(a) => a.fmt(f)?,
            ScAddress::Contract(Hash(h)) => {
                let k = stellar_strkey::Contract(*h);
                let s = k.to_string();
                f.write_str(&s)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for AssetCode4 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = AssetCode4([0u8; 4]);
        escape_bytes::unescape_into(&mut code.0, s.as_bytes()).map_err(|_| Error::Invalid)?;
        Ok(code)
    }
}

impl core::fmt::Display for AssetCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(last_idx) = self.0.iter().rposition(|c| *c != 0) {
            for b in escape_bytes::Escape::new(&self.0[..=last_idx]) {
                write!(f, "{}", b as char)?;
            }
        }
        Ok(())
    }
}

impl core::str::FromStr for AssetCode12 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = AssetCode12([0u8; 12]);
        escape_bytes::unescape_into(&mut code.0, s.as_bytes()).map_err(|_| Error::Invalid)?;
        Ok(code)
    }
}

impl core::fmt::Display for AssetCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // AssetCode12's are always rendered as at least 5 characters, because
        // any asset code shorter than 5 characters is an AssetCode4.
        // AssetCode12 contains a fixed length 12-byte array, and the constant
        // and slices in this function never operate out-of-bounds because of
        // that.
        const MIN_LENGTH: usize = 5;
        let len = MIN_LENGTH
            + self
                .0
                .iter()
                .skip(MIN_LENGTH)
                .rposition(|c| *c != 0)
                .map_or(0, |last_idx| last_idx + 1);
        for b in escape_bytes::Escape::new(&self.0[..len]) {
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

impl core::str::FromStr for AssetCode {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut code = [0u8; 12];
        let n = escape_bytes::unescape_into(&mut code, s.as_bytes()).map_err(|_| Error::Invalid)?;
        if n <= 4 {
            Ok(AssetCode::CreditAlphanum4(AssetCode4([
                code[0], code[1], code[2], code[3],
            ])))
        } else if n <= 12 {
            Ok(AssetCode::CreditAlphanum12(AssetCode12(code)))
        } else {
            Err(Error::Invalid)
        }
    }
}

impl core::fmt::Display for AssetCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AssetCode::CreditAlphanum4(c) => c.fmt(f),
            AssetCode::CreditAlphanum12(c) => c.fmt(f),
        }
    }
}

impl core::str::FromStr for ClaimableBalanceId {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        // This conversion to a hex string could be done by XDR encoding the
        // self value, but because XDR encoding requires the std feature, this
        // approach is taken instead to preserve the fact that the serde feature
        // is available with alloc only.
        let bytes = hex::decode(s).map_err(|_| Error::InvalidHex)?;
        match bytes.as_slice() {
            [0, 0, 0, 0, ..] => Ok(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash(
                (&bytes[4..]).try_into()?,
            ))),
            _ => Err(Error::Invalid),
        }
    }
}

impl core::fmt::Display for ClaimableBalanceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // This conversion from a hex string could be done by XDR decoding the
        // self value, but because XDR decoding requires the std feature, this
        // approach is taken instead to preserve the fact that the serde feature
        // is available with alloc only.
        match self {
            ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash(bytes)) => {
                for b in [0u8, 0, 0, 0] {
                    write!(f, "{b:02x}")?;
                }
                for b in bytes {
                    write!(f, "{b:02x}")?;
                }
            }
        }
        Ok(())
    }
}
