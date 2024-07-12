use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, Transaction, TransactionEnvelope,
    TransactionV1Envelope, VecM,
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
