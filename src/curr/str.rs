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
#![cfg(feature = "alloc")]

use super::{
    AccountId, AssetCode, AssetCode12, AssetCode4, Error, Hash, MuxedAccount, MuxedAccountMed25519,
    NodeId, PublicKey, ScAddress, SignerKey, SignerKeyEd25519SignedPayload, Uint256,
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
        let b = s.as_bytes();
        let mut code = AssetCode4([0u8; 4]);
        if b.len() <= code.0.len() {
            code.0[..b.len()].copy_from_slice(b);
            Ok(code)
        } else {
            Err(Error::Invalid)
        }
    }
}

impl core::fmt::Display for AssetCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(last_idx) = self.0.iter().rposition(|c| *c != 0) {
            write_utf8_lossy_with_nuls(f, &self.0[..=last_idx])
        } else {
            Ok(())
        }
    }
}

impl core::str::FromStr for AssetCode12 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let b = s.as_bytes();
        let mut code = AssetCode12([0u8; 12]);
        if b.len() <= code.0.len() {
            code.0[..b.len()].copy_from_slice(b);
            Ok(code)
        } else {
            Err(Error::Invalid)
        }
    }
}

impl core::fmt::Display for AssetCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(last_idx) = self.0.iter().rposition(|c| *c != 0) {
            write_utf8_lossy_with_nuls(f, &self.0[..=last_idx])
        } else {
            Ok(())
        }
    }
}

impl core::str::FromStr for AssetCode {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() <= 4 {
            Ok(AssetCode::CreditAlphanum4(AssetCode4::from_str(s)?))
        } else if b.len() <= 12 {
            Ok(AssetCode::CreditAlphanum12(AssetCode12::from_str(s)?))
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

/// Writes a byte slice as a utf8 string, replacing any bytes in invalid utf8
/// sequences as the nul byte.
///
/// A modified copy of the Rust stdlib docs examples here:
/// <https://doc.rust-lang.org/stable/core/str/struct.Utf8Error.html#examples>
///
/// This particular implementation preserves the length of the string written
/// such that exactly one byte is written for every byte in an invalid sequence,
/// by writing a nul (0x00) byte for each.
///
/// Normally it would be common to write a Unicode Replacement Character
/// (U+FFFD) for lossy coding but doing so would not preserve the length as a
/// single invalid byte would be replaced by two bytes.
pub fn write_utf8_lossy_with_nuls(
    f: &mut impl core::fmt::Write,
    mut input: &[u8],
) -> core::fmt::Result {
    loop {
        match core::str::from_utf8(input) {
            Ok(valid) => {
                write!(f, "{valid}")?;
                break;
            }
            Err(error) => {
                let (valid, after_valid) = input.split_at(error.valid_up_to());
                write!(f, "{}", core::str::from_utf8(valid).unwrap())?;

                if let Some(invalid_sequence_length) = error.error_len() {
                    for _ in 0..invalid_sequence_length {
                        write!(f, "\0")?;
                    }
                    input = &after_valid[invalid_sequence_length..];
                } else {
                    for _ in 0..after_valid.len() {
                        write!(f, "\0")?;
                    }
                    break;
                }
            }
        }
    }
    Ok(())
}
