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
    AccountId, CreateAccountOp, LedgerCloseMeta, LedgerCloseMetaBatch, LedgerCloseMetaTxHashes,
    Limits, Memo, MuxedAccount, Operation, OperationBody, Preconditions, PublicKey, ReadXdr,
    SeekableReadXdr, SequenceNumber, Transaction, TransactionEnvelope, TransactionEnvelopeSparse,
    TransactionExt, TransactionV1Envelope, Uint256, WriteXdr,
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
///
/// The files are in Frame<LedgerCloseMeta> format with zstd compression.
fn benchmark_ledger_close_meta(c: &mut Criterion) {
    // Try to load real ledger data, or skip this benchmark if not available
    // Look for any .xdr file in the fixtures directory
    let fixtures_dir = std::path::Path::new("benches/fixtures");
    let fixture_path = std::fs::read_dir(fixtures_dir).ok().and_then(|entries| {
        entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| p.extension().is_some_and(|ext| ext == "xdr"))
    });

    let Some(fixture_path) = fixture_path else {
        eprintln!("Skipping LedgerCloseMeta benchmarks: no .xdr file found in benches/fixtures/");
        eprintln!("To enable, download a ledger file from AWS Public Blockchain Dataset:");
        eprintln!("  curl -o ledger.xdr.zst 'https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet/FC6331FF--60608000-60671999/FC62697B--60659332.xdr.zst'");
        eprintln!("  zstd -d ledger.xdr.zst -o benches/fixtures/FC62697B--60659332.xdr");
        return;
    };

    eprintln!("Using fixture: {}", fixture_path.display());

    let xdr_data = std::fs::read(&fixture_path).expect("read fixture");

    // The files from AWS are LedgerCloseMetaBatch format
    let batch = match LedgerCloseMetaBatch::from_xdr(&xdr_data, Limits::none()) {
        Ok(batch) => batch,
        Err(e) => {
            eprintln!(
                "Warning: Could not decode LedgerCloseMetaBatch from fixture: {:?}",
                e
            );
            eprintln!("This may be due to XDR version mismatch with the data file");
            return;
        }
    };

    // Extract the first LedgerCloseMeta and re-encode it for benchmarking
    let Some(first_meta) = batch.ledger_close_metas.first() else {
        eprintln!("Warning: LedgerCloseMetaBatch contains no ledger metas");
        return;
    };

    let ledger_meta_xdr = first_meta.to_xdr(Limits::none()).expect("re-encode");
    let size = ledger_meta_xdr.len();

    let mut group = c.benchmark_group("LedgerCloseMeta");
    group.throughput(Throughput::Bytes(size as u64));

    // Benchmark full decode and extract tx hashes
    group.bench_function("full_decode_tx_hashes", |b| {
        b.iter(|| {
            let meta = LedgerCloseMeta::from_xdr(black_box(&ledger_meta_xdr), Limits::none())
                .expect("decode");
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
            let sparse =
                LedgerCloseMetaTxHashes::from_xdr(black_box(&ledger_meta_xdr), Limits::none())
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
            let sparse = LedgerCloseMetaTxHashes::from_xdr_seekable(
                black_box(&ledger_meta_xdr),
                Limits::none(),
            )
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

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark_transaction_envelope, benchmark_ledger_close_meta
}
criterion_main!(benches);
