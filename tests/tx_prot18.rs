#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(all(feature = "std", feature = "base64"))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{Error, Limits, OperationBody, ReadXdr, SequenceNumber, TransactionEnvelope};

#[test]
fn test_parse_pubnet_v18_tx() -> Result<(), Error> {
    let xdr = "AAAAAgAAAAA/ESDPPSBIB8pWPGt/zZ3dSJhShRxziDdkmLQXrdytCQAPQkAACMblAAAABQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAAAtDSg//ZfvJXgv2/0yiA7QUDWdXpKYhdjYEWkN4yVm+AAAABdIdugAAAAAAAAAAAKt3K0JAAAAQC3/n83fG/BCSRaIQjuqL2i1koiCHChxt1aagXn2ABCRP9IL83u5zldxuUaDBklKOHEdy4cOvl2BhPNbjs7w0QSGVuCcAAAAQKxHSgHZgZY7AMlPumIt0iZvtkbsRAtt6BYahJdnxrqm3+JuCVv/1ijWi1kM85uLfo7NAITi1TbdLg0gVFO16wM=";
    let te = TransactionEnvelope::from_xdr_base64(xdr, Limits::none())?;
    println!("{te:?}");

    if let TransactionEnvelope::Tx(te) = te {
        assert_eq!(te.tx.seq_num, SequenceNumber(2_470_486_663_495_685));
        if let OperationBody::CreateAccount(op) = &te.tx.operations.as_vec()[0].body {
            assert_eq!(op.starting_balance, 100_000_000_000);
        } else {
            panic!("op is not the type expected");
        }
    } else {
        panic!("envelope is not the type expected");
    }
    Ok(())
}
