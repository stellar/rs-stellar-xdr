//! Tests for the const XDR serializer.

#![cfg(all(feature = "const", feature = "std"))]

use stellar_xdr::{r#const, Limits, ScSpecEntry, TransactionEnvelope, WriteXdr};

mod common;
use common::{spec_entry_const, spec_entry_owned, tx_env_const, tx_env_owned};

#[test]
fn const_and_owned_encode_same() {
    const R: r#const::TransactionEnvelope = const { tx_env_const() };
    let o: TransactionEnvelope = tx_env_owned();

    let r_xdr: [u8; R.const_xdr_len()] = R.const_to_xdr();
    let o_xdr = o.to_xdr(Limits::none()).unwrap();
    assert_eq!(r_xdr, o_xdr.as_slice());
}

/// The same check over a value whose recursive arms nest several levels deep,
/// which is where the const form's `&'static` borrows stand in for the owned
/// form's `Box`.
#[test]
fn const_and_owned_encode_same_spec_entry() {
    const R: r#const::ScSpecEntry = const { spec_entry_const() };
    let o: ScSpecEntry = spec_entry_owned();

    let r_xdr: [u8; R.const_xdr_len()] = R.const_to_xdr();
    let o_xdr = o.to_xdr(Limits::none()).unwrap();
    assert_eq!(r_xdr, o_xdr.as_slice());
}
