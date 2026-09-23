//! Decoding a transaction envelope that adds an ed25519 signed payload signer,
//! whose payload must be 1 to 64 bytes. The XDR definition only expresses the
//! maximum, so an empty payload is rejected by the `BytesM` minimum length that
//! the generator is configured to give the payload.

#![cfg(feature = "std")]

use crate::{
    Error, Limits, Memo, MuxedAccount, Operation, OperationBody, Preconditions, ReadXdr,
    SequenceNumber, SetOptionsOp, Signer, SignerKey, SignerKeyEd25519SignedPayload, Transaction,
    TransactionEnvelope, TransactionExt, TransactionV1Envelope, Uint256, WriteXdr,
};

const ED25519: [u8; 32] = [0x11; 32];

/// A transaction envelope with a set options operation that adds a signed
/// payload signer with a one byte payload of `0xab`.
fn envelope() -> TransactionEnvelope {
    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 100,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: [Operation {
                source_account: None,
                body: OperationBody::SetOptions(SetOptionsOp {
                    inflation_dest: None,
                    clear_flags: None,
                    set_flags: None,
                    master_weight: None,
                    low_threshold: None,
                    med_threshold: None,
                    high_threshold: None,
                    home_domain: None,
                    signer: Some(Signer {
                        key: SignerKey::Ed25519SignedPayload(SignerKeyEd25519SignedPayload {
                            ed25519: Uint256(ED25519),
                            payload: [0xab].try_into().unwrap(),
                        }),
                        weight: 1,
                    }),
                }),
            }]
            .try_into()
            .unwrap(),
            ext: TransactionExt::V0,
        },
        signatures: [].try_into().unwrap(),
    })
}

#[test]
fn tx_envelope_with_valid_signed_payload_signer_decodes() {
    let te = envelope();
    let xdr = te.to_xdr(Limits::none()).unwrap();
    let decoded = TransactionEnvelope::from_xdr(xdr, Limits::none());
    assert_eq!(decoded, Ok(te));
}

#[test]
fn tx_envelope_with_empty_signed_payload_signer_does_not_decode() {
    // The XDR types cannot hold an empty payload, so build the XDR with a one
    // byte payload and rewrite the payload's length, byte, and padding as an
    // empty payload, which XDR encodes as its zero length alone.
    let xdr = envelope().to_xdr(Limits::none()).unwrap();
    let valid = [&ED25519[..], &[0, 0, 0, 1, 0xab, 0, 0, 0]].concat();
    let empty = [&ED25519[..], &[0, 0, 0, 0]].concat();
    let at = xdr
        .windows(valid.len())
        .position(|w| w == valid.as_slice())
        .unwrap();
    let xdr = [&xdr[..at], &empty[..], &xdr[at + valid.len()..]].concat();

    let decoded = TransactionEnvelope::from_xdr(xdr, Limits::none());
    assert_eq!(decoded, Err(Error::LengthBelowMin));
}
