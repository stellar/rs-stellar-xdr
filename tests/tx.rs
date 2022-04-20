use stellar_xdr::*;

#[test]
fn test_build_small_tx() {
    let te = TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::KeyTypeEd25519(Uint256([0; 32])),
            fee: Uint32(0),
            seq_num: SequenceNumber(Int64(1)),
            cond: Preconditions::PrecondNone,
            memo: Memo::MemoText("Stellar".as_bytes().to_vec()),
            operations: [].to_vec(),
            ext: TransactionExt::V0,
        },
        signatures: [].to_vec(),
    });
    let xdr = te.to_xdr_base64().unwrap();
    assert_eq!(xdr, "AAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAABAAAAB1N0ZWxsYXIAAAAAAAAAAAAAAAAA");
}

#[test]
fn test_parse_pubnet_v18_tx() {
    let xdr = "AAAAAgAAAAA/ESDPPSBIB8pWPGt/zZ3dSJhShRxziDdkmLQXrdytCQAPQkAACMblAAAABQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAAAtDSg//ZfvJXgv2/0yiA7QUDWdXpKYhdjYEWkN4yVm+AAAABdIdugAAAAAAAAAAAKt3K0JAAAAQC3/n83fG/BCSRaIQjuqL2i1koiCHChxt1aagXn2ABCRP9IL83u5zldxuUaDBklKOHEdy4cOvl2BhPNbjs7w0QSGVuCcAAAAQKxHSgHZgZY7AMlPumIt0iZvtkbsRAtt6BYahJdnxrqm3+JuCVv/1ijWi1kM85uLfo7NAITi1TbdLg0gVFO16wM=";
    let te = TransactionEnvelope::from_xdr_base64(xdr.to_string()).unwrap();
    println!("{:?}", te);

    if let TransactionEnvelope::EnvelopeTypeTx(te) = te {
        assert_eq!(te.tx.seq_num, SequenceNumber(Int64(2470486663495685)));
        if let OperationBody::CreateAccount(op) = &te.tx.operations[0].body {
            assert_eq!(op.starting_balance, Int64(100000000000));
        } else {
            panic!("op is not the type expected");
        }
    } else {
        panic!("envelope is not the type expected");
    }
}
