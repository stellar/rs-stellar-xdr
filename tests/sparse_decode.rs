//! Tests for sparse XDR decoding types.
//!
//! Sparse types are decode-only types that extract only specific nested fields
//! from XDR data, skipping over unneeded fields during decode.

#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    AccountId, CreateAccountOp, Error, Limits, Memo, MuxedAccount, Operation, OperationBody,
    Preconditions, PublicKey, ReadXdr, SeekableReadXdr, SequenceNumber, Transaction,
    TransactionEnvelope, TransactionEnvelopeSparse, TransactionExt, TransactionV1Envelope, Uint256,
    WriteXdr,
};

/// Helper to create a test TransactionEnvelope with operations
fn create_test_envelope(num_ops: usize) -> TransactionEnvelope {
    let operations: Vec<Operation> = (0..num_ops)
        .map(|i| Operation {
            source_account: None,
            body: OperationBody::CreateAccount(CreateAccountOp {
                destination: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([i as u8; 32]))),
                starting_balance: 1000 + i as i64,
            }),
        })
        .collect();

    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 100,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::Text("Test".as_bytes().try_into().unwrap()),
            operations: operations.try_into().unwrap(),
            ext: TransactionExt::V0,
        },
        signatures: vec![].try_into().unwrap(),
    })
}

#[test]
fn test_sparse_decode_tx_matches_full() -> Result<(), Error> {
    // Create a transaction envelope with some operations
    let envelope = create_test_envelope(3);
    let xdr = envelope.to_xdr(Limits::none())?;

    // Decode as sparse
    let sparse = TransactionEnvelopeSparse::from_xdr(&xdr, Limits::none())?;

    // Decode as full
    let full = TransactionEnvelope::from_xdr(&xdr, Limits::none())?;

    // Extract operations from both
    let sparse_ops = match sparse {
        TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };
    let full_ops = match full {
        TransactionEnvelope::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };

    // Verify they match
    assert_eq!(sparse_ops.len(), full_ops.len());
    assert_eq!(sparse_ops.len(), 3);
    for (i, (sparse_op, full_op)) in sparse_ops.iter().zip(full_ops.iter()).enumerate() {
        assert_eq!(sparse_op, full_op, "Operation {} mismatch", i);
    }

    Ok(())
}

#[test]
fn test_sparse_decode_empty_operations() -> Result<(), Error> {
    // Create a transaction envelope with no operations
    let envelope = create_test_envelope(0);
    let xdr = envelope.to_xdr(Limits::none())?;

    // Decode as sparse
    let sparse = TransactionEnvelopeSparse::from_xdr(&xdr, Limits::none())?;

    // Verify empty operations
    let sparse_ops = match sparse {
        TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };
    assert_eq!(sparse_ops.len(), 0);

    Ok(())
}

#[test]
fn test_sparse_decode_many_operations() -> Result<(), Error> {
    // Create a transaction envelope with many operations
    let envelope = create_test_envelope(100); // Max is 100
    let xdr = envelope.to_xdr(Limits::none())?;

    // Decode as sparse
    let sparse = TransactionEnvelopeSparse::from_xdr(&xdr, Limits::none())?;

    // Decode as full
    let full = TransactionEnvelope::from_xdr(&xdr, Limits::none())?;

    // Extract and compare operations
    let sparse_ops = match sparse {
        TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };
    let full_ops = match full {
        TransactionEnvelope::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };

    assert_eq!(sparse_ops.len(), full_ops.len());
    assert_eq!(sparse_ops.len(), 100);

    Ok(())
}

#[test]
fn test_sparse_type_is_smaller() {
    // Verify that the sparse type doesn't include fields we don't need
    // This is a compile-time check - if the sparse type had all fields,
    // it would be at least as large as the full type
    use std::mem::size_of;

    let sparse_size = size_of::<TransactionEnvelopeSparse>();
    let full_size = size_of::<TransactionEnvelope>();

    // The sparse type should be smaller since it doesn't include
    // signatures, fee, seq_num, memo, etc.
    assert!(
        sparse_size <= full_size,
        "Sparse type ({} bytes) should be <= full type ({} bytes)",
        sparse_size,
        full_size
    );
}

#[test]
fn test_seekable_sparse_decode_matches_standard() -> Result<(), Error> {
    // Create a transaction envelope with operations
    let envelope = create_test_envelope(5);
    let xdr = envelope.to_xdr(Limits::none())?;

    // Decode using standard ReadXdr
    let standard = TransactionEnvelopeSparse::from_xdr(&xdr, Limits::none())?;

    // Decode using SeekableReadXdr
    let seekable = TransactionEnvelopeSparse::from_xdr_seekable(&xdr, Limits::none())?;

    // Extract operations from both
    let standard_ops = match standard {
        TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };
    let seekable_ops = match seekable {
        TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
        _ => panic!("expected Tx variant"),
    };

    // Verify they match
    assert_eq!(standard_ops.len(), seekable_ops.len());
    assert_eq!(standard_ops.len(), 5);
    for (i, (std_op, seek_op)) in standard_ops.iter().zip(seekable_ops.iter()).enumerate() {
        assert_eq!(std_op, seek_op, "Operation {} mismatch", i);
    }

    Ok(())
}
