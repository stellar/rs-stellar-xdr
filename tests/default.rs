#![cfg(feature = "alloc")]



use stellar_xdr::{Hash, TransactionEnvelope, Uint32};

#[test]
fn default() {
    let v = Uint32::default();
    assert_eq!(v, 0);

    let v = Hash::default();
    assert_eq!(v, Hash([0; 32]));

    let v = TransactionEnvelope::default();
    assert!(matches!(v, TransactionEnvelope::Tx(_)));
}
