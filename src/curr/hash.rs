use super::Hash;
#[cfg(feature = "sha2")]
use super::{HashIdPreimage, Limits, WriteXdr};
#[cfg(feature = "sha2")]
use sha2::{Digest, Sha256};

#[cfg(feature = "sha2")]
pub trait HashBytes {
    fn hash_bytes(bytes: impl AsRef<[u8]>) -> Self;
}
#[cfg(feature = "sha2")]
impl HashBytes for Hash {
    fn hash_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Hash(Sha256::digest(bytes.as_ref()).into())
    }
}

#[cfg(feature = "sha2")]
impl TryFrom<HashIdPreimage> for Hash {
    type Error = super::Error;
    fn try_from(value: HashIdPreimage) -> Result<Self, Self::Error> {
        Ok(Hash::hash_bytes(&value.to_xdr(Limits::none())?))
    }
}

#[cfg(feature = "sha2")]
impl TryFrom<HashIdPreimage> for stellar_strkey::Contract {
    type Error = super::Error;
    fn try_from(value: HashIdPreimage) -> Result<Self, Self::Error> {
        let hash: Hash = match &value {
            HashIdPreimage::ContractId(_) => value.try_into()?,
            _ => return Err(super::Error::Invalid),
        };
        Ok(hash.into())
    }
}

pub trait FromHex {
    type Error;
    fn from_hex(s: &str) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}

impl FromHex for Hash {
    type Error = hex::FromHexError;

    fn from_hex(s: &str) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized,
    {
        Ok(Hash(padded_hex_from_str(s)?))
    }
}
/// # Errors
///
/// Might return an error
pub fn padded_hex_from_str(s: &str) -> Result<[u8; 32], hex::FromHexError> {
    let n = 32;
    if s.len() > n * 2 {
        return Err(hex::FromHexError::InvalidStringLength);
    }
    let mut decoded = [0u8; 32];
    let padded = format!("{s:0>width$}", width = n * 2);
    hex::decode_to_slice(padded, &mut decoded)?;
    Ok(decoded)
}
