//! A significantly complex `TransactionEnvelope` structure that covers every
//! shape the generator emits: nested structs, both union kinds with void,
//! scalar, fixed-opaque and heap arms, a heap-free struct the `const` module
//! aliases rather than replaces, a cyclic field where the const form borrows
//! and the owned form boxes, `Option` in both states, `VecM` of borrowing and
//! of heap-free values, an empty `VecM`, `StringM`, `BytesM`, and u32/u64/i64
//! scalars.
//!
//! The const and owned values are identical.

pub use const_value::tx_env_const;
pub use owned_value::tx_env_owned;

/// The value as `const`, built from the `const` module's types.
mod const_value {
    use stellar_xdr::r#const::*;

    #[allow(clippy::too_many_lines)]
    pub const fn tx_env_const() -> TransactionEnvelope {
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
                    extra_signers: VecM::try_from_slice_or_panic(
                        &const {
                            [
                                SignerKey::Ed25519(Uint256([0x33; 32])),
                                SignerKey::Ed25519SignedPayload(SignerKeyEd25519SignedPayload {
                                    ed25519: Uint256([0x44; 32]),
                                    payload: BytesM::try_from_slice_or_panic(b"signed payload"),
                                }),
                            ]
                        },
                    ),
                }),
                memo: Memo::Text(StringM::try_from_str_or_panic("hello world")),
                operations: VecM::try_from_slice_or_panic(
                    &const {
                        [
                            Operation {
                                source_account: None,
                                body: OperationBody::Payment(PaymentOp {
                                    destination: MuxedAccount::Ed25519(Uint256([0x55; 32])),
                                    asset: Asset::CreditAlphanum4(AlphaNum4 {
                                        asset_code: AssetCode4(*b"USDC"),
                                        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(
                                            Uint256([0x22; 32]),
                                        )),
                                    }),
                                    amount: 1_000_000_007,
                                }),
                            },
                            Operation {
                                source_account: Some(MuxedAccount::Ed25519(Uint256([0x66; 32]))),
                                body: OperationBody::ManageData(ManageDataOp {
                                    data_name: String64(StringM::try_from_str_or_panic("config")),
                                    data_value: Some(DataValue(BytesM::try_from_slice_or_panic(
                                        b"value",
                                    ))),
                                }),
                            },
                            Operation {
                                source_account: None,
                                body: OperationBody::Inflation,
                            },
                            Operation {
                                source_account: None,
                                body: OperationBody::CreateClaimableBalance(
                                    CreateClaimableBalanceOp {
                                        asset: Asset::Native,
                                        amount: 250_000_000,
                                        claimants: VecM::try_from_slice_or_panic(
                                            &const {
                                                [Claimant::ClaimantTypeV0(ClaimantV0 {
                                                    destination: AccountId(
                                                        PublicKey::PublicKeyTypeEd25519(Uint256(
                                                            [0x22; 32],
                                                        )),
                                                    ),
                                                    predicate: ClaimPredicate::Not(Some(
                                                        &ClaimPredicate::BeforeRelativeTime(86_400),
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
                ext: TransactionExt::V1(SorobanTransactionData {
                    ext: SorobanTransactionDataExt::V0,
                    resources: SorobanResources {
                        footprint: LedgerFootprint {
                            read_only: VecM::try_from_slice_or_panic(&[LedgerKey::Account(
                                LedgerKeyAccount {
                                    account_id: AccountId(PublicKey::PublicKeyTypeEd25519(
                                        Uint256([0x22; 32]),
                                    )),
                                },
                            )]),
                            read_write: VecM::try_from_slice_or_panic(&[]),
                        },
                        instructions: 1_000_000,
                        disk_read_bytes: 2_048,
                        write_bytes: 512,
                    },
                    resource_fee: 100_000,
                }),
            },
            signatures: VecM::try_from_slice_or_panic(
                &const {
                    [
                        DecoratedSignature {
                            hint: SignatureHint([1, 2, 3, 4]),
                            signature: Signature(BytesM::try_from_slice_or_panic(b"sig-one")),
                        },
                        DecoratedSignature {
                            hint: SignatureHint([5, 6, 7, 8]),
                            signature: Signature(BytesM::try_from_slice_or_panic(b"sig-two")),
                        },
                    ]
                },
            ),
        })
    }
}

/// The same value owned, built from the generated types.
mod owned_value {
    use stellar_xdr::*;

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
                                issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                    [0x22; 32],
                                ))),
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
}
