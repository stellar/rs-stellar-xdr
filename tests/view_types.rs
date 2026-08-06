#![allow(clippy::items_after_test_module)]

use stellar_xdr::{
    Asset, BytesMView, ClaimPredicateView, DecoratedSignatureView, Error, MemoView, MuxedAccount,
    OperationBodyView, OperationView, PaymentOp, PreconditionsView, ScBytesView, ScSymbolView,
    ScValView, ScVecView, SequenceNumber, SignatureHint, SignatureView, StringMView,
    TransactionEnvelopeView, TransactionExtView, TransactionV1EnvelopeView, TransactionView,
    Uint256, VecMView,
};

// A complete transaction envelope built entirely in a const context from
// borrowed data: slices of fixed-size arrays, string literals, and references.

const OPERATIONS: [OperationView; 2] = [
    OperationView {
        source_account: None,
        body: OperationBodyView::Payment(PaymentOp {
            destination: MuxedAccount::Ed25519(Uint256([1; 32])),
            asset: Asset::Native,
            amount: 100,
        }),
    },
    OperationView {
        source_account: Some(MuxedAccount::Ed25519(Uint256([2; 32]))),
        body: OperationBodyView::Payment(PaymentOp {
            destination: MuxedAccount::Ed25519(Uint256([3; 32])),
            asset: Asset::Native,
            amount: 200,
        }),
    },
];

const SIGNATURES: [DecoratedSignatureView; 1] = [DecoratedSignatureView {
    hint: SignatureHint([1, 2, 3, 4]),
    signature: SignatureView(BytesMView::new(&[9; 64])),
}];

const TX: TransactionView = TransactionView {
    source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
    fee: 100,
    seq_num: SequenceNumber(7),
    cond: PreconditionsView::None,
    memo: MemoView::Text(StringMView::new_str("hello")),
    operations: VecMView::new(&OPERATIONS),
    ext: TransactionExtView::V0,
};

const ENVELOPE: TransactionEnvelopeView = TransactionEnvelopeView::Tx(TransactionV1EnvelopeView {
    tx: TX,
    signatures: VecMView::new(&SIGNATURES),
});

// A recursive ScVal built in a const context: a vec of vals, one of which is
// itself a vec.

const SCVAL_LEAVES: [ScValView; 3] = [
    ScValView::I32(1),
    ScValView::Symbol(ScSymbolView(StringMView::new_str("sym"))),
    ScValView::Bytes(ScBytesView(BytesMView::new(b"bytes"))),
];

const SCVAL: ScValView = ScValView::Vec(Some(ScVecView(VecMView::new(&[
    ScValView::Vec(Some(ScVecView(VecMView::new(&SCVAL_LEAVES)))),
    ScValView::Bool(true),
]))));

// A cyclic type built in a const context: where the owned type boxes the
// cycle (`Option<Box<ClaimPredicate>>`), the View type borrows instead.

const PREDICATE: ClaimPredicateView =
    ClaimPredicateView::Not(Some(&ClaimPredicateView::And(VecMView::new(&[
        ClaimPredicateView::Unconditional,
        ClaimPredicateView::BeforeAbsoluteTime(123),
    ]))));

#[test]
fn const_constructed_values() {
    assert!(matches!(
        ENVELOPE,
        TransactionEnvelopeView::Tx(TransactionV1EnvelopeView { .. })
    ));
    assert_eq!(TX.fee, 100);
    assert_eq!(TX.operations.len(), 2);
    assert_eq!(TX.operations.as_slice().len(), 2);
    assert!(matches!(TX.memo, MemoView::Text(m) if m.as_slice() == b"hello"));
    assert!(matches!(SCVAL, ScValView::Vec(Some(v)) if v.0.len() == 2));
    assert!(matches!(
        PREDICATE,
        ClaimPredicateView::Not(Some(ClaimPredicateView::And(p))) if p.len() == 2
    ));
}

#[test]
fn vecm_view_construction_limits() {
    let elems = [1u32, 2, 3];
    let v = VecMView::<u32, 3>::try_new(&elems).unwrap();
    assert_eq!(v.len(), 3);
    assert!(!v.is_empty());
    assert_eq!(v.as_slice(), &[1, 2, 3]);
    assert_eq!(v.iter().copied().sum::<u32>(), 6);
    assert_eq!(v.max_len(), 3);
    assert_eq!(
        VecMView::<u32, 2>::try_new(&elems),
        Err(Error::LengthExceedsMax)
    );
}

#[test]
#[should_panic(expected = "length exceeds max")]
fn vecm_view_new_panics_over_max() {
    let elems = [1u32, 2, 3];
    let _ = VecMView::<u32, 2>::new(&elems);
}

#[test]
fn bytesm_view_construction_limits() {
    let v = BytesMView::<3>::try_new(b"abc").unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(v.as_slice(), b"abc");
    assert_eq!(
        BytesMView::<2>::try_new(b"abc"),
        Err(Error::LengthExceedsMax)
    );
}

#[test]
fn stringm_view_construction_limits() {
    let v = StringMView::<5>::try_new_str("abc").unwrap();
    assert_eq!(v.as_slice(), b"abc");
    let v = StringMView::<5>::try_new(b"abc").unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(
        StringMView::<2>::try_new_str("abc"),
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
    fn envelope_view_to_owned() {
        let by_ref: TransactionEnvelope = (&ENVELOPE).into();
        let by_value: TransactionEnvelope = ENVELOPE.into();
        assert_eq!(by_ref, owned_envelope());
        assert_eq!(by_value, owned_envelope());
    }

    #[test]
    fn scval_view_to_owned() {
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
    fn claim_predicate_view_to_owned() {
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
    fn view_types_from_owned_m_types() {
        let vecm: VecM<u32, 5> = vec![1, 2, 3].try_into().unwrap();
        let vecm_view: VecMView<u32, 5> = (&vecm).into();
        assert_eq!(vecm_view.to_vecm(), vecm);

        let bytesm: BytesM<5> = vec![1u8, 2, 3].try_into().unwrap();
        let bytesm_view: BytesMView<5> = (&bytesm).into();
        assert_eq!(bytesm_view.to_bytesm(), bytesm);

        let stringm: StringM<5> = "abc".try_into().unwrap();
        let stringm_view: StringMView<5> = (&stringm).into();
        assert_eq!(stringm_view.to_stringm(), stringm);
    }

    #[cfg(feature = "std")]
    #[test]
    fn envelope_view_xdr_roundtrip() {
        use stellar_xdr::{Limits, ReadXdr, WriteXdr};
        let owned: TransactionEnvelope = (&ENVELOPE).into();
        let bytes = owned.to_xdr(Limits::none()).unwrap();
        assert_eq!(bytes, owned_envelope().to_xdr(Limits::none()).unwrap());
        let decoded = TransactionEnvelope::from_xdr(bytes, Limits::none()).unwrap();
        assert_eq!(owned, decoded);
    }

    // View types encode directly via WriteXdr, producing bytes identical to
    // the owned types'.
    #[cfg(feature = "std")]
    #[test]
    fn view_write_xdr_matches_owned() {
        use stellar_xdr::{Limits, WriteXdr};
        assert_eq!(
            ENVELOPE.to_xdr(Limits::none()).unwrap(),
            owned_envelope().to_xdr(Limits::none()).unwrap()
        );
        assert_eq!(
            SCVAL.to_xdr(Limits::none()).unwrap(),
            ScVal::from(&SCVAL).to_xdr(Limits::none()).unwrap()
        );
        assert_eq!(
            PREDICATE.to_xdr(Limits::none()).unwrap(),
            ClaimPredicate::from(&PREDICATE)
                .to_xdr(Limits::none())
                .unwrap()
        );
    }
}
