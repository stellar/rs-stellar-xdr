use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, MuxedAccount, Preconditions, TimeBounds,
    Transaction, TransactionEnvelope, TransactionExt, TransactionV0, TransactionV0Ext,
    TransactionV1Envelope, Uint256, VecM,
};

impl From<Transaction> for TransactionEnvelope {
    fn from(tx: Transaction) -> Self {
        TransactionEnvelope::Tx(TransactionV1Envelope {
            tx,
            signatures: VecM::default(),
        })
    }
}

impl From<FeeBumpTransaction> for TransactionEnvelope {
    fn from(tx: FeeBumpTransaction) -> Self {
        TransactionEnvelope::TxFeeBump(FeeBumpTransactionEnvelope {
            tx,
            signatures: VecM::default(),
        })
    }
}

impl From<Uint256> for MuxedAccount {
    fn from(ed25519: Uint256) -> Self {
        MuxedAccount::Ed25519(ed25519)
    }
}

impl From<TransactionV0Ext> for TransactionExt {
    fn from(ext: TransactionV0Ext) -> Self {
        match ext {
            TransactionV0Ext::V0 => TransactionExt::V0,
        }
    }
}

impl From<Option<TimeBounds>> for Preconditions {
    fn from(maybe_timebounds: Option<TimeBounds>) -> Self {
        match maybe_timebounds {
            None => Preconditions::None,
            Some(timebounds) => Preconditions::Time(timebounds),
        }
    }
}

impl From<TransactionV0> for Transaction {
    fn from(tx: TransactionV0) -> Self {
        Transaction {
            source_account: tx.source_account_ed25519.into(),
            cond: tx.time_bounds.into(),
            ext: tx.ext.into(),
            fee: tx.fee,
            memo: tx.memo,
            operations: tx.operations,
            seq_num: tx.seq_num,
        }
    }
}

impl From<&FeeBumpTransaction> for TransactionEnvelope {
    fn from(tx: &FeeBumpTransaction) -> Self {
        tx.clone().into()
    }
}

impl From<&Transaction> for TransactionEnvelope {
    fn from(tx: &Transaction) -> Self {
        tx.clone().into()
    }
}

impl From<&Uint256> for MuxedAccount {
    fn from(ed25519: &Uint256) -> Self {
        ed25519.clone().into()
    }
}

impl From<&TransactionV0Ext> for TransactionExt {
    fn from(ext: &TransactionV0Ext) -> Self {
        ext.clone().into()
    }
}

impl From<&Option<TimeBounds>> for Preconditions {
    fn from(maybe_timebounds: &Option<TimeBounds>) -> Self {
        maybe_timebounds.clone().into()
    }
}

impl From<&TransactionV0> for Transaction {
    fn from(tx: &TransactionV0) -> Self {
        tx.clone().into()
    }
}
