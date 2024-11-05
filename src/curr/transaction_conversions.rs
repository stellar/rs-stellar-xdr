use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, Memo, MuxedAccount, Operation, Preconditions,
    SequenceNumber, Transaction, TransactionEnvelope, TransactionExt, TransactionV1Envelope, VecM,
};
#[cfg(feature = "sha2")]
use super::{
    Hash, Limits, TransactionSignaturePayload, TransactionSignaturePayloadTaggedTransaction,
    WriteXdr,
};

use super::Error;

impl Transaction {
    pub fn new_tx(
        source_account: impl Into<MuxedAccount>,
        fee: u32,
        seq_num: impl Into<SequenceNumber>,
        operation: Operation,
    ) -> Transaction {
        Transaction {
            source_account: source_account.into(),
            fee,
            seq_num: seq_num.into(),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: [operation].try_into().unwrap(),
            ext: TransactionExt::V0,
        }
    }

    pub fn add_operation(mut self, operation: Operation) -> Result<Self, Error> {
        let mut ops = self.operations.to_vec();
        ops.push(operation);
        self.operations = ops.try_into().map_err(|_| Error::Invalid)?;
        Ok(self)
    }

    #[cfg(feature = "sha2")]
    pub fn hash(&self, network_passphrase: &str) -> Result<Hash, Error> {
        let signature_payload = TransactionSignaturePayload {
            network_id: Hash::hash_bytes(network_passphrase),
            tagged_transaction: TransactionSignaturePayloadTaggedTransaction::Tx(self.clone()),
        };
        signature_payload
            .to_xdr(Limits::none())
            .map(Hash::hash_bytes)
    }
}

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
