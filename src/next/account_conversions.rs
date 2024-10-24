use super::{AccountId, MuxedAccount, PublicKey};

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        account_id.0.into()
    }
}

impl From<stellar_strkey::ed25519::PublicKey> for PublicKey {
    fn from(k: stellar_strkey::ed25519::PublicKey) -> Self {
        PublicKey::PublicKeyTypeEd25519(k.0.into())
    }
}

impl From<PublicKey> for stellar_strkey::ed25519::PublicKey {
    fn from(k: PublicKey) -> Self {
        match k {
            PublicKey::PublicKeyTypeEd25519(k) => stellar_strkey::ed25519::PublicKey(k.into()),
        }
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

impl From<MuxedAccount> for AccountId {
    fn from(muxed_account: MuxedAccount) -> Self {
        muxed_account.account_id()
    }
}
