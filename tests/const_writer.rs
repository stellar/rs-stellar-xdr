//! Tests for the const XDR serializer.

#![cfg(all(feature = "const", feature = "std"))]

use stellar_xdr::{Limits, TransactionEnvelope, TransactionEnvelopeRef, WriteXdr};

mod common;
use common::{tx_env_owned, tx_env_ref};

#[test]
fn ref_const_and_owned_encode_same() {
    const R: TransactionEnvelopeRef = const { tx_env_ref() };
    let o: TransactionEnvelope = tx_env_owned();

    let r_xdr: [u8; R.const_xdr_len()] = R.const_to_xdr();
    let o_xdr = o.to_xdr(Limits::none()).unwrap();
    assert_eq!(r_xdr, o_xdr.as_slice());
}
