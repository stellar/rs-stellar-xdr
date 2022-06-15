use stellar_xdr::{
    Error, Memo, MuxedAccount, Preconditions, SequenceNumber, Transaction, TransactionEnvelope,
    TransactionExt, TransactionV1Envelope, Uint256,
};

#[cfg(feature = "std")]
use stellar_xdr::WriteXdr;

#[cfg(feature = "std")]
#[test]
fn test_vecm_deref() -> Result<(), Error> {
    use stellar_xdr::VecM;
    let vecm: VecM<u32, 10> = [1, 2, 3].try_into()?;
    let vec: &Vec<u32> = vecm.deref();
    let contains = vec.contains(&2);
    assert!(contains);
    Ok(())
}

#[cfg(feature = "std")]
#[test]
fn test_build_small_tx_with_std() -> Result<(), Error> {
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
