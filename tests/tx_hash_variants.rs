#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

#[cfg(all(feature = "std", feature = "base64"))]
#[test]
fn test_transaction_envelope_hash_variants() -> Result<(), stellar_xdr::Error> {
    use stellar_xdr::{
        FeeBumpTransaction, FeeBumpTransactionEnvelope, FeeBumpTransactionInnerTx, Memo,
        MuxedAccount, Preconditions, SequenceNumber, Transaction, TransactionEnvelope,
        TransactionExt, TransactionV0, TransactionV0Envelope, TransactionV0Ext, TransactionV1Envelope, Uint256,
    };
    
    // Define a common network ID for all tests
    let network_id = "Test SDF Network ; September 2015".as_bytes();
    
    // Test V0 Transaction Envelope
    let v0_tx = TransactionV0 {
        source_account_ed25519: Uint256([0; 32]),
        fee: 100,
        seq_num: SequenceNumber(1),
        time_bounds: None,
        operations: [].try_into()?,
        memo: Memo::Text("v0".as_bytes().try_into()?),
        ext: TransactionV0Ext::V0,
    };
    
    let v0_env = TransactionEnvelope::TxV0(TransactionV0Envelope {
        tx: v0_tx,
        signatures: [].try_into()?,
    });
    
    let v0_hash = v0_env.hash(network_id)?;
    println!("V0 Transaction Hash: {:?}", v0_hash);
    
    // Test V1 Transaction Envelope
    let v1_tx = Transaction {
        source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
        fee: 100,
        seq_num: SequenceNumber(1),
        cond: Preconditions::None,
        operations: [].try_into()?,
        memo: Memo::Text("v1".as_bytes().try_into()?),
        ext: TransactionExt::V0,
    };
    
    let v1_env = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: v1_tx.clone(),
        signatures: [].try_into()?,
    });
    
    let v1_hash = v1_env.hash(network_id)?;
    println!("V1 Transaction Hash: {:?}", v1_hash);
    
    // Test FeeBump Transaction Envelope
    let inner_tx_env = TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 100,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            operations: [].try_into()?,
            memo: Memo::Text("inner".as_bytes().try_into()?),
            ext: TransactionExt::V0,
        },
        signatures: [].try_into()?,
    };
    
    let fee_bump_tx = FeeBumpTransaction {
        fee_source: MuxedAccount::Ed25519(Uint256([0; 32])),
        fee: 200,
        inner_tx: FeeBumpTransactionInnerTx::Tx(inner_tx_env),
        ext: stellar_xdr::FeeBumpTransactionExt::V0,
    };
    
    let fee_bump_env = TransactionEnvelope::TxFeeBump(FeeBumpTransactionEnvelope {
        tx: fee_bump_tx,
        signatures: [].try_into()?,
    });
    
    let fee_bump_hash = fee_bump_env.hash(network_id)?;
    println!("FeeBump Transaction Hash: {:?}", fee_bump_hash);
    
    // Verify that different transaction types produce different hashes
    assert_ne!(v0_hash, v1_hash);
    assert_ne!(v0_hash, fee_bump_hash);
    assert_ne!(v1_hash, fee_bump_hash);
    
    // Verify that same input produces the same hash (idempotence)
    assert_eq!(v0_env.hash(network_id)?, v0_hash);
    assert_eq!(v1_env.hash(network_id)?, v1_hash);
    assert_eq!(fee_bump_env.hash(network_id)?, fee_bump_hash);
    
    // Verify that different network IDs produce different hashes
    let different_network_id = "Public Global Stellar Network ; September 2015".as_bytes();
    assert_ne!(v0_env.hash(network_id)?, v0_env.hash(different_network_id)?);
    assert_ne!(v1_env.hash(network_id)?, v1_env.hash(different_network_id)?);
    assert_ne!(fee_bump_env.hash(network_id)?, fee_bump_env.hash(different_network_id)?);
    
    Ok(())
}