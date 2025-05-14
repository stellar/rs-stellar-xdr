//# Custom default implementations of some types.

use super::TransactionEnvelope;

impl Default for TransactionEnvelope {
    fn default() -> Self {
        Self::Tx(Default::default())
    }
}
