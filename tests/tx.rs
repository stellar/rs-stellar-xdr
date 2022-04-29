use stellar_xdr::*;

#[test]
fn test_build_small_tx() -> Result<(), Error> {
    let te = TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::KeyTypeEd25519(Uint256([0; 32])),
            fee: 0,
            seq_num: SequenceNumber(1),
            cond: Preconditions::PrecondNone,
            memo: Memo::MemoText("Stellar".as_bytes().try_into()?),
            operations: [].to_vec().try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: [].try_into()?,
    });
    let xdr = te.to_xdr_base64()?;
    assert_eq!(xdr, "AAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAABAAAAB1N0ZWxsYXIAAAAAAAAAAAAAAAAA");
    Ok(())
}

#[test]
fn test_parse_pubnet_v18_tx() -> Result<(), Error> {
    let xdr = "AAAAAgAAAAA/ESDPPSBIB8pWPGt/zZ3dSJhShRxziDdkmLQXrdytCQAPQkAACMblAAAABQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAAAtDSg//ZfvJXgv2/0yiA7QUDWdXpKYhdjYEWkN4yVm+AAAABdIdugAAAAAAAAAAAKt3K0JAAAAQC3/n83fG/BCSRaIQjuqL2i1koiCHChxt1aagXn2ABCRP9IL83u5zldxuUaDBklKOHEdy4cOvl2BhPNbjs7w0QSGVuCcAAAAQKxHSgHZgZY7AMlPumIt0iZvtkbsRAtt6BYahJdnxrqm3+JuCVv/1ijWi1kM85uLfo7NAITi1TbdLg0gVFO16wM=";
    let te = TransactionEnvelope::from_xdr_base64(xdr.to_string()).unwrap();
    println!("{:?}", te);

    if let TransactionEnvelope::EnvelopeTypeTx(te) = te {
        assert_eq!(te.tx.seq_num, SequenceNumber(2470486663495685));
        if let OperationBody::CreateAccount(op) = &te.tx.operations.as_vec()[0].body {
            assert_eq!(op.starting_balance, 100000000000);
        } else {
            panic!("op is not the type expected");
        }
    } else {
        panic!("envelope is not the type expected");
    }
    Ok(())
}
