#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    Error, Memo, MuxedAccount, Preconditions, SequenceNumber, Transaction, TransactionEnvelope,
    TransactionExt, TransactionV1Envelope, Uint256,
};

#[cfg(all(feature = "std", feature = "base64"))]
#[test]
fn test_transaction_hash() -> Result<(), Error> {
    // Create a simple transaction envelope
    let te = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 0,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::Text("Stellar".as_bytes().try_into()?),
            operations: [].to_vec().try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: [].try_into()?,
    });

    // Create a network ID (using the "Test SDF Network ; September 2015" as an example)
    let network_id = "Test SDF Network ; September 2015".as_bytes();
    
    // Compute the hash
    let hash = te.hash(network_id)?;
    
    // Print the hash for debugging
    println!("Transaction hash: {:?}", hash);
    
    // Verify the hash has 32 bytes
    assert_eq!(hash.0.len(), 32);
    
    // The same transaction with the same network ID should produce the same hash
    let hash2 = te.hash(network_id)?;
    assert_eq!(hash, hash2);
    
    // Different network ID should produce a different hash
    let public_network_id = "Public Global Stellar Network ; September 2015".as_bytes();
    let hash3 = te.hash(public_network_id)?;
    assert_ne!(hash, hash3);
    
    Ok(())
}