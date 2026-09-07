//! Tests for the borrowing `View` types.

use stellar_xdr::{BytesMView, ErrorLengthExceedsMax, StringMView, VecMView};

#[cfg(feature = "alloc")]
use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, ClaimPredicate, ClaimPredicateView, Claimant,
    ClaimantV0, ClaimantV0View, ClaimantView, CreateClaimableBalanceOp,
    CreateClaimableBalanceOpView, DataValue, DataValueView, DecoratedSignature,
    DecoratedSignatureView, Duration, LedgerFootprint, LedgerFootprintView, LedgerKey,
    LedgerKeyAccount, LedgerKeyView, ManageDataOp, ManageDataOpView, Memo, MemoView, MuxedAccount,
    MuxedAccountMed25519, Operation, OperationBody, OperationBodyView, OperationView, PaymentOp,
    Preconditions, PreconditionsV2, PreconditionsV2View, PreconditionsView, PublicKey,
    SequenceNumber, Signature, SignatureHint, SignatureView, SignerKey,
    SignerKeyEd25519SignedPayload, SignerKeyEd25519SignedPayloadView, SignerKeyView,
    SorobanResources, SorobanResourcesView, SorobanTransactionData, SorobanTransactionDataExt,
    SorobanTransactionDataExtView, SorobanTransactionDataView, String64, String64View, TimeBounds,
    TimePoint, Transaction, TransactionEnvelope, TransactionEnvelopeView, TransactionExt,
    TransactionExtView, TransactionV1Envelope, TransactionV1EnvelopeView, TransactionView, Uint256,
};

#[cfg(feature = "std")]
use stellar_xdr::{Limits, WriteXdr};
#[test]
fn views_try_from_enforce_max_len() {
    assert!(VecMView::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
    assert_eq!(
        VecMView::<u32, 2>::try_from_slice(&[1, 2, 3]),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesMView::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesMView::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringMView::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringMView::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    assert!(StringMView::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        StringMView::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );
}

#[test]
fn views_try_from_or_panic() {
    assert_eq!(
        VecMView::<u32, 3>::try_from_slice_or_panic(&[1, 2, 3]),
        VecMView::<u32, 3>::try_from_slice(&[1, 2, 3]).unwrap()
    );
    assert_eq!(
        BytesMView::<3>::try_from_slice_or_panic(b"abc"),
        BytesMView::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringMView::<3>::try_from_str_or_panic("abc"),
        StringMView::<3>::try_from_str("abc").unwrap()
    );
    assert_eq!(
        StringMView::<3>::try_from_slice_or_panic(b"abc"),
        StringMView::<3>::try_from_slice(b"abc").unwrap()
    );
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_view_try_from_or_panic_panics_enforce_max_len() {
    let _ = VecMView::<u32, 2>::try_from_slice_or_panic(&[1, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_view_try_from_or_panic_panics_enforce_max_len() {
    let _ = BytesMView::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_view_try_from_or_panic_panics_enforce_max_len() {
    let _ = StringMView::<2>::try_from_str_or_panic("abc");
}

#[test]
fn views_default() {
    assert_eq!(
        VecMView::<u32, 3>::default(),
        VecMView::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        BytesMView::<3>::default(),
        BytesMView::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        StringMView::<3>::default(),
        StringMView::try_from_str("").unwrap()
    );
}

#[cfg(feature = "alloc")]
#[test]
fn view_converts_to_owned() {
    // A view and owned value that are identically defined.
    let view: TransactionEnvelopeView = const { view() };
    let owned: TransactionEnvelope = owned();

    assert_eq!(TransactionEnvelope::from(&view), owned);
}

#[cfg(feature = "std")]
#[test]
fn view_and_owned_encode_same() {
    // A view and owned value that are identically defined.
    let view: TransactionEnvelopeView = const { view() };
    let owned: TransactionEnvelope = owned();

    // View and owned encode to the same XDR.
    let view_xdr = view.to_xdr(Limits::none()).unwrap();
    let owned_xdr = owned.to_xdr(Limits::none()).unwrap();
    assert_eq!(view_xdr, owned_xdr);
}

#[cfg(feature = "alloc")]
#[allow(clippy::too_many_lines)]
const fn view() -> TransactionEnvelopeView<'static> {
    TransactionEnvelopeView::Tx(TransactionV1EnvelopeView {
        tx: TransactionView {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 0xdead_beef_0000_0001,
                ed25519: Uint256([0x11; 32]),
            }),
            fee: 4_321,
            seq_num: SequenceNumber(9_876_543_210),
            cond: PreconditionsView::V2(PreconditionsV2View {
                time_bounds: Some(TimeBounds {
                    min_time: TimePoint(100),
                    max_time: TimePoint(200),
                }),
                ledger_bounds: None,
                min_seq_num: Some(SequenceNumber(42)),
                min_seq_age: Duration(3_600),
                min_seq_ledger_gap: 7,
                extra_signers: VecMView::try_from_slice_or_panic(
                    &const {
                        [
                            SignerKeyView::Ed25519(Uint256([0x33; 32])),
                            SignerKeyView::Ed25519SignedPayload(
                                SignerKeyEd25519SignedPayloadView {
                                    ed25519: Uint256([0x44; 32]),
                                    payload: BytesMView::try_from_slice_or_panic(b"signed payload"),
                                },
                            ),
                        ]
                    },
                ),
            }),
            memo: MemoView::Text(StringMView::try_from_str_or_panic("hello world")),
            operations: VecMView::try_from_slice_or_panic(
                &const {
                    [
                        OperationView {
                            source_account: None,
                            body: OperationBodyView::Payment(PaymentOp {
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
                        OperationView {
                            source_account: Some(MuxedAccount::Ed25519(Uint256([0x66; 32]))),
                            body: OperationBodyView::ManageData(ManageDataOpView {
                                data_name: String64View(StringMView::try_from_str_or_panic(
                                    "config",
                                )),
                                data_value: Some(DataValueView(
                                    BytesMView::try_from_slice_or_panic(b"value"),
                                )),
                            }),
                        },
                        OperationView {
                            source_account: None,
                            body: OperationBodyView::Inflation,
                        },
                        OperationView {
                            source_account: None,
                            body: OperationBodyView::CreateClaimableBalance(
                                CreateClaimableBalanceOpView {
                                    asset: Asset::Native,
                                    amount: 250_000_000,
                                    claimants: VecMView::try_from_slice_or_panic(
                                        &const {
                                            [ClaimantView::ClaimantTypeV0(ClaimantV0View {
                                                destination: AccountId(
                                                    PublicKey::PublicKeyTypeEd25519(Uint256(
                                                        [0x22; 32],
                                                    )),
                                                ),
                                                predicate: ClaimPredicateView::Not(Some(
                                                    &ClaimPredicateView::BeforeRelativeTime(86_400),
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
            ext: TransactionExtView::V1(SorobanTransactionDataView {
                ext: SorobanTransactionDataExtView::V0,
                resources: SorobanResourcesView {
                    footprint: LedgerFootprintView {
                        read_only: VecMView::try_from_slice_or_panic(&[LedgerKeyView::Account(
                            LedgerKeyAccount {
                                account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                                    [0x22; 32],
                                ))),
                            },
                        )]),
                        read_write: VecMView::try_from_slice_or_panic(&[]),
                    },
                    instructions: 1_000_000,
                    disk_read_bytes: 2_048,
                    write_bytes: 512,
                },
                resource_fee: 100_000,
            }),
        },
        signatures: VecMView::try_from_slice_or_panic(
            &const {
                [
                    DecoratedSignatureView {
                        hint: SignatureHint([1, 2, 3, 4]),
                        signature: SignatureView(BytesMView::try_from_slice_or_panic(b"sig-one")),
                    },
                    DecoratedSignatureView {
                        hint: SignatureHint([5, 6, 7, 8]),
                        signature: SignatureView(BytesMView::try_from_slice_or_panic(b"sig-two")),
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
