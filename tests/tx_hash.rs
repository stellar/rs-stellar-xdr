#![cfg(all(
    feature = "std",
    feature = "base64",
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

use bytes_lit::bytes;

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, FeeBumpTransactionInnerTx, Limits, Memo,
    MuxedAccount, Preconditions, SequenceNumber, Transaction, TransactionEnvelope, TransactionExt,
    TransactionV0, TransactionV0Envelope, TransactionV0Ext, TransactionV1Envelope, Uint256,
    WriteXdr as _,
};

const NETWORK_ID: [u8; 32] =
    // SHA256("Test SDF Network ; September 2015")
    bytes!(0xcee0302d59844d32bdca915c8203dd44b33fbb7edc19051ea37abedf28ecd472);

#[test]
fn test_transaction_v0_hash() -> Result<(), stellar_xdr::Error> {
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

    let v0_xdr = v0_env.to_xdr_base64(Limits::none())?;
    println!("TransactionV0: {v0_xdr}");
    let v0_hash = hex::encode(v0_env.hash(NETWORK_ID)?);
    println!("TransactionV0 Hash: {v0_hash}");
    assert_eq!(
        v0_hash,
        "037d99f3d0ded34afbf25ae9bf33d84d0a7b6cb2f37a8751301490660c4fc9c7"
    );

    Ok(())
}

#[test]
fn test_transaction_v1_hash() -> Result<(), stellar_xdr::Error> {
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

    let v1_xdr = v1_env.to_xdr_base64(Limits::none())?;
    println!("Transaction (V1): {v1_xdr}");
    let v1_hash = hex::encode(v1_env.hash(NETWORK_ID)?);
    println!("Transaction (V1) Hash: {v1_hash}");
    assert_eq!(
        v1_hash,
        "cbe35083a4f43334c6314ef6f8c950200c91a336d7373efd34cfb71d2d009a63"
    );

    Ok(())
}

#[test]
fn test_fee_bump_transaction_hash() -> Result<(), stellar_xdr::Error> {
    let fee_bump_tx = FeeBumpTransaction {
        fee_source: MuxedAccount::Ed25519(Uint256([0; 32])),
        fee: 200,
        inner_tx: FeeBumpTransactionInnerTx::Tx(TransactionV1Envelope {
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
        }),
        ext: stellar_xdr::FeeBumpTransactionExt::V0,
    };

    let inner_tx_xdr = fee_bump_tx.inner_tx.to_xdr_base64(Limits::none())?;
    println!("Fee Bump Inner Transaction: {inner_tx_xdr}");
    let inner_tx_hash = hex::encode(fee_bump_tx.inner_tx.hash(NETWORK_ID)?);
    println!("Fee Bump Inner Transaction Hash: {inner_tx_hash}");
    assert_eq!(
        inner_tx_hash,
        "9f8bd4d2b3bb1ad9b4aac192fd4ee430a12862115d4464b904b4836ab2a60a35"
    );

    let fee_bump_env = TransactionEnvelope::TxFeeBump(FeeBumpTransactionEnvelope {
        tx: fee_bump_tx,
        signatures: [].try_into()?,
    });

    let fee_bump_xdr = fee_bump_env.to_xdr_base64(Limits::none())?;
    println!("Fee Bump Transaction: {fee_bump_xdr}");
    let fee_bump_hash = hex::encode(fee_bump_env.hash(NETWORK_ID)?);
    println!("Fee Bump Transaction Hash: {fee_bump_hash}");
    assert_eq!(
        fee_bump_hash,
        "93136bdb126dfda1154143c5e16894fc9ccad41c1a54ed1f8dc8ed0964d53330"
    );

    Ok(())
}
