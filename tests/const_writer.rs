#![cfg(all(feature = "const", feature = "std"))]

//! Tests for the const XDR serializers, which live as `write_type_{type}`
//! methods on `ConstWriter` rather than on the generated types. Each type keeps
//! a thin `const_xdr_len`/`const_to_xdr` pair that wraps its writer method. A
//! type that owns heap data is written through its borrowing `Ref` form; one
//! that owns none is written directly. Output is compared against the owned
//! type's `to_xdr` (needs `std`).

use stellar_xdr::{
    AccountId, AlphaNum4, Asset, AssetCode4, BytesMRef, ConstWriter, CreateAccountOp, DataValueRef,
    DecoratedSignatureRef, Duration, ExtensionPoint, GeneralizedTransactionSetRef, Hash,
    LedgerBounds, LedgerCloseMeta, LedgerCloseMetaExt, LedgerCloseMetaExtV1, LedgerCloseMetaRef,
    LedgerCloseMetaV2Ref, LedgerCloseValueSignatureRef, LedgerEntryChangesRef, LedgerFootprintRef,
    LedgerHeaderExt, LedgerHeaderHistoryEntryExt, LedgerHeaderHistoryEntryRef, LedgerHeaderRef,
    LedgerKeyContractCode, LedgerKeyRef, LedgerKeyTtl, LedgerScpMessagesRef, LedgerUpgrade, Limits,
    ManageDataOpRef, ManageSellOfferOp, Memo, MemoRef, MemoType, MuxedAccount,
    MuxedAccountMed25519, NodeId, OperationBodyRef, OperationRef, PathPaymentStrictReceiveOpRef,
    PaymentOp, PreconditionsRef, PreconditionsV2Ref, Price, PublicKey, ReadXdr, ScpHistoryEntryRef,
    ScpHistoryEntryV0Ref, ScpQuorumSetRef, SequenceNumber, SetOptionsOpRef, SignatureHint,
    SignatureRef, SignerKeyEd25519SignedPayloadRef, SignerKeyRef, SignerRef,
    SorobanResourcesExtV0Ref, SorobanResourcesRef, SorobanTransactionDataExtRef,
    SorobanTransactionDataRef, StellarValueExtRef, StellarValueRef, String32Ref, String64Ref,
    StringMRef, TimeBounds, TimePoint, TransactionEnvelope, TransactionEnvelopeRef,
    TransactionExtRef, TransactionMetaRef, TransactionPhaseRef, TransactionRef,
    TransactionResultExt, TransactionResultMetaV1Ref, TransactionResultPairRef,
    TransactionResultRef, TransactionResultResultRef, TransactionSetV1Ref,
    TransactionV1EnvelopeRef, TxDemandVector, TxDemandVectorRef, Uint256, UpgradeEntryMetaRef,
    UpgradeTypeRef, VecMRef, WriteXdr,
};

/// Serialize with `f`, measuring into an empty buffer and then filling a buffer
/// of exactly that size: the two passes a caller such as a proc-macro makes,
/// since the encoded length is only known by encoding.
mod common;

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
fn writes_ref_union_arms() {
    // Union arms exercised through the borrowing MemoRef: void, length-prefixed
    // and padded string, scalar, and a fixed-opaque newtype.
    let cases: [MemoRef; 4] = [
        MemoRef::None,
        MemoRef::Text(StringMRef::try_from_str("hello").unwrap()),
        MemoRef::Id(42),
        MemoRef::Hash(Hash([9u8; 32])),
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
fn writes_ref_var_array() {
    // A newtype over `VecM<Hash>`, written through its borrowing Ref form,
    // built from a slice of fixed-size arrays.
    let hashes = [Hash([1u8; 32]), Hash([2u8; 32]), Hash([3u8; 32])];
    let v = TxDemandVectorRef(VecMRef::try_from_slice(&hashes).unwrap());
    let owned: TxDemandVector = (&v).into();
    let bytes = to_xdr_via(|w| w.write_type_tx_demand_vector(&v));
    // 4-byte length prefix + 3 * 32 bytes = 100.
    assert_eq!(bytes.len(), 100);
    assert_eq!(bytes, owned.to_xdr(Limits::none()).unwrap());
}

#[test]
fn bytes_round_trip() {
    let m = MemoRef::Text(StringMRef::try_from_str("round trip").unwrap());
    let owned: Memo = (&m).into();
    let bytes = to_xdr_via(|w| w.write_type_memo(&m));
    assert_eq!(Memo::from_xdr(&bytes, Limits::none()).unwrap(), owned);
}

// Compile-time serialization to a fixed array, the way a proc-macro would emit
// it: size the array with `const_xdr_len`, then fill it with `const_to_xdr`.
// Both wrap a `ConstWriter` around the type's `write_type_*` method. An owned
// non-heap type and a borrowing Ref type are both serialized entirely in const
// contexts.
const TB: TimeBounds = TimeBounds {
    min_time: TimePoint(1),
    max_time: TimePoint(0x0102_0304_0506_0708),
};
const TB_LEN: usize = TB.const_xdr_len();
const TB_XDR: [u8; TB_LEN] = TB.const_to_xdr::<TB_LEN>();

// `StringMRef::try_from_str` returns a `Result`, but its error is the
// drop-free `ErrorLengthExceedsMax`, so it can be unwrapped with `if let` in a
// const context (`Result::unwrap` is not const).
const MEMO: MemoRef = MemoRef::Text(if let Ok(v) = StringMRef::try_from_str("hi") {
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
    let m = MemoRef::Text(StringMRef::try_from_str("hello").unwrap());
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
    ($write:ident, $r:expr, $owned:ty) => {{
        let r = $r;
        let owned: $owned = (&r).into();
        let streamed = owned.to_xdr(Limits::none()).unwrap();
        let bytes = to_xdr_via(|w| w.$write(&r));
        assert_eq!(bytes, streamed, "const bytes != streamed bytes");
        assert_eq!(
            <$owned>::from_xdr(&bytes, Limits::none()).unwrap(),
            owned,
            "const bytes did not round-trip back to the owned value"
        );
    }};
}

/// Build a deeply-populated `TransactionEnvelopeRef` exercising as many fields
/// and operation arms as possible, and check its `const` serialization matches
/// the owned type's streaming `write_xdr`.
#[test]
#[allow(clippy::too_many_lines)]
fn const_transaction_envelope_matches_stream() {
    // Borrowed leaf data, declared first so the Ref can reference it.
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
        OperationRef {
            source_account: Some(MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 7,
                ed25519: Uint256([4u8; 32]),
            })),
            body: OperationBodyRef::CreateAccount(CreateAccountOp {
                destination: account(),
                starting_balance: 100_000,
            }),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::Payment(PaymentOp {
                destination: MuxedAccount::Ed25519(Uint256([5u8; 32])),
                asset: Asset::Native,
                amount: 42,
            }),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::ManageSellOffer(ManageSellOfferOp {
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
        OperationRef {
            source_account: None,
            body: OperationBodyRef::PathPaymentStrictReceive(PathPaymentStrictReceiveOpRef {
                send_asset: Asset::Native,
                send_max: 10,
                destination: MuxedAccount::Ed25519(Uint256([6u8; 32])),
                dest_asset: Asset::Native,
                dest_amount: 5,
                path: VecMRef::try_from_slice(&path_assets).unwrap(),
            }),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::SetOptions(SetOptionsOpRef {
                inflation_dest: Some(account()),
                clear_flags: Some(1),
                set_flags: Some(2),
                master_weight: Some(3),
                low_threshold: Some(4),
                med_threshold: Some(5),
                high_threshold: Some(6),
                home_domain: Some(String32Ref(
                    StringMRef::try_from_str("example.com").unwrap(),
                )),
                signer: Some(SignerRef {
                    key: SignerKeyRef::Ed25519(Uint256([8u8; 32])),
                    weight: 1,
                }),
            }),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::ManageData(ManageDataOpRef {
                data_name: String64Ref(StringMRef::try_from_str("data key").unwrap()),
                data_value: Some(DataValueRef(
                    BytesMRef::try_from_slice(&data_value_bytes).unwrap(),
                )),
            }),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::AccountMerge(MuxedAccount::Ed25519(Uint256([1u8; 32]))),
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::Inflation,
        },
        OperationRef {
            source_account: None,
            body: OperationBodyRef::EndSponsoringFutureReserves,
        },
    ];

    let extra_signers = [SignerKeyRef::Ed25519SignedPayload(
        SignerKeyEd25519SignedPayloadRef {
            ed25519: Uint256([9u8; 32]),
            payload: BytesMRef::try_from_slice(&signed_payload).unwrap(),
        },
    )];

    let signatures = [DecoratedSignatureRef {
        hint: SignatureHint([1, 2, 3, 4]),
        signature: SignatureRef(BytesMRef::try_from_slice(&sig_bytes).unwrap()),
    }];

    let r = TransactionEnvelopeRef::Tx(TransactionV1EnvelopeRef {
        tx: TransactionRef {
            source_account: MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
                id: 99,
                ed25519: Uint256([0u8; 32]),
            }),
            fee: 1234,
            seq_num: SequenceNumber(42),
            cond: PreconditionsRef::V2(PreconditionsV2Ref {
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
                extra_signers: VecMRef::try_from_slice(&extra_signers).unwrap(),
            }),
            memo: MemoRef::Text(StringMRef::try_from_str("hello memo").unwrap()),
            operations: VecMRef::try_from_slice(&operations).unwrap(),
            ext: TransactionExtRef::V1(SorobanTransactionDataRef {
                ext: SorobanTransactionDataExtRef::V1(SorobanResourcesExtV0Ref {
                    archived_soroban_entries: VecMRef::try_from_slice(&archived).unwrap(),
                }),
                resources: SorobanResourcesRef {
                    footprint: LedgerFootprintRef {
                        read_only: VecMRef::default(),
                        read_write: VecMRef::default(),
                    },
                    instructions: 100,
                    disk_read_bytes: 200,
                    write_bytes: 300,
                },
                resource_fee: 999,
            }),
        },
        signatures: VecMRef::try_from_slice(&signatures).unwrap(),
    });

    assert_const_matches_stream!(write_type_transaction_envelope, r, TransactionEnvelope);
}

/// The same envelope as the `Ref`/owned encoding test in `ref_types`, written
/// through the const writer instead. The oracle here is the hand-built owned
/// value from the shared fixture, not one derived from the `Ref`, so a bug in
/// the `Ref` -> owned conversion cannot hide the way it can in
/// `const_transaction_envelope_matches_stream` above, which builds its owned
/// value with `(&r).into()`.
#[test]
fn const_transaction_envelope_matches_hand_built_owned() {
    let r = const { common::tx_env_ref() };
    let owned = common::tx_env_owned();

    let bytes = to_xdr_via(|w| w.write_type_transaction_envelope(&r));
    assert_eq!(
        bytes,
        owned.to_xdr(Limits::none()).unwrap(),
        "const bytes != the owned value's bytes"
    );
    assert_eq!(
        TransactionEnvelope::from_xdr(&bytes, Limits::none()).unwrap(),
        owned,
        "const bytes did not round-trip back to the owned value"
    );
}

/// Build a deeply-populated `LedgerCloseMetaRef` (the richest `V2` variant),
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
        UpgradeTypeRef(BytesMRef::try_from_slice(&upgrade_a).unwrap()),
        UpgradeTypeRef(BytesMRef::try_from_slice(&upgrade_b).unwrap()),
    ];

    let ledger_header = LedgerHeaderHistoryEntryRef {
        hash: Hash([1u8; 32]),
        header: LedgerHeaderRef {
            ledger_version: 21,
            previous_ledger_hash: Hash([2u8; 32]),
            scp_value: StellarValueRef {
                tx_set_hash: Hash([3u8; 32]),
                close_time: TimePoint(1_700_000_000),
                upgrades: VecMRef::try_from_slice(&upgrades).unwrap(),
                ext: StellarValueExtRef::Signed(LedgerCloseValueSignatureRef {
                    node_id: node(),
                    signature: SignatureRef(BytesMRef::try_from_slice(&scp_sig).unwrap()),
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
    let phases = [TransactionPhaseRef::V0(VecMRef::default())];
    let tx_set = GeneralizedTransactionSetRef::V1(TransactionSetV1Ref {
        previous_ledger_hash: Hash([10u8; 32]),
        phases: VecMRef::try_from_slice(&phases).unwrap(),
    });

    // One tx-processing entry; the deep meta/change sub-trees are left empty.
    let tx_processing = [TransactionResultMetaV1Ref {
        ext: ExtensionPoint::V0,
        result: TransactionResultPairRef {
            transaction_hash: Hash([11u8; 32]),
            result: TransactionResultRef {
                fee_charged: 100,
                result: TransactionResultResultRef::TxSuccess(VecMRef::default()),
                ext: TransactionResultExt::V0,
            },
        },
        fee_processing: LedgerEntryChangesRef(VecMRef::default()),
        tx_apply_processing: TransactionMetaRef::V0(VecMRef::default()),
        post_tx_apply_fee_processing: LedgerEntryChangesRef(VecMRef::default()),
    }];

    let upgrades_processing = [UpgradeEntryMetaRef {
        upgrade: LedgerUpgrade::Version(21),
        changes: LedgerEntryChangesRef(VecMRef::default()),
    }];

    // One SCP history entry with a quorum set.
    let validators = [node()];
    let quorum_sets = [ScpQuorumSetRef {
        threshold: 1,
        validators: VecMRef::try_from_slice(&validators).unwrap(),
        inner_sets: VecMRef::default(),
    }];
    let scp_info = [ScpHistoryEntryRef::V0(ScpHistoryEntryV0Ref {
        quorum_sets: VecMRef::try_from_slice(&quorum_sets).unwrap(),
        ledger_messages: LedgerScpMessagesRef {
            ledger_seq: 1000,
            messages: VecMRef::default(),
        },
    })];

    // A couple of simple owned-payload ledger-key arms.
    let evicted_keys = [
        LedgerKeyRef::Ttl(LedgerKeyTtl {
            key_hash: Hash([12u8; 32]),
        }),
        LedgerKeyRef::ContractCode(LedgerKeyContractCode {
            hash: Hash([13u8; 32]),
        }),
    ];

    let r = LedgerCloseMetaRef::V2(LedgerCloseMetaV2Ref {
        ext: LedgerCloseMetaExt::V1(LedgerCloseMetaExtV1 {
            ext: ExtensionPoint::V0,
            soroban_fee_write1_kb: 1234,
        }),
        ledger_header,
        tx_set,
        tx_processing: VecMRef::try_from_slice(&tx_processing).unwrap(),
        upgrades_processing: VecMRef::try_from_slice(&upgrades_processing).unwrap(),
        scp_info: VecMRef::try_from_slice(&scp_info).unwrap(),
        total_byte_size_of_live_soroban_state: 9_999,
        evicted_keys: VecMRef::try_from_slice(&evicted_keys).unwrap(),
    });

    assert_const_matches_stream!(write_type_ledger_close_meta, r, LedgerCloseMeta);
}
