use super::{
    AccountId, Hash, MuxedAccount, MuxedAccountMed25519, PublicKey, ScAddress, ScVal, Uint256,
};

impl From<stellar_strkey::ed25519::PublicKey> for PublicKey {
    fn from(k: stellar_strkey::ed25519::PublicKey) -> Self {
        PublicKey::PublicKeyTypeEd25519(k.0.into())
    }
}
impl From<&stellar_strkey::ed25519::PublicKey> for PublicKey {
    fn from(k: &stellar_strkey::ed25519::PublicKey) -> Self {
        PublicKey::PublicKeyTypeEd25519(k.0.into())
    }
}

impl From<stellar_strkey::ed25519::PublicKey> for AccountId {
    fn from(k: stellar_strkey::ed25519::PublicKey) -> Self {
        AccountId(k.into())
    }
}
impl From<&stellar_strkey::ed25519::PublicKey> for AccountId {
    fn from(k: &stellar_strkey::ed25519::PublicKey) -> Self {
        AccountId(k.into())
    }
}

impl From<stellar_strkey::ed25519::PublicKey> for ScAddress {
    fn from(t: stellar_strkey::ed25519::PublicKey) -> Self {
        ScAddress::Account(t.into())
    }
}
impl From<&stellar_strkey::ed25519::PublicKey> for ScAddress {
    fn from(t: &stellar_strkey::ed25519::PublicKey) -> Self {
        ScAddress::Account(t.into())
    }
}

impl From<stellar_strkey::ed25519::PublicKey> for ScVal {
    fn from(k: stellar_strkey::ed25519::PublicKey) -> Self {
        ScVal::Address(k.into())
    }
}
impl From<&stellar_strkey::ed25519::PublicKey> for ScVal {
    fn from(k: &stellar_strkey::ed25519::PublicKey) -> Self {
        ScVal::Address(k.into())
    }
}

impl From<stellar_strkey::ed25519::MuxedAccount> for PublicKey {
    fn from(k: stellar_strkey::ed25519::MuxedAccount) -> Self {
        PublicKey::PublicKeyTypeEd25519(k.ed25519.into())
    }
}
impl From<&stellar_strkey::ed25519::MuxedAccount> for PublicKey {
    fn from(
        stellar_strkey::ed25519::MuxedAccount{ ed25519, .. }: &stellar_strkey::ed25519::MuxedAccount,
    ) -> Self {
        PublicKey::PublicKeyTypeEd25519(ed25519.into())
    }
}

impl From<stellar_strkey::ed25519::MuxedAccount> for AccountId {
    fn from(k: stellar_strkey::ed25519::MuxedAccount) -> Self {
        AccountId(k.into())
    }
}
impl From<&stellar_strkey::ed25519::MuxedAccount> for AccountId {
    fn from(k: &stellar_strkey::ed25519::MuxedAccount) -> Self {
        AccountId(k.into())
    }
}

impl From<stellar_strkey::ed25519::MuxedAccount> for ScAddress {
    fn from(t: stellar_strkey::ed25519::MuxedAccount) -> Self {
        ScAddress::Account(t.into())
    }
}
impl From<&stellar_strkey::ed25519::MuxedAccount> for ScAddress {
    fn from(t: &stellar_strkey::ed25519::MuxedAccount) -> Self {
        ScAddress::Account(t.into())
    }
}

impl From<stellar_strkey::ed25519::MuxedAccount> for ScVal {
    fn from(k: stellar_strkey::ed25519::MuxedAccount) -> Self {
        ScVal::Address(k.into())
    }
}
impl From<&stellar_strkey::ed25519::MuxedAccount> for ScVal {
    fn from(k: &stellar_strkey::ed25519::MuxedAccount) -> Self {
        ScVal::Address(k.into())
    }
}

impl TryFrom<&stellar_strkey::Strkey> for ScAddress {
    type Error = super::Error;
    fn try_from(strkey: &stellar_strkey::Strkey) -> Result<Self, Self::Error> {
        match strkey {
            stellar_strkey::Strkey::PublicKeyEd25519(k) => Ok(ScAddress::Account(k.into())),
            stellar_strkey::Strkey::MuxedAccountEd25519(m) => Ok(ScAddress::Account(m.into())),
            stellar_strkey::Strkey::Contract(k) => Ok(ScAddress::Contract(k.into())),
            _ => Err(super::Error::Invalid),
        }
    }
}

impl From<PublicKey> for stellar_strkey::ed25519::PublicKey {
    fn from(k: PublicKey) -> Self {
        (&k).into()
    }
}
impl From<&PublicKey> for stellar_strkey::ed25519::PublicKey {
    fn from(k: &PublicKey) -> Self {
        match k {
            PublicKey::PublicKeyTypeEd25519(k) => stellar_strkey::ed25519::PublicKey(k.into()),
        }
    }
}

impl From<PublicKey> for MuxedAccount {
    fn from(public_key: PublicKey) -> Self {
        (&public_key).into()
    }
}
impl From<&PublicKey> for MuxedAccount {
    fn from(public_key: &PublicKey) -> Self {
        match public_key {
            PublicKey::PublicKeyTypeEd25519(k) => MuxedAccount::Ed25519(k.into()),
        }
    }
}

impl From<&PublicKey> for PublicKey {
    fn from(public_key: &PublicKey) -> Self {
        match public_key {
            PublicKey::PublicKeyTypeEd25519(k) => PublicKey::PublicKeyTypeEd25519(k.into()),
        }
    }
}

impl From<PublicKey> for ScAddress {
    fn from(t: PublicKey) -> Self {
        ScAddress::Account(t.into())
    }
}
impl From<&PublicKey> for ScAddress {
    fn from(t: &PublicKey) -> Self {
        ScAddress::Account(t.into())
    }
}

// From<PublicKey> for AccountId already exists
impl From<&PublicKey> for AccountId {
    fn from(public_key: &PublicKey) -> Self {
        AccountId(public_key.into())
    }
}

impl From<AccountId> for ScAddress {
    fn from(account_id: AccountId) -> Self {
        ScAddress::Account(account_id)
    }
}
impl From<&AccountId> for ScAddress {
    fn from(AccountId(public_key): &AccountId) -> Self {
        ScAddress::Account(public_key.into())
    }
}

impl From<&PublicKey> for ScVal {
    fn from(public_key: &PublicKey) -> Self {
        ScVal::Address(public_key.into())
    }
}

impl From<PublicKey> for ScVal {
    fn from(public_key: PublicKey) -> Self {
        ScVal::Address(public_key.into())
    }
}

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        account_id.0.into()
    }
}
impl From<&AccountId> for MuxedAccount {
    fn from(AccountId(public_key): &AccountId) -> Self {
        public_key.into()
    }
}

impl From<AccountId> for stellar_strkey::ed25519::PublicKey {
    fn from(value: AccountId) -> Self {
        match value {
            AccountId(key) => key.into(),
        }
    }
}
impl From<&AccountId> for stellar_strkey::ed25519::PublicKey {
    fn from(value: &AccountId) -> Self {
        match value {
            AccountId(key) => key.into(),
        }
    }
}

impl From<&AccountId> for AccountId {
    fn from(AccountId(public_key): &AccountId) -> Self {
        AccountId(public_key.into())
    }
}

// MuxedAccount conversions
impl From<MuxedAccount> for AccountId {
    fn from(muxed_account: MuxedAccount) -> Self {
        (&muxed_account).into()
    }
}
impl From<&MuxedAccount> for AccountId {
    fn from(muxed_account: &MuxedAccount) -> Self {
        match muxed_account {
            MuxedAccount::Ed25519(k) => AccountId(PublicKey::PublicKeyTypeEd25519(k.into())),
            MuxedAccount::MuxedEd25519(MuxedAccountMed25519 { ed25519, .. }) => {
                AccountId(PublicKey::PublicKeyTypeEd25519(ed25519.into()))
            }
        }
    }
}

impl From<MuxedAccount> for ScAddress {
    fn from(t: MuxedAccount) -> Self {
        ScAddress::Account(t.into())
    }
}
impl From<&MuxedAccount> for ScAddress {
    fn from(t: &MuxedAccount) -> Self {
        ScAddress::Account(t.into())
    }
}

impl From<&MuxedAccount> for MuxedAccount {
    fn from(value: &MuxedAccount) -> Self {
        match value {
            MuxedAccount::Ed25519(k) => MuxedAccount::Ed25519(k.into()),
            MuxedAccount::MuxedEd25519(MuxedAccountMed25519 { ed25519, id }) => {
                MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                    ed25519: ed25519.into(),
                    id: *id,
                })
            }
        }
    }
}

impl From<ScAddress> for stellar_strkey::Strkey {
    fn from(sc_address: ScAddress) -> Self {
        match sc_address {
            ScAddress::Account(account_id) => {
                stellar_strkey::Strkey::PublicKeyEd25519(account_id.into())
            }
            ScAddress::Contract(contract) => stellar_strkey::Strkey::Contract(contract.into()),
        }
    }
}
impl From<&ScAddress> for stellar_strkey::Strkey {
    fn from(sc_address: &ScAddress) -> Self {
        match sc_address {
            ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(k)))) => {
                stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(*k))
            }
            ScAddress::Contract(Hash(h)) => {
                stellar_strkey::Strkey::Contract(stellar_strkey::Contract(*h))
            }
        }
    }
}

impl From<&ScAddress> for ScAddress {
    fn from(sc_address: &ScAddress) -> Self {
        match sc_address {
            ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(k)))) => {
                ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(*k))))
            }
            ScAddress::Contract(Hash(h)) => ScAddress::Contract(Hash(*h)),
        }
    }
}

impl From<ScAddress> for ScVal {
    fn from(sc_address: ScAddress) -> Self {
        ScVal::Address(sc_address)
    }
}

impl From<&ScAddress> for ScVal {
    fn from(sc_address: &ScAddress) -> Self {
        let sc_address: ScAddress = sc_address.into();
        sc_address.into()
    }
}

impl From<&Uint256> for Uint256 {
    fn from(Uint256(k): &Uint256) -> Self {
        Uint256(*k)
    }
}

impl From<&[u8; 32]> for Uint256 {
    fn from(k: &[u8; 32]) -> Self {
        Uint256(*k)
    }
}

impl From<&Uint256> for [u8; 32] {
    fn from(Uint256(k): &Uint256) -> Self {
        *k
    }
}
