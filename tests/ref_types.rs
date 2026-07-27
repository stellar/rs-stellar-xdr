#![allow(clippy::items_after_test_module)]

use stellar_xdr::{
    Asset, BytesMRef, ClaimPredicateRef, DecoratedSignatureRef, Error, MemoRef, MuxedAccount,
    OperationBodyRef, OperationRef, PaymentOp, PreconditionsRef, ScBytesRef, ScSymbolRef,
    ScValRef, ScVecRef, SequenceNumber, SignatureHint, SignatureRef, StringMRef,
    TransactionEnvelopeRef, TransactionExtRef, TransactionRef, TransactionV1EnvelopeRef, Uint256,
    VecMRef,
};

// A complete transaction envelope built entirely in a const context from
// borrowed data: slices of fixed-size arrays, string literals, and references.

const OPERATIONS: [OperationRef; 2] = [
    OperationRef {
        source_account: None,
        body: OperationBodyRef::Payment(PaymentOp {
            destination: MuxedAccount::Ed25519(Uint256([1; 32])),
            asset: Asset::Native,
            amount: 100,
        }),
    },
    OperationRef {
        source_account: Some(MuxedAccount::Ed25519(Uint256([2; 32]))),
        body: OperationBodyRef::Payment(PaymentOp {
            destination: MuxedAccount::Ed25519(Uint256([3; 32])),
            asset: Asset::Native,
            amount: 200,
        }),
    },
];

const SIGNATURES: [DecoratedSignatureRef; 1] = [DecoratedSignatureRef {
    hint: SignatureHint([1, 2, 3, 4]),
    signature: SignatureRef(BytesMRef::new(&[9; 64])),
}];

const TX: TransactionRef = TransactionRef {
    source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
    fee: 100,
    seq_num: SequenceNumber(7),
    cond: PreconditionsRef::None,
    memo: MemoRef::Text(StringMRef::new_str("hello")),
    operations: VecMRef::new(&OPERATIONS),
    ext: TransactionExtRef::V0,
};

const ENVELOPE: TransactionEnvelopeRef = TransactionEnvelopeRef::Tx(TransactionV1EnvelopeRef {
    tx: TX,
    signatures: VecMRef::new(&SIGNATURES),
});

// A recursive ScVal built in a const context: a vec of vals, one of which is
// itself a vec.

const SCVAL_LEAVES: [ScValRef; 3] = [
    ScValRef::I32(1),
    ScValRef::Symbol(ScSymbolRef(StringMRef::new_str("sym"))),
    ScValRef::Bytes(ScBytesRef(BytesMRef::new(b"bytes"))),
];

const SCVAL: ScValRef = ScValRef::Vec(Some(ScVecRef(VecMRef::new(&[
    ScValRef::Vec(Some(ScVecRef(VecMRef::new(&SCVAL_LEAVES)))),
    ScValRef::Bool(true),
]))));

// A cyclic type built in a const context: where the owned type boxes the
// cycle (`Option<Box<ClaimPredicate>>`), the Ref type borrows instead.

const PREDICATE: ClaimPredicateRef = ClaimPredicateRef::Not(Some(&ClaimPredicateRef::And(
    VecMRef::new(&[
        ClaimPredicateRef::Unconditional,
        ClaimPredicateRef::BeforeAbsoluteTime(123),
    ]),
)));

#[test]
fn const_constructed_values() {
    assert!(matches!(
        ENVELOPE,
        TransactionEnvelopeRef::Tx(TransactionV1EnvelopeRef { .. })
    ));
    assert_eq!(TX.fee, 100);
    assert_eq!(TX.operations.len(), 2);
    assert_eq!(TX.operations.as_slice().len(), 2);
    assert!(matches!(TX.memo, MemoRef::Text(m) if m.as_slice() == b"hello"));
    assert!(matches!(SCVAL, ScValRef::Vec(Some(v)) if v.0.len() == 2));
    assert!(matches!(
        PREDICATE,
        ClaimPredicateRef::Not(Some(ClaimPredicateRef::And(p))) if p.len() == 2
    ));
}

#[test]
fn vecm_ref_construction_limits() {
    let elems = [1u32, 2, 3];
    let v = VecMRef::<u32, 3>::try_new(&elems).unwrap();
    assert_eq!(v.len(), 3);
    assert!(!v.is_empty());
    assert_eq!(v.as_slice(), &[1, 2, 3]);
    assert_eq!(v.iter().copied().sum::<u32>(), 6);
    assert_eq!(v.max_len(), 3);
    assert_eq!(
        VecMRef::<u32, 2>::try_new(&elems),
        Err(Error::LengthExceedsMax)
    );
}

#[test]
#[should_panic(expected = "length exceeds max")]
fn vecm_ref_new_panics_over_max() {
    let elems = [1u32, 2, 3];
    let _ = VecMRef::<u32, 2>::new(&elems);
}

#[test]
fn bytesm_ref_construction_limits() {
    let v = BytesMRef::<3>::try_new(b"abc").unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(v.as_slice(), b"abc");
    assert_eq!(BytesMRef::<2>::try_new(b"abc"), Err(Error::LengthExceedsMax));
}

#[test]
fn stringm_ref_construction_limits() {
    let v = StringMRef::<5>::try_new_str("abc").unwrap();
    assert_eq!(v.as_slice(), b"abc");
    let v = StringMRef::<5>::try_new(b"abc").unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(
        StringMRef::<2>::try_new_str("abc"),
        Err(Error::LengthExceedsMax)
    );
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use stellar_xdr::{
        BytesM, ClaimPredicate, DecoratedSignature, Memo, Operation, OperationBody, Preconditions,
        ScBytes, ScSymbol, ScVal, ScVec, Signature, StringM, Transaction, TransactionEnvelope,
        TransactionExt, TransactionV1Envelope, VecM,
    };

    fn owned_envelope() -> TransactionEnvelope {
        TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: Transaction {
                source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
                fee: 100,
                seq_num: SequenceNumber(7),
                cond: Preconditions::None,
                memo: Memo::Text("hello".try_into().unwrap()),
                operations: vec![
                    Operation {
                        source_account: None,
                        body: OperationBody::Payment(PaymentOp {
                            destination: MuxedAccount::Ed25519(Uint256([1; 32])),
                            asset: Asset::Native,
                            amount: 100,
                        }),
                    },
                    Operation {
                        source_account: Some(MuxedAccount::Ed25519(Uint256([2; 32]))),
                        body: OperationBody::Payment(PaymentOp {
                            destination: MuxedAccount::Ed25519(Uint256([3; 32])),
                            asset: Asset::Native,
                            amount: 200,
                        }),
                    },
                ]
                .try_into()
                .unwrap(),
                ext: TransactionExt::V0,
            },
            signatures: vec![DecoratedSignature {
                hint: SignatureHint([1, 2, 3, 4]),
                signature: Signature(vec![9u8; 64].try_into().unwrap()),
            }]
            .try_into()
            .unwrap(),
        })
    }

    #[test]
    fn envelope_ref_to_owned() {
        let by_ref: TransactionEnvelope = (&ENVELOPE).into();
        let by_value: TransactionEnvelope = ENVELOPE.into();
        assert_eq!(by_ref, owned_envelope());
        assert_eq!(by_value, owned_envelope());
    }

    #[test]
    fn scval_ref_to_owned() {
        let owned: ScVal = (&SCVAL).into();
        let expected = ScVal::Vec(Some(ScVec(
            vec![
                ScVal::Vec(Some(ScVec(
                    vec![
                        ScVal::I32(1),
                        ScVal::Symbol(ScSymbol("sym".try_into().unwrap())),
                        ScVal::Bytes(ScBytes(b"bytes".to_vec().try_into().unwrap())),
                    ]
                    .try_into()
                    .unwrap(),
                ))),
                ScVal::Bool(true),
            ]
            .try_into()
            .unwrap(),
        )));
        assert_eq!(owned, expected);
    }

    #[test]
    fn claim_predicate_ref_to_owned() {
        let owned: ClaimPredicate = (&PREDICATE).into();
        let expected = ClaimPredicate::Not(Some(Box::new(ClaimPredicate::And(
            vec![
                ClaimPredicate::Unconditional,
                ClaimPredicate::BeforeAbsoluteTime(123),
            ]
            .try_into()
            .unwrap(),
        ))));
        assert_eq!(owned, expected);
    }

    #[test]
    fn ref_types_from_owned_m_types() {
        let vecm: VecM<u32, 5> = vec![1, 2, 3].try_into().unwrap();
        let vecm_ref: VecMRef<u32, 5> = (&vecm).into();
        assert_eq!(vecm_ref.to_vecm(), vecm);

        let bytesm: BytesM<5> = vec![1u8, 2, 3].try_into().unwrap();
        let bytesm_ref: BytesMRef<5> = (&bytesm).into();
        assert_eq!(bytesm_ref.to_bytesm(), bytesm);

        let stringm: StringM<5> = "abc".try_into().unwrap();
        let stringm_ref: StringMRef<5> = (&stringm).into();
        assert_eq!(stringm_ref.to_stringm(), stringm);
    }

    #[cfg(feature = "std")]
    #[test]
    fn envelope_ref_xdr_roundtrip() {
        use stellar_xdr::{Limits, ReadXdr, WriteXdr};
        let owned: TransactionEnvelope = (&ENVELOPE).into();
        let bytes = owned.to_xdr(Limits::none()).unwrap();
        assert_eq!(bytes, owned_envelope().to_xdr(Limits::none()).unwrap());
        let decoded = TransactionEnvelope::from_xdr(bytes, Limits::none()).unwrap();
        assert_eq!(owned, decoded);
    }
}
