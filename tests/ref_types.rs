//! Tests for the borrowing `Ref` types.

use stellar_xdr::{BytesMRef, ErrorLengthExceedsMax, StringMRef, VecMRef};

#[cfg(feature = "alloc")]
use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, ClaimPredicate, ClaimPredicateRef, Claimant,
    ClaimantRef, ClaimantV0, ClaimantV0Ref, CreateClaimableBalanceOp, CreateClaimableBalanceOpRef,
    DataValue, DataValueRef, DecoratedSignature, DecoratedSignatureRef, Duration, LedgerFootprint,
    LedgerFootprintRef, LedgerKey, LedgerKeyAccount, LedgerKeyRef, ManageDataOp, ManageDataOpRef,
    Memo, MemoRef, MuxedAccount, MuxedAccountMed25519, Operation, OperationBody, OperationBodyRef,
    OperationRef, PaymentOp, Preconditions, PreconditionsRef, PreconditionsV2, PreconditionsV2Ref,
    PublicKey, SequenceNumber, Signature, SignatureHint, SignatureRef, SignerKey,
    SignerKeyEd25519SignedPayload, SignerKeyEd25519SignedPayloadRef, SignerKeyRef,
    SorobanResources, SorobanResourcesRef, SorobanTransactionData, SorobanTransactionDataExt,
    SorobanTransactionDataExtRef, SorobanTransactionDataRef, String64, String64Ref, TimeBounds,
    TimePoint, Transaction, TransactionEnvelope, TransactionEnvelopeRef, TransactionExt,
    TransactionExtRef, TransactionRef, TransactionV1Envelope, TransactionV1EnvelopeRef, Uint256,
};

#[cfg(feature = "std")]
use stellar_xdr::{Limits, WriteXdr};
#[test]
fn refs_try_from_enforce_max_len() {
    assert!(VecMRef::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
    assert_eq!(
        VecMRef::<u32, 2>::try_from_slice(&[1, 2, 3]),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesMRef::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesMRef::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringMRef::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringMRef::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    assert!(StringMRef::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        StringMRef::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );
}

#[test]
fn refs_try_from_or_panic() {
    assert_eq!(
        VecMRef::<u32, 3>::try_from_slice_or_panic(&[1, 2, 3]),
        VecMRef::<u32, 3>::try_from_slice(&[1, 2, 3]).unwrap()
    );
    assert_eq!(
        BytesMRef::<3>::try_from_slice_or_panic(b"abc"),
        BytesMRef::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringMRef::<3>::try_from_str_or_panic("abc"),
        StringMRef::<3>::try_from_str("abc").unwrap()
    );
    assert_eq!(
        StringMRef::<3>::try_from_slice_or_panic(b"abc"),
        StringMRef::<3>::try_from_slice(b"abc").unwrap()
    );
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = VecMRef::<u32, 2>::try_from_slice_or_panic(&[1, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = BytesMRef::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_ref_try_from_or_panic_panics_enforce_max_len() {
    let _ = StringMRef::<2>::try_from_str_or_panic("abc");
}

#[test]
fn refs_default() {
    assert_eq!(
        VecMRef::<u32, 3>::default(),
        VecMRef::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        BytesMRef::<3>::default(),
        BytesMRef::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        StringMRef::<3>::default(),
        StringMRef::try_from_str("").unwrap()
    );
}

#[cfg(feature = "alloc")]
#[test]
fn ref_converts_to_owned() {
    // A ref and owned value that are identically defined.
    let r: TransactionEnvelopeRef = const { xdr_ref() };
    let owned: TransactionEnvelope = owned();

    assert_eq!(TransactionEnvelope::from(&r), owned);
}

#[cfg(feature = "std")]
#[test]
fn ref_and_owned_encode_same() {
    // A ref and owned value that are identically defined.
    let r: TransactionEnvelopeRef = const { xdr_ref() };
    let owned: TransactionEnvelope = owned();

    // Ref and owned encode to the same XDR.
    let ref_xdr = r.to_xdr(Limits::none()).unwrap();
    let owned_xdr = owned.to_xdr(Limits::none()).unwrap();
    assert_eq!(ref_xdr, owned_xdr);
}

#[cfg(feature = "alloc")]
#[allow(clippy::too_many_lines)]
const fn xdr_ref() -> TransactionEnvelopeRef<'static> {
    TransactionEnvelopeRef::Tx(TransactionV1EnvelopeRef {
        tx: TransactionRef {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 0xdead_beef_0000_0001,
                ed25519: Uint256([0x11; 32]),
            }),
            fee: 4_321,
            seq_num: SequenceNumber(9_876_543_210),
            cond: PreconditionsRef::V2(PreconditionsV2Ref {
                time_bounds: Some(TimeBounds {
                    min_time: TimePoint(100),
                    max_time: TimePoint(200),
                }),
                ledger_bounds: None,
                min_seq_num: Some(SequenceNumber(42)),
                min_seq_age: Duration(3_600),
                min_seq_ledger_gap: 7,
                extra_signers: VecMRef::try_from_slice_or_panic(
                    &const {
                        [
                            SignerKeyRef::Ed25519(Uint256([0x33; 32])),
                            SignerKeyRef::Ed25519SignedPayload(SignerKeyEd25519SignedPayloadRef {
                                ed25519: Uint256([0x44; 32]),
                                payload: BytesMRef::try_from_slice_or_panic(b"signed payload"),
                            }),
                        ]
                    },
                ),
            }),
            memo: MemoRef::Text(StringMRef::try_from_str_or_panic("hello world")),
            operations: VecMRef::try_from_slice_or_panic(
                &const {
                    [
                        OperationRef {
                            source_account: None,
                            body: OperationBodyRef::Payment(PaymentOp {
                                destination: MuxedAccount::Ed25519(Uint256([0x55; 32])),
                                asset: Asset::CreditAlphanum4(AlphaNum4 {
                                    asset_code: AssetCode4(*b"USDC"),
                                    issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                        [0x22; 32],
                                    ))),
                                }),
                                amount: 1_000_000_007,
                            }),
                        },
                        OperationRef {
                            source_account: Some(MuxedAccount::Ed25519(Uint256([0x66; 32]))),
                            body: OperationBodyRef::ManageData(ManageDataOpRef {
                                data_name: String64Ref(StringMRef::try_from_str_or_panic("config")),
                                data_value: Some(DataValueRef(BytesMRef::try_from_slice_or_panic(
                                    b"value",
                                ))),
                            }),
                        },
                        OperationRef {
                            source_account: None,
                            body: OperationBodyRef::Inflation,
                        },
                        OperationRef {
                            source_account: None,
                            body: OperationBodyRef::CreateClaimableBalance(
                                CreateClaimableBalanceOpRef {
                                    asset: Asset::Native,
                                    amount: 250_000_000,
                                    claimants: VecMRef::try_from_slice_or_panic(
                                        &const {
                                            [ClaimantRef::ClaimantTypeV0(ClaimantV0Ref {
                                                destination: AccountId(
                                                    PublicKey::PublicKeyTypeEd25519(Uint256(
                                                        [0x22; 32],
                                                    )),
                                                ),
                                                predicate: ClaimPredicateRef::Not(Some(
                                                    &ClaimPredicateRef::BeforeRelativeTime(86_400),
                                                )),
                                            })]
                                        },
                                    ),
                                },
                            ),
                        },
                    ]
                },
            ),
            ext: TransactionExtRef::V1(SorobanTransactionDataRef {
                ext: SorobanTransactionDataExtRef::V0,
                resources: SorobanResourcesRef {
                    footprint: LedgerFootprintRef {
                        read_only: VecMRef::try_from_slice_or_panic(&[LedgerKeyRef::Account(
                            LedgerKeyAccount {
                                account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                    [0x22; 32],
                                ))),
                            },
                        )]),
                        read_write: VecMRef::try_from_slice_or_panic(&[]),
                    },
                    instructions: 1_000_000,
                    disk_read_bytes: 2_048,
                    write_bytes: 512,
                },
                resource_fee: 100_000,
            }),
        },
        signatures: VecMRef::try_from_slice_or_panic(
            &const {
                [
                    DecoratedSignatureRef {
                        hint: SignatureHint([1, 2, 3, 4]),
                        signature: SignatureRef(BytesMRef::try_from_slice_or_panic(b"sig-one")),
                    },
                    DecoratedSignatureRef {
                        hint: SignatureHint([5, 6, 7, 8]),
                        signature: SignatureRef(BytesMRef::try_from_slice_or_panic(b"sig-two")),
                    },
                ]
            },
        ),
    })
}

#[cfg(feature = "alloc")]
#[allow(clippy::too_many_lines)]
fn owned() -> TransactionEnvelope {
    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 0xdead_beef_0000_0001,
                ed25519: Uint256([0x11; 32]),
            }),
            fee: 4_321,
            seq_num: SequenceNumber(9_876_543_210),
            cond: Preconditions::V2(PreconditionsV2 {
                time_bounds: Some(TimeBounds {
                    min_time: TimePoint(100),
                    max_time: TimePoint(200),
                }),
                ledger_bounds: None,
                min_seq_num: Some(SequenceNumber(42)),
                min_seq_age: Duration(3_600),
                min_seq_ledger_gap: 7,
                extra_signers: vec![
                    SignerKey::Ed25519(Uint256([0x33; 32])),
                    SignerKey::Ed25519SignedPayload(SignerKeyEd25519SignedPayload {
                        ed25519: Uint256([0x44; 32]),
                        payload: b"signed payload".to_vec().try_into().unwrap(),
                    }),
                ]
                .try_into()
                .unwrap(),
            }),
            memo: Memo::Text("hello world".try_into().unwrap()),
            operations: vec![
                Operation {
                    source_account: None,
                    body: OperationBody::Payment(PaymentOp {
                        destination: MuxedAccount::Ed25519(Uint256([0x55; 32])),
                        asset: Asset::CreditAlphanum4(AlphaNum4 {
                            asset_code: AssetCode4(*b"USDC"),
                            issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0x22; 32]))),
                        }),
                        amount: 1_000_000_007,
                    }),
                },
                Operation {
                    source_account: Some(MuxedAccount::Ed25519(Uint256([0x66; 32]))),
                    body: OperationBody::ManageData(ManageDataOp {
                        data_name: String64("config".try_into().unwrap()),
                        data_value: Some(DataValue(b"value".to_vec().try_into().unwrap())),
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::Inflation,
                },
                Operation {
                    source_account: None,
                    body: OperationBody::CreateClaimableBalance(CreateClaimableBalanceOp {
                        asset: Asset::Native,
                        amount: 250_000_000,
                        claimants: vec![Claimant::ClaimantTypeV0(ClaimantV0 {
                            destination: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                [0x22; 32],
                            ))),
                            predicate: ClaimPredicate::Not(Some(Box::new(
                                ClaimPredicate::BeforeRelativeTime(86_400),
                            ))),
                        })]
                        .try_into()
                        .unwrap(),
                    }),
                },
            ]
            .try_into()
            .unwrap(),
            ext: TransactionExt::V1(SorobanTransactionData {
                ext: SorobanTransactionDataExt::V0,
                resources: SorobanResources {
                    footprint: LedgerFootprint {
                        read_only: vec![LedgerKey::Account(LedgerKeyAccount {
                            account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                [0x22; 32],
                            ))),
                        })]
                        .try_into()
                        .unwrap(),
                        read_write: vec![].try_into().unwrap(),
                    },
                    instructions: 1_000_000,
                    disk_read_bytes: 2_048,
                    write_bytes: 512,
                },
                resource_fee: 100_000,
            }),
        },
        signatures: vec![
            DecoratedSignature {
                hint: SignatureHint([1, 2, 3, 4]),
                signature: Signature(b"sig-one".to_vec().try_into().unwrap()),
            },
            DecoratedSignature {
                hint: SignatureHint([5, 6, 7, 8]),
                signature: Signature(b"sig-two".to_vec().try_into().unwrap()),
            },
        ]
        .try_into()
        .unwrap(),
    })
}
