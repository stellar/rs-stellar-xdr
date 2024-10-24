use super::{Hash, ScAddress};

impl From<Hash> for stellar_strkey::Contract {
    fn from(v: Hash) -> Self {
        stellar_strkey::Contract(v.0)
    }
}

impl From<stellar_strkey::Contract> for Hash {
    fn from(v: stellar_strkey::Contract) -> Self {
        Hash(v.0)
    }
}

impl From<stellar_strkey::Contract> for ScAddress {
    fn from(v: stellar_strkey::Contract) -> Self {
        ScAddress::Contract(v.into())
    }
}

impl TryFrom<ScAddress> for stellar_strkey::Contract {
    type Error = super::Error;
    fn try_from(sc_address: ScAddress) -> Result<Self, Self::Error> {
        match sc_address {
            ScAddress::Contract(c) => Ok(c.into()),
            _ => Err(super::Error::Invalid),
        }
    }
}
