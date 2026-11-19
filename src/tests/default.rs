#![cfg(feature = "alloc")]

use crate::{Hash, TransactionEnvelope};

#[test]
fn default() {
    let v = Hash::default();
    assert_eq!(v, Hash([0; 32]));

    let v = TransactionEnvelope::default();
    assert!(matches!(v, TransactionEnvelope::Tx(_)));
}
