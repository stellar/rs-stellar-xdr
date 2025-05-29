#![cfg(feature = "std")]
use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, FeeBumpTransactionInnerTx, Hash, Limits,
    Transaction, TransactionEnvelope, TransactionSignaturePayload,
    TransactionSignaturePayloadTaggedTransaction, TransactionV0, TransactionV0Envelope,
    TransactionV1Envelope, WriteXdr,
};

use sha2::{Digest, Sha256};

impl TransactionEnvelope {
    /// Computes the hash of the transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        match self {
            TransactionEnvelope::TxV0(e) => e.hash(network_id),
            TransactionEnvelope::Tx(e) => e.hash(network_id),
            TransactionEnvelope::TxFeeBump(e) => e.hash(network_id),
        }
    }
}

impl TransactionV0Envelope {
    /// Computes the hash of the V0 transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        self.tx.hash(network_id)
    }
}

impl TransactionV1Envelope {
    /// Computes the hash of the V1 transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        self.tx.hash(network_id)
    }
}

impl TransactionV0 {
    /// Computes the hash of the V0 transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        <_ as Into<Transaction>>::into(self).hash(network_id)
    }
}

impl FeeBumpTransactionEnvelope {
    /// Computes the hash of the fee bump transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        self.tx.hash(network_id)
    }
}

impl FeeBumpTransactionInnerTx {
    /// Computes the hash of the fee bump transaction inner tx.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        match self {
            Self::Tx(inner_tx) => inner_tx.hash(network_id),
        }
    }
}

impl Transaction {
    /// Computes the hash of the transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        let payload = TransactionSignaturePayload {
            network_id: Hash(network_id),
            // TODO: Add support for borrowing the self here instead of cloning it.
            tagged_transaction: TransactionSignaturePayloadTaggedTransaction::Tx(self.clone()),
        };
        let payload = payload.to_xdr(Limits::none())?;
        let hash = Sha256::digest(payload);
        Ok(hash.into())
    }
}

impl FeeBumpTransaction {
    /// Computes the hash of the fee bump transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash.
    pub fn hash(&self, network_id: [u8; 32]) -> Result<[u8; 32], super::Error> {
        let payload = TransactionSignaturePayload {
            network_id: Hash(network_id),
            tagged_transaction: TransactionSignaturePayloadTaggedTransaction::TxFeeBump(
                self.clone(),
            ),
        };
        let payload = payload.to_xdr(Limits::none())?;
        let hash = Sha256::digest(payload);
        Ok(hash.into())
    }
}
