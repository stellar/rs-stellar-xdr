use super::super::{Hash, HashIdPreimage, Limits, WriteXdr};

use sha2::{Digest, Sha256};

impl Hash {
    pub fn hash_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Hash(Sha256::digest(bytes.as_ref()).into())
    }
}

impl TryFrom<HashIdPreimage> for Hash {
    type Error = super::super::Error;
    fn try_from(value: HashIdPreimage) -> Result<Self, Self::Error> {
        Ok(Hash::hash_bytes(&value.to_xdr(Limits::none())?))
    }
}

impl TryFrom<HashIdPreimage> for stellar_strkey::Contract {
    type Error = super::super::Error;
    fn try_from(value: HashIdPreimage) -> Result<Self, Self::Error> {
        let hash: Hash = match &value {
            HashIdPreimage::ContractId(_) => value.try_into()?,
            _ => return Err(Self::Error::Invalid),
        };
        Ok(hash.into())
    }
}
