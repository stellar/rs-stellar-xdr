#![cfg(all(feature = "const", feature = "std"))]

//! Tests for the const XDR serializers, which live as `write_type_{type}`
//! methods on `ConstWriter` rather than on the generated types. Each type keeps
//! a thin `const_xdr_len`/`const_to_xdr` pair that wraps its writer method. A
//! type that owns heap data is written through its borrowing `View` form; one
//! that owns none is written directly. Output is compared against the owned
//! type's `to_xdr` (needs `std`).

use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, BytesMView, ConstWriter, CreateAccountOp,
    DataValueView, DecoratedSignatureView, Duration, ExtensionPoint, GeneralizedTransactionSetView,
    Hash, LedgerBounds, LedgerCloseMeta, LedgerCloseMetaExt, LedgerCloseMetaExtV1,
    LedgerCloseMetaV2View, LedgerCloseMetaView, LedgerCloseValueSignatureView,
    LedgerEntryChangesView, LedgerFootprintView, LedgerHeaderExt, LedgerHeaderHistoryEntryExt,
    LedgerHeaderHistoryEntryView, LedgerHeaderView, LedgerKeyContractCode, LedgerKeyTtl,
    LedgerKeyView, LedgerScpMessagesView, LedgerUpgrade, Limits, ManageDataOpView,
    ManageSellOfferOp, Memo, MemoType, MemoView, MuxedAccount, MuxedAccountMed25519, NodeId,
    OperationBodyView, OperationView, PathPaymentStrictReceiveOpView, PaymentOp,
    PreconditionsV2View, PreconditionsView, Price, PublicKey, ReadXdr, ScpHistoryEntryV0View,
    ScpHistoryEntryView, ScpQuorumSetView, SequenceNumber, SetOptionsOpView, SignatureHint,
    SignatureView, SignerKeyEd25519SignedPayloadView, SignerKeyView, SignerView,
    SorobanResourcesExtV0View, SorobanResourcesView, SorobanTransactionDataExtView,
    SorobanTransactionDataView, StellarValueExtView, StellarValueView, String32View, String64View,
    StringMView, TimeBounds, TimePoint, TransactionEnvelope, TransactionEnvelopeView,
    TransactionExtView, TransactionMetaView, TransactionPhaseView, TransactionResultExt,
    TransactionResultMetaV1View, TransactionResultPairView, TransactionResultResultView,
    TransactionResultView, TransactionSetV1View, TransactionV1EnvelopeView, TransactionView,
    TxDemandVector, TxDemandVectorView, Uint256, UpgradeEntryMetaView, UpgradeTypeView, VecMView,
    WriteXdr,
};

/// Serialize with `f`, measuring into an empty buffer and then filling a buffer
/// of exactly that size: the two passes a caller such as a proc-macro makes,
/// since the encoded length is only known by encoding.
fn to_xdr_via<F: Fn(&mut ConstWriter)>(f: F) -> Vec<u8> {
    let mut empty: [u8; 0] = [];
    let mut w = ConstWriter::new(&mut empty);
    f(&mut w);
    let n = w.len();
    let mut buf = vec![0u8; n];
    let mut w = ConstWriter::new(&mut buf);
    f(&mut w);
    assert_eq!(w.len(), n, "length differed between the two passes");
    buf
}

#[test]
fn writes_owned_non_heap_types() {
    // Fixed-length opaque newtype: 32 bytes, no length prefix.
    let u = Uint256([7u8; 32]);
    assert_eq!(
        to_xdr_via(|w| w.write_type_uint256(&u)),
        u.to_xdr(Limits::none()).unwrap()
    );

    // Struct of newtypes over scalars: 2 * u64 = 16 bytes.
    let tb = TimeBounds {
        min_time: TimePoint(1),
        max_time: TimePoint(u64::MAX),
    };
    assert_eq!(to_xdr_via(|w| w.write_type_time_bounds(&tb)).len(), 16);
    assert_eq!(
        to_xdr_via(|w| w.write_type_time_bounds(&tb)),
        tb.to_xdr(Limits::none()).unwrap()
    );

    // Enum: 4 bytes.
    assert_eq!(
        to_xdr_via(|w| w.write_type_memo_type(&MemoType::Id)),
        MemoType::Id.to_xdr(Limits::none()).unwrap()
    );
}

#[test]
fn writes_view_union_arms() {
    // Union arms exercised through the borrowing MemoView: void, length-prefixed
    // and padded string, scalar, and a fixed-opaque newtype.
    let cases: [MemoView; 4] = [
        MemoView::None,
        MemoView::Text(StringMView::try_from_str("hello").unwrap()),
        MemoView::Id(42),
        MemoView::Hash(Hash([9u8; 32])),
    ];
    for m in cases {
        let owned: Memo = (&m).into();
        assert_eq!(
            to_xdr_via(|w| w.write_type_memo(&m)),
            owned.to_xdr(Limits::none()).unwrap(),
            "{owned:?}"
        );
    }
}

#[test]
fn writes_view_var_array() {
    // A newtype over `VecM<Hash>`, written through its borrowing View form,
    // built from a slice of fixed-size arrays.
    let hashes = [Hash([1u8; 32]), Hash([2u8; 32]), Hash([3u8; 32])];
    let v = TxDemandVectorView(VecMView::try_from_slice(&hashes).unwrap());
    let owned: TxDemandVector = (&v).into();
    let bytes = to_xdr_via(|w| w.write_type_tx_demand_vector(&v));
    // 4-byte length prefix + 3 * 32 bytes = 100.
    assert_eq!(bytes.len(), 100);
    assert_eq!(bytes, owned.to_xdr(Limits::none()).unwrap());
}

#[test]
fn bytes_round_trip() {
    let m = MemoView::Text(StringMView::try_from_str("round trip").unwrap());
    let owned: Memo = (&m).into();
    let bytes = to_xdr_via(|w| w.write_type_memo(&m));
    assert_eq!(Memo::from_xdr(&bytes, Limits::none()).unwrap(), owned);
}

// Compile-time serialization to a fixed array, the way a proc-macro would emit
// it: size the array with `const_xdr_len`, then fill it with `const_to_xdr`.
// Both wrap a `ConstWriter` around the type's `write_type_*` method. An owned
// non-heap type and a borrowing View type are both serialized entirely in const
// contexts.
const TB: TimeBounds = TimeBounds {
    min_time: TimePoint(1),
    max_time: TimePoint(0x0102_0304_0506_0708),
};
const TB_LEN: usize = TB.const_xdr_len();
const TB_XDR: [u8; TB_LEN] = TB.const_to_xdr::<TB_LEN>();

// `StringMView::try_from_str` returns a `Result`, but its error is the
// drop-free `ErrorLengthExceedsMax`, so it can be unwrapped with `if let` in a
// const context (`Result::unwrap` is not const).
const MEMO: MemoView = MemoView::Text(if let Ok(v) = StringMView::try_from_str("hi") {
    v
} else {
    panic!()
});
const MEMO_LEN: usize = MEMO.const_xdr_len();
const MEMO_XDR: [u8; MEMO_LEN] = MEMO.const_to_xdr::<MEMO_LEN>();

#[test]
fn wrapper_matches_direct_writer_call() {
    // `const_to_xdr` is only a `ConstWriter` set up around the type's
    // `write_type_*` method, so the two must agree.
    let m = MemoView::Text(StringMView::try_from_str("hello").unwrap());
    assert_eq!(
        m.const_to_xdr::<16>().to_vec(),
        to_xdr_via(|w| w.write_type_memo(&m))
    );
    assert_eq!(
        m.const_xdr_len(),
        to_xdr_via(|w| w.write_type_memo(&m)).len()
    );
}

#[test]
fn const_context() {
    assert_eq!(TB_LEN, 16);
    assert_eq!(TB_XDR.to_vec(), TB.to_xdr(Limits::none()).unwrap());

    // 4 (discriminant) + 4 (len) + 2 ("hi") + 2 (pad) = 12.
    assert_eq!(MEMO_LEN, 12);
    let owned: Memo = (&MEMO).into();
    assert_eq!(MEMO_XDR.to_vec(), owned.to_xdr(Limits::none()).unwrap());
}

/// Assert that a value's const serialization through `$write` is byte-identical
/// to the streaming `write_xdr` of the equivalent owned value, and that the
/// bytes decode back to that owned value.
macro_rules! assert_const_matches_stream {
    ($write:ident, $view:expr, $owned:ty) => {{
        let view = $view;
        let owned: $owned = (&view).into();
        let streamed = owned.to_xdr(Limits::none()).unwrap();
        let bytes = to_xdr_via(|w| w.$write(&view));
        assert_eq!(bytes, streamed, "const bytes != streamed bytes");
        assert_eq!(
            <$owned>::from_xdr(&bytes, Limits::none()).unwrap(),
            owned,
            "const bytes did not round-trip back to the owned value"
        );
    }};
}

/// Build a deeply-populated `TransactionEnvelopeView` exercising as many fields
/// and operation arms as possible, and check its `const` serialization matches
/// the owned type's streaming `write_xdr`.
#[test]
#[allow(clippy::too_many_lines)]
fn const_transaction_envelope_matches_stream() {
    // Borrowed leaf data, declared first so the View can reference it.
    let sig_bytes = [3u8; 32];
    let signed_payload = [7u8; 20];
    let data_value_bytes = [1u8, 2, 3, 4, 5];
    let archived = [10u32, 20, 30];
    let account = || AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([2u8; 32])));
    let path_assets = [
        Asset::Native,
        Asset::CreditAlphanum4(AlphaNum4 {
            asset_code: AssetCode4(*b"ABCD"),
            issuer: account(),
        }),
    ];

    // A mix of void, owned-payload, and borrowing-payload operation arms.
    let operations = [
        OperationView {
            source_account: Some(MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 7,
                ed25519: Uint256([4u8; 32]),
            })),
            body: OperationBodyView::CreateAccount(CreateAccountOp {
                destination: account(),
                starting_balance: 100_000,
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::Payment(PaymentOp {
                destination: MuxedAccount::Ed25519(Uint256([5u8; 32])),
                asset: Asset::Native,
                amount: 42,
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::ManageSellOffer(ManageSellOfferOp {
                selling: Asset::Native,
                buying: Asset::CreditAlphanum4(AlphaNum4 {
                    asset_code: AssetCode4(*b"USD\0"),
                    issuer: account(),
                }),
                amount: 7,
                price: Price { n: 1, d: 2 },
                offer_id: 0,
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::PathPaymentStrictReceive(PathPaymentStrictReceiveOpView {
                send_asset: Asset::Native,
                send_max: 10,
                destination: MuxedAccount::Ed25519(Uint256([6u8; 32])),
                dest_asset: Asset::Native,
                dest_amount: 5,
                path: VecMView::try_from_slice(&path_assets).unwrap(),
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::SetOptions(SetOptionsOpView {
                inflation_dest: Some(account()),
                clear_flags: Some(1),
                set_flags: Some(2),
                master_weight: Some(3),
                low_threshold: Some(4),
                med_threshold: Some(5),
                high_threshold: Some(6),
                home_domain: Some(String32View(
                    StringMView::try_from_str("example.com").unwrap(),
                )),
                signer: Some(SignerView {
                    key: SignerKeyView::Ed25519(Uint256([8u8; 32])),
                    weight: 1,
                }),
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::ManageData(ManageDataOpView {
                data_name: String64View(StringMView::try_from_str("data key").unwrap()),
                data_value: Some(DataValueView(
                    BytesMView::try_from_slice(&data_value_bytes).unwrap(),
                )),
            }),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::AccountMerge(MuxedAccount::Ed25519(Uint256([1u8; 32]))),
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::Inflation,
        },
        OperationView {
            source_account: None,
            body: OperationBodyView::EndSponsoringFutureReserves,
        },
    ];

    let extra_signers = [SignerKeyView::Ed25519SignedPayload(
        SignerKeyEd25519SignedPayloadView {
            ed25519: Uint256([9u8; 32]),
            payload: BytesMView::try_from_slice(&signed_payload).unwrap(),
        },
    )];

    let signatures = [DecoratedSignatureView {
        hint: SignatureHint([1, 2, 3, 4]),
        signature: SignatureView(BytesMView::try_from_slice(&sig_bytes).unwrap()),
    }];

    let view = TransactionEnvelopeView::Tx(TransactionV1EnvelopeView {
        tx: TransactionView {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 99,
                ed25519: Uint256([0u8; 32]),
            }),
            fee: 1234,
            seq_num: SequenceNumber(42),
            cond: PreconditionsView::V2(PreconditionsV2View {
                time_bounds: Some(TimeBounds {
                    min_time: TimePoint(1),
                    max_time: TimePoint(2),
                }),
                ledger_bounds: Some(LedgerBounds {
                    min_ledger: 3,
                    max_ledger: 4,
                }),
                min_seq_num: Some(SequenceNumber(5)),
                min_seq_age: Duration(6),
                min_seq_ledger_gap: 7,
                extra_signers: VecMView::try_from_slice(&extra_signers).unwrap(),
            }),
            memo: MemoView::Text(StringMView::try_from_str("hello memo").unwrap()),
            operations: VecMView::try_from_slice(&operations).unwrap(),
            ext: TransactionExtView::V1(SorobanTransactionDataView {
                ext: SorobanTransactionDataExtView::V1(SorobanResourcesExtV0View {
                    archived_soroban_entries: VecMView::try_from_slice(&archived).unwrap(),
                }),
                resources: SorobanResourcesView {
                    footprint: LedgerFootprintView {
                        read_only: VecMView::default(),
                        read_write: VecMView::default(),
                    },
                    instructions: 100,
                    disk_read_bytes: 200,
                    write_bytes: 300,
                },
                resource_fee: 999,
            }),
        },
        signatures: VecMView::try_from_slice(&signatures).unwrap(),
    });

    assert_const_matches_stream!(write_type_transaction_envelope, view, TransactionEnvelope);
}

/// Build a deeply-populated `LedgerCloseMetaView` (the richest `V2` variant),
/// with the top levels fully populated and the deepest sub-trees left minimal,
/// and check its `const` serialization matches the owned streaming `write_xdr`.
#[test]
fn const_ledger_close_meta_matches_stream() {
    let node = || NodeId(PublicKey::PublicKeyTypeEd25519(Uint256([1u8; 32])));

    // Borrowed leaf data for the ledger header's SCP value.
    let upgrade_a = [1u8, 2, 3];
    let upgrade_b = [4u8, 5];
    let scp_sig = [7u8; 32];
    let upgrades = [
        UpgradeTypeView(BytesMView::try_from_slice(&upgrade_a).unwrap()),
        UpgradeTypeView(BytesMView::try_from_slice(&upgrade_b).unwrap()),
    ];

    let ledger_header = LedgerHeaderHistoryEntryView {
        hash: Hash([1u8; 32]),
        header: LedgerHeaderView {
            ledger_version: 21,
            previous_ledger_hash: Hash([2u8; 32]),
            scp_value: StellarValueView {
                tx_set_hash: Hash([3u8; 32]),
                close_time: TimePoint(1_700_000_000),
                upgrades: VecMView::try_from_slice(&upgrades).unwrap(),
                ext: StellarValueExtView::Signed(LedgerCloseValueSignatureView {
                    node_id: node(),
                    signature: SignatureView(BytesMView::try_from_slice(&scp_sig).unwrap()),
                }),
            },
            tx_set_result_hash: Hash([4u8; 32]),
            bucket_list_hash: Hash([5u8; 32]),
            ledger_seq: 1000,
            total_coins: 1_000_000_000,
            fee_pool: 500,
            inflation_seq: 3,
            id_pool: 42,
            base_fee: 100,
            base_reserve: 5_000_000,
            max_tx_set_size: 1000,
            skip_list: [
                Hash([6u8; 32]),
                Hash([7u8; 32]),
                Hash([8u8; 32]),
                Hash([9u8; 32]),
            ],
            ext: LedgerHeaderExt::V0,
        },
        ext: LedgerHeaderHistoryEntryExt::V0,
    };

    // One transaction set phase (empty component list keeps the tx tree shallow).
    let phases = [TransactionPhaseView::V0(VecMView::default())];
    let tx_set = GeneralizedTransactionSetView::V1(TransactionSetV1View {
        previous_ledger_hash: Hash([10u8; 32]),
        phases: VecMView::try_from_slice(&phases).unwrap(),
    });

    // One tx-processing entry; the deep meta/change sub-trees are left empty.
    let tx_processing = [TransactionResultMetaV1View {
        ext: ExtensionPoint::V0,
        result: TransactionResultPairView {
            transaction_hash: Hash([11u8; 32]),
            result: TransactionResultView {
                fee_charged: 100,
                result: TransactionResultResultView::TxSuccess(VecMView::default()),
                ext: TransactionResultExt::V0,
            },
        },
        fee_processing: LedgerEntryChangesView(VecMView::default()),
        tx_apply_processing: TransactionMetaView::V0(VecMView::default()),
        post_tx_apply_fee_processing: LedgerEntryChangesView(VecMView::default()),
    }];

    let upgrades_processing = [UpgradeEntryMetaView {
        upgrade: LedgerUpgrade::Version(21),
        changes: LedgerEntryChangesView(VecMView::default()),
    }];

    // One SCP history entry with a quorum set.
    let validators = [node()];
    let quorum_sets = [ScpQuorumSetView {
        threshold: 1,
        validators: VecMView::try_from_slice(&validators).unwrap(),
        inner_sets: VecMView::default(),
    }];
    let scp_info = [ScpHistoryEntryView::V0(ScpHistoryEntryV0View {
        quorum_sets: VecMView::try_from_slice(&quorum_sets).unwrap(),
        ledger_messages: LedgerScpMessagesView {
            ledger_seq: 1000,
            messages: VecMView::default(),
        },
    })];

    // A couple of simple owned-payload ledger-key arms.
    let evicted_keys = [
        LedgerKeyView::Ttl(LedgerKeyTtl {
            key_hash: Hash([12u8; 32]),
        }),
        LedgerKeyView::ContractCode(LedgerKeyContractCode {
            hash: Hash([13u8; 32]),
        }),
    ];

    let view = LedgerCloseMetaView::V2(LedgerCloseMetaV2View {
        ext: LedgerCloseMetaExt::V1(LedgerCloseMetaExtV1 {
            ext: ExtensionPoint::V0,
            soroban_fee_write1_kb: 1234,
        }),
        ledger_header,
        tx_set,
        tx_processing: VecMView::try_from_slice(&tx_processing).unwrap(),
        upgrades_processing: VecMView::try_from_slice(&upgrades_processing).unwrap(),
        scp_info: VecMView::try_from_slice(&scp_info).unwrap(),
        total_byte_size_of_live_soroban_state: 9_999,
        evicted_keys: VecMView::try_from_slice(&evicted_keys).unwrap(),
    });

    assert_const_matches_stream!(write_type_ledger_close_meta, view, LedgerCloseMeta);
}
