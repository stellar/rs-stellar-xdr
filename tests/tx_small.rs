use stellar_xdr::{
    Error, Memo, MuxedAccount, Preconditions, SequenceNumber, Transaction, TransactionEnvelope,
    TransactionExt, TransactionV1Envelope, Uint256,
};

#[cfg(all(feature = "std", feature = "base64"))]
#[test]
fn test_build_small_tx_with_std() -> Result<(), Error> {
    use stellar_xdr::WriteXdr;

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
    let xdr = te.to_xdr_base64()?;
    assert_eq!(xdr, "AAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAABAAAAB1N0ZWxsYXIAAAAAAAAAAAAAAAAA");
    let xdr = te.to_xdr()?;
    assert_eq!(
        xdr,
        vec![
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 7, 83, 116, 101, 108, 108, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0
        ]
    );
    Ok(())
}

#[cfg(feature = "alloc")]
#[test]
fn test_build_small_tx_with_alloc() -> Result<(), Error> {
    let _tx = TransactionEnvelope::Tx(TransactionV1Envelope {
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
    Ok(())
}

#[cfg(not(feature = "alloc"))]
#[test]
fn test_build_small_tx_with_alloc() -> Result<(), Error> {
    let _ = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 0,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::Text("Stellar".as_bytes().try_into()?),
            operations: (&[]).try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: (&[]).try_into()?,
    });
    Ok(())
}
