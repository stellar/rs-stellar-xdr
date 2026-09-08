//! A significantly complex `TransactionEnvelope` structure that covers every
//! shape the generator emits: nested structs, both union kinds with void,
//! scalar, fixed-opaque and heap arms, a heap-free struct used directly where
//! it has no `Ref`, a cyclic field where the `Ref` borrows and the owned form
//! boxes, `Option` in both states, `VecM` of `Ref`s and of heap-free values, an
//! empty `VecM`, `StringM`, `BytesM`, and u32/u64/ i64 scalars.
//!
//! The ref and owned values are identical.

use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, BytesMRef, ClaimPredicate, ClaimPredicateRef,
    Claimant, ClaimantRef, ClaimantV0, ClaimantV0Ref, CreateClaimableBalanceOp,
    CreateClaimableBalanceOpRef, DataValue, DataValueRef, DecoratedSignature,
    DecoratedSignatureRef, Duration, LedgerFootprint, LedgerFootprintRef, LedgerKey,
    LedgerKeyAccount, LedgerKeyRef, ManageDataOp, ManageDataOpRef, Memo, MemoRef, MuxedAccount,
    MuxedAccountMed25519, Operation, OperationBody, OperationBodyRef, OperationRef, PaymentOp,
    Preconditions, PreconditionsRef, PreconditionsV2, PreconditionsV2Ref, PublicKey,
    SequenceNumber, Signature, SignatureHint, SignatureRef, SignerKey,
    SignerKeyEd25519SignedPayload, SignerKeyEd25519SignedPayloadRef, SignerKeyRef,
    SorobanResources, SorobanResourcesRef, SorobanTransactionData, SorobanTransactionDataExt,
    SorobanTransactionDataExtRef, SorobanTransactionDataRef, String64, String64Ref, StringMRef,
    TimeBounds, TimePoint, Transaction, TransactionEnvelope, TransactionEnvelopeRef,
    TransactionExt, TransactionExtRef, TransactionRef, TransactionV1Envelope,
    TransactionV1EnvelopeRef, Uint256, VecMRef,
};

#[allow(clippy::too_many_lines)]
pub const fn tx_env_ref() -> TransactionEnvelopeRef<'static> {
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

#[allow(clippy::too_many_lines)]
pub fn tx_env_owned() -> TransactionEnvelope {
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
