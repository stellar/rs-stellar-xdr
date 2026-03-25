use super::{AccountId, MuxedAccount, PublicKey};

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        account_id.0.into()
    }
}

impl From<PublicKey> for MuxedAccount {
    fn from(public_key: PublicKey) -> Self {
        match public_key {
            PublicKey::PublicKeyTypeEd25519(k) => MuxedAccount::Ed25519(k),
        }
    }
}

impl MuxedAccount {
    #[must_use]
    pub fn account_id(self) -> AccountId {
        match self {
            MuxedAccount::Ed25519(k) => AccountId(PublicKey::PublicKeyTypeEd25519(k)),
            MuxedAccount::MuxedEd25519(m) => AccountId(PublicKey::PublicKeyTypeEd25519(m.ed25519)),
        }
    }
}
