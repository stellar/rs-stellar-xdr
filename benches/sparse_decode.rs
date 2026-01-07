//! Benchmarks for sparse XDR decoding.
//!
//! Compares full decoding vs sparse decoding vs seekable sparse decoding
//! for extracting specific fields from XDR data.
//!
//! To run these benchmarks:
//! ```
//! cargo bench --features curr
//! ```
//!
//! To use real ledger data from the AWS Public Blockchain Dataset:
//! 1. Download a ledger close meta file from s3://sdf-stellar-pubnet-public-blockchain-data/ledgers/
//! 2. Place it at benches/fixtures/ledger_close_meta.xdr
//! 3. Run the benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use stellar_xdr::curr::{
    AccountId, CreateAccountOp, LedgerCloseMeta, LedgerCloseMetaTxHashes, Limits, Memo,
    MuxedAccount, Operation, OperationBody, Preconditions, PublicKey, ReadXdr, SeekableReadXdr,
    SequenceNumber, Transaction, TransactionEnvelope, TransactionEnvelopeSparse, TransactionExt,
    TransactionV1Envelope, Uint256, WriteXdr,
};

/// Create a test TransactionEnvelope with the specified number of operations
fn create_test_envelope(num_ops: usize) -> TransactionEnvelope {
    let operations: Vec<Operation> = (0..num_ops)
        .map(|i| Operation {
            source_account: None,
            body: OperationBody::CreateAccount(CreateAccountOp {
                destination: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([i as u8; 32]))),
                starting_balance: 1000 + i as i64,
            }),
        })
        .collect();

    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 100,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::Text("Benchmark test memo".as_bytes().try_into().unwrap()),
            operations: operations.try_into().unwrap(),
            ext: TransactionExt::V0,
        },
        signatures: vec![].try_into().unwrap(),
    })
}

fn benchmark_transaction_envelope(c: &mut Criterion) {
    let mut group = c.benchmark_group("TransactionEnvelope");

    for num_ops in [1, 10, 50, 100] {
        let envelope = create_test_envelope(num_ops);
        let xdr_data = envelope.to_xdr(Limits::none()).unwrap();
        let size = xdr_data.len();

        group.throughput(Throughput::Bytes(size as u64));

        // Benchmark full decode
        group.bench_with_input(
            BenchmarkId::new("full_decode", num_ops),
            &xdr_data,
            |b, data| {
                b.iter(|| {
                    let full = TransactionEnvelope::from_xdr(black_box(data), Limits::none())
                        .expect("decode");
                    let ops = match &full {
                        TransactionEnvelope::Tx(e) => &e.tx.operations,
                        _ => panic!("expected Tx variant"),
                    };
                    black_box(ops.len())
                })
            },
        );

        // Benchmark sparse decode (using Read to skip)
        group.bench_with_input(
            BenchmarkId::new("sparse_decode", num_ops),
            &xdr_data,
            |b, data| {
                b.iter(|| {
                    let sparse =
                        TransactionEnvelopeSparse::from_xdr(black_box(data), Limits::none())
                            .expect("decode");
                    let ops = match &sparse {
                        TransactionEnvelopeSparse::Tx(e) => &e.tx.operations,
                        _ => panic!("expected Tx variant"),
                    };
                    black_box(ops.len())
                })
            },
        );

        // Benchmark seekable sparse decode (using Seek to skip)
        group.bench_with_input(
            BenchmarkId::new("seekable_sparse_decode", num_ops),
            &xdr_data,
            |b, data| {
                b.iter(|| {
                    let sparse = TransactionEnvelopeSparse::from_xdr_seekable(
                        black_box(data),
                        Limits::none(),
                    )
                    .expect("decode");
                    let ops = match &sparse {
                        TransactionEnvelopeSparse::Tx(e) => &e.tx.operations,
                        _ => panic!("expected Tx variant"),
                    };
                    black_box(ops.len())
                })
            },
        );
    }

    group.finish();
}

/// Benchmark LedgerCloseMeta decoding using real or synthetic data.
///
/// For real data, download from the AWS Public Blockchain Dataset:
/// https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet/
fn benchmark_ledger_close_meta(c: &mut Criterion) {
    // Try to load real ledger data, or skip this benchmark if not available
    let fixture_path = std::path::Path::new("benches/fixtures/ledger_close_meta.xdr");

    if !fixture_path.exists() {
        eprintln!(
            "Skipping LedgerCloseMeta benchmarks: {} not found",
            fixture_path.display()
        );
        eprintln!("To enable, download a ledger file from AWS Public Blockchain Dataset:");
        eprintln!("  curl -o ledger.xdr.zstd 'https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet/ledger-NNNNNN.xdr.zstd'");
        eprintln!("  zstd -d ledger.xdr.zstd -o benches/fixtures/ledger_close_meta.xdr");
        return;
    }

    let xdr_data = std::fs::read(fixture_path).expect("read fixture");
    let size = xdr_data.len();

    let mut group = c.benchmark_group("LedgerCloseMeta");
    group.throughput(Throughput::Bytes(size as u64));

    // Benchmark full decode and extract tx hashes
    group.bench_function("full_decode_tx_hashes", |b| {
        b.iter(|| {
            let meta =
                LedgerCloseMeta::from_xdr(black_box(&xdr_data), Limits::none()).expect("decode");
            let hashes: Vec<_> = match &meta {
                LedgerCloseMeta::V0(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMeta::V1(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMeta::V2(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
            };
            black_box(hashes.len())
        })
    });

    // Benchmark sparse decode for tx hashes
    group.bench_function("sparse_decode_tx_hashes", |b| {
        b.iter(|| {
            let sparse = LedgerCloseMetaTxHashes::from_xdr(black_box(&xdr_data), Limits::none())
                .expect("decode");
            let hashes: Vec<_> = match &sparse {
                LedgerCloseMetaTxHashes::V0(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMetaTxHashes::V1(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMetaTxHashes::V2(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
            };
            black_box(hashes.len())
        })
    });

    // Benchmark seekable sparse decode for tx hashes
    group.bench_function("seekable_sparse_decode_tx_hashes", |b| {
        b.iter(|| {
            let sparse =
                LedgerCloseMetaTxHashes::from_xdr_seekable(black_box(&xdr_data), Limits::none())
                    .expect("decode");
            let hashes: Vec<_> = match &sparse {
                LedgerCloseMetaTxHashes::V0(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMetaTxHashes::V1(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
                LedgerCloseMetaTxHashes::V2(v) => v
                    .tx_processing
                    .iter()
                    .map(|t| t.result.transaction_hash.clone())
                    .collect(),
            };
            black_box(hashes.len())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_transaction_envelope,
    benchmark_ledger_close_meta
);
criterion_main!(benches);
