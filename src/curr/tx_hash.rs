use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, Hash, Limits, Transaction,
    TransactionEnvelope, TransactionV0, TransactionV0Envelope, TransactionV1Envelope, WriteXdr,
};

#[cfg(feature = "std")]
use sha2::{Digest, Sha256};

#[cfg(feature = "std")]
impl TransactionEnvelope {
    /// Computes the hash of the transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        match self {
            TransactionEnvelope::TxV0(e) => e.hash(network_id),
            TransactionEnvelope::Tx(e) => e.hash(network_id),
            TransactionEnvelope::TxFeeBump(e) => e.hash(network_id),
        }
    }
}

#[cfg(feature = "std")]
impl TransactionV0Envelope {
    /// Computes the hash of the V0 transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        self.tx.hash(network_id)
    }
}

#[cfg(feature = "std")]
impl TransactionV1Envelope {
    /// Computes the hash of the V1 transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        self.tx.hash(network_id)
    }
}

#[cfg(feature = "std")]
impl FeeBumpTransactionEnvelope {
    /// Computes the hash of the fee bump transaction envelope.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        self.tx.hash(network_id)
    }
}

#[cfg(feature = "std")]
impl Transaction {
    /// Computes the hash of the transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        let mut hasher = Sha256::new();
        
        // Add network ID
        hasher.update(network_id);
        
        // Add the ENVELOPE_TYPE_TX (2)
        hasher.update([0, 0, 0, 2]);
        
        // Add transaction data
        let tx_bytes = self.to_xdr(Limits::none())?;
        hasher.update(tx_bytes);
        
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        
        Ok(Hash(hash_bytes))
    }
}

#[cfg(feature = "std")]
impl TransactionV0 {
    /// Computes the hash of the V0 transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        let mut hasher = Sha256::new();
        
        // Add network ID
        hasher.update(network_id);
        
        // Add the ENVELOPE_TYPE_TX_V0 (0)
        hasher.update([0, 0, 0, 0]);
        
        // Add transaction data
        let tx_bytes = self.to_xdr(Limits::none())?;
        hasher.update(tx_bytes);
        
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        
        Ok(Hash(hash_bytes))
    }
}

#[cfg(feature = "std")]
impl FeeBumpTransaction {
    /// Computes the hash of the fee bump transaction.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to include in the hash computation.
    ///
    /// # Returns
    ///
    /// The transaction hash as `Hash`.
    pub fn hash(&self, network_id: &[u8]) -> Result<Hash, super::Error> {
        let mut hasher = Sha256::new();
        
        // Add network ID
        hasher.update(network_id);
        
        // Add the ENVELOPE_TYPE_TX_FEE_BUMP (5)
        hasher.update([0, 0, 0, 5]);
        
        // Add transaction data
        let tx_bytes = self.to_xdr(Limits::none())?;
        hasher.update(tx_bytes);
        
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        
        Ok(Hash(hash_bytes))
    }
}