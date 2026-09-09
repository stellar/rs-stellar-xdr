//! A significantly complex `TransactionEnvelope` structure that covers every
//! shape the generator emits: nested structs, both union kinds with void,
//! scalar, fixed-opaque and heap arms, a heap-free struct used directly where
//! it has no `Const`, a cyclic field where the `Const` borrows and the owned
//! form boxes, `Option` in both states, `VecM` of `Const`s and of heap-free
//! values, an empty `VecM`, `StringM`, `BytesM`, and u32/u64/i64 scalars.
//!
//! The const and owned values are identical.

use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, BytesMConst, ClaimPredicate, ClaimPredicateConst,
    Claimant, ClaimantConst, ClaimantV0, ClaimantV0Const, CreateClaimableBalanceOp,
    CreateClaimableBalanceOpConst, DataValue, DataValueConst, DecoratedSignature,
    DecoratedSignatureConst, Duration, LedgerFootprint, LedgerFootprintConst, LedgerKey,
    LedgerKeyAccount, LedgerKeyConst, ManageDataOp, ManageDataOpConst, Memo, MemoConst,
    MuxedAccount, MuxedAccountMed25519, Operation, OperationBody, OperationBodyConst,
    OperationConst, PaymentOp, Preconditions, PreconditionsConst, PreconditionsV2,
    PreconditionsV2Const, PublicKey, SequenceNumber, Signature, SignatureConst, SignatureHint,
    SignerKey, SignerKeyConst, SignerKeyEd25519SignedPayload, SignerKeyEd25519SignedPayloadConst,
    SorobanResources, SorobanResourcesConst, SorobanTransactionData, SorobanTransactionDataConst,
    SorobanTransactionDataExt, SorobanTransactionDataExtConst, String64, String64Const,
    StringMConst, TimeBounds, TimePoint, Transaction, TransactionConst, TransactionEnvelope,
    TransactionEnvelopeConst, TransactionExt, TransactionExtConst, TransactionV1Envelope,
    TransactionV1EnvelopeConst, Uint256, VecMConst,
};

#[allow(clippy::too_many_lines)]
pub const fn tx_env_const() -> TransactionEnvelopeConst {
    TransactionEnvelopeConst::Tx(TransactionV1EnvelopeConst {
        tx: TransactionConst {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 0xdead_beef_0000_0001,
                ed25519: Uint256([0x11; 32]),
            }),
            fee: 4_321,
            seq_num: SequenceNumber(9_876_543_210),
            cond: PreconditionsConst::V2(PreconditionsV2Const {
                time_bounds: Some(TimeBounds {
                    min_time: TimePoint(100),
                    max_time: TimePoint(200),
                }),
                ledger_bounds: None,
                min_seq_num: Some(SequenceNumber(42)),
                min_seq_age: Duration(3_600),
                min_seq_ledger_gap: 7,
                extra_signers: VecMConst::try_from_slice_or_panic(
                    &const {
                        [
                            SignerKeyConst::Ed25519(Uint256([0x33; 32])),
                            SignerKeyConst::Ed25519SignedPayload(
                                SignerKeyEd25519SignedPayloadConst {
                                    ed25519: Uint256([0x44; 32]),
                                    payload: BytesMConst::try_from_slice_or_panic(
                                        b"signed payload",
                                    ),
                                },
                            ),
                        ]
                    },
                ),
            }),
            memo: MemoConst::Text(StringMConst::try_from_str_or_panic("hello world")),
            operations: VecMConst::try_from_slice_or_panic(
                &const {
                    [
                        OperationConst {
                            source_account: None,
                            body: OperationBodyConst::Payment(PaymentOp {
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
                        OperationConst {
                            source_account: Some(MuxedAccount::Ed25519(Uint256([0x66; 32]))),
                            body: OperationBodyConst::ManageData(ManageDataOpConst {
                                data_name: String64Const(StringMConst::try_from_str_or_panic(
                                    "config",
                                )),
                                data_value: Some(DataValueConst(
                                    BytesMConst::try_from_slice_or_panic(b"value"),
                                )),
                            }),
                        },
                        OperationConst {
                            source_account: None,
                            body: OperationBodyConst::Inflation,
                        },
                        OperationConst {
                            source_account: None,
                            body: OperationBodyConst::CreateClaimableBalance(
                                CreateClaimableBalanceOpConst {
                                    asset: Asset::Native,
                                    amount: 250_000_000,
                                    claimants: VecMConst::try_from_slice_or_panic(
                                        &const {
                                            [ClaimantConst::ClaimantTypeV0(ClaimantV0Const {
                                                destination: AccountId(
                                                    PublicKey::PublicKeyTypeEd25519(Uint256(
                                                        [0x22; 32],
                                                    )),
                                                ),
                                                predicate: ClaimPredicateConst::Not(Some(
                                                    &ClaimPredicateConst::BeforeRelativeTime(
                                                        86_400,
                                                    ),
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
            ext: TransactionExtConst::V1(SorobanTransactionDataConst {
                ext: SorobanTransactionDataExtConst::V0,
                resources: SorobanResourcesConst {
                    footprint: LedgerFootprintConst {
                        read_only: VecMConst::try_from_slice_or_panic(&[LedgerKeyConst::Account(
                            LedgerKeyAccount {
                                account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                    [0x22; 32],
                                ))),
                            },
                        )]),
                        read_write: VecMConst::try_from_slice_or_panic(&[]),
                    },
                    instructions: 1_000_000,
                    disk_read_bytes: 2_048,
                    write_bytes: 512,
                },
                resource_fee: 100_000,
            }),
        },
        signatures: VecMConst::try_from_slice_or_panic(
            &const {
                [
                    DecoratedSignatureConst {
                        hint: SignatureHint([1, 2, 3, 4]),
                        signature: SignatureConst(BytesMConst::try_from_slice_or_panic(b"sig-one")),
                    },
                    DecoratedSignatureConst {
                        hint: SignatureHint([5, 6, 7, 8]),
                        signature: SignatureConst(BytesMConst::try_from_slice_or_panic(b"sig-two")),
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
