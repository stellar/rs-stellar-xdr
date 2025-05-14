//# Custom default implementations of some types.

use super::{TransactionEnvelope, TransactionV1Envelope};

impl Default for TransactionEnvelope {
    fn default() -> Self {
        Self::Tx(TransactionV1Envelope::default())
    }
}
