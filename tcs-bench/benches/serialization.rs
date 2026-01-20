//! Serialization benchmarks: TCS (wincode) vs BCS vs others
//!
//! Run with: cargo bench -p tcs-bench
//!
//! TCS uses wincode encoding with SchemaRead/SchemaWrite derives.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use tcs_bench::generators::*;
use tcs_bench::*;

// ============================================================================
// Serialization Benchmarks
// ============================================================================

fn bench_serialize_block_header(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize/block_header");
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let header = random_block_header(&mut rng);

    // BCS
    group.bench_function("bcs", |b| {
        b.iter(|| bcs::to_bytes(black_box(&header)).unwrap())
    });

    // TCS (wincode)
    group.bench_function("tcs", |b| {
        b.iter(|| wincode::serialize(black_box(&header)).unwrap())
    });

    // Postcard (for comparison - varint encoding)
    group.bench_function("postcard", |b| {
        b.iter(|| postcard::to_allocvec(black_box(&header)).unwrap())
    });

    group.finish();
}

fn bench_deserialize_block_header(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/block_header");
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let header = random_block_header(&mut rng);

    let bcs_bytes = bcs::to_bytes(&header).unwrap();
    let tcs_bytes = wincode::serialize(&header).unwrap();
    let postcard_bytes = postcard::to_allocvec(&header).unwrap();

    // BCS
    group.bench_function("bcs", |b| {
        b.iter(|| bcs::from_bytes::<BlockHeader>(black_box(&bcs_bytes)).unwrap())
    });

    // TCS (wincode)
    group.bench_function("tcs", |b| {
        b.iter(|| wincode::deserialize::<BlockHeader>(black_box(&tcs_bytes)).unwrap())
    });

    // Postcard
    group.bench_function("postcard", |b| {
        b.iter(|| postcard::from_bytes::<BlockHeader>(black_box(&postcard_bytes)).unwrap())
    });

    group.finish();
}

fn bench_serialize_transaction(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize/transaction");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Test with different payload sizes
    for payload_size in [64, 256, 1024, 4096] {
        let tx = random_transaction(&mut rng, payload_size);
        let param = format!("payload_{}", payload_size);

        group.throughput(Throughput::Bytes(payload_size as u64));

        group.bench_with_input(BenchmarkId::new("bcs", &param), &tx, |b, tx| {
            b.iter(|| bcs::to_bytes(black_box(tx)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("tcs", &param), &tx, |b, tx| {
            b.iter(|| wincode::serialize(black_box(tx)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("postcard", &param), &tx, |b, tx| {
            b.iter(|| postcard::to_allocvec(black_box(tx)).unwrap())
        });
    }

    group.finish();
}

fn bench_deserialize_transaction(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/transaction");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for payload_size in [64, 256, 1024, 4096] {
        let tx = random_transaction(&mut rng, payload_size);
        let param = format!("payload_{}", payload_size);

        let bcs_bytes = bcs::to_bytes(&tx).unwrap();
        let tcs_bytes = wincode::serialize(&tx).unwrap();
        let postcard_bytes = postcard::to_allocvec(&tx).unwrap();

        group.throughput(Throughput::Bytes(payload_size as u64));

        group.bench_with_input(BenchmarkId::new("bcs", &param), &bcs_bytes, |b, bytes| {
            b.iter(|| bcs::from_bytes::<Transaction>(black_box(bytes)).unwrap())
        });

        group.bench_with_input(
            BenchmarkId::new("tcs", &param),
            &tcs_bytes,
            |b, bytes| b.iter(|| wincode::deserialize::<Transaction>(black_box(bytes)).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("postcard", &param),
            &postcard_bytes,
            |b, bytes| b.iter(|| postcard::from_bytes::<Transaction>(black_box(bytes)).unwrap()),
        );
    }

    group.finish();
}

fn bench_serialize_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize/slice");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Slices are 1/1024th of a blob - test various sizes
    // Assuming blobs from 1KB to 1MB
    for data_size in [1, 64, 256, 1024] {
        let slice = random_slice(&mut rng, data_size);
        let param = format!("data_{}", data_size);

        group.throughput(Throughput::Bytes(data_size as u64));

        group.bench_with_input(BenchmarkId::new("bcs", &param), &slice, |b, s| {
            b.iter(|| bcs::to_bytes(black_box(s)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("tcs", &param), &slice, |b, s| {
            b.iter(|| wincode::serialize(black_box(s)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("postcard", &param), &slice, |b, s| {
            b.iter(|| postcard::to_allocvec(black_box(s)).unwrap())
        });
    }

    group.finish();
}

fn bench_serialize_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize/transaction_batch");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Test batches of different sizes
    for (tx_count, payload_size) in [(10, 256), (50, 256), (100, 256), (100, 1024)] {
        let batch = random_transaction_batch(&mut rng, tx_count, payload_size);
        let param = format!("{}tx_{}bytes", tx_count, payload_size);

        let total_size = tx_count * (32 + 8 + payload_size + 64 + 4); // approx
        group.throughput(Throughput::Bytes(total_size as u64));

        group.bench_with_input(BenchmarkId::new("bcs", &param), &batch, |b, batch| {
            b.iter(|| bcs::to_bytes(black_box(batch)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("tcs", &param), &batch, |b, batch| {
            b.iter(|| wincode::serialize(black_box(batch)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("postcard", &param), &batch, |b, batch| {
            b.iter(|| postcard::to_allocvec(black_box(batch)).unwrap())
        });
    }

    group.finish();
}

fn bench_deserialize_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/transaction_batch");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for (tx_count, payload_size) in [(10, 256), (50, 256), (100, 256)] {
        let batch = random_transaction_batch(&mut rng, tx_count, payload_size);
        let param = format!("{}tx_{}bytes", tx_count, payload_size);

        let bcs_bytes = bcs::to_bytes(&batch).unwrap();
        let tcs_bytes = wincode::serialize(&batch).unwrap();
        let postcard_bytes = postcard::to_allocvec(&batch).unwrap();

        let total_size = tx_count * (32 + 8 + payload_size + 64 + 4);
        group.throughput(Throughput::Bytes(total_size as u64));

        group.bench_with_input(BenchmarkId::new("bcs", &param), &bcs_bytes, |b, bytes| {
            b.iter(|| bcs::from_bytes::<TransactionBatch>(black_box(bytes)).unwrap())
        });

        group.bench_with_input(
            BenchmarkId::new("tcs", &param),
            &tcs_bytes,
            |b, bytes| {
                b.iter(|| wincode::deserialize::<TransactionBatch>(black_box(bytes)).unwrap())
            },
        );

        group.bench_with_input(
            BenchmarkId::new("postcard", &param),
            &postcard_bytes,
            |b, bytes| {
                b.iter(|| postcard::from_bytes::<TransactionBatch>(black_box(bytes)).unwrap())
            },
        );
    }

    group.finish();
}

// ============================================================================
// Size Comparison (not timed, just for reference)
// ============================================================================

fn bench_size_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("size_comparison");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // BlockHeader
    let header = random_block_header(&mut rng);
    println!("\n=== BlockHeader Serialized Sizes ===");
    println!("  BCS:      {} bytes", bcs::to_bytes(&header).unwrap().len());
    println!(
        "  TCS:  {} bytes",
        wincode::serialize(&header).unwrap().len()
    );
    println!(
        "  Postcard: {} bytes",
        postcard::to_allocvec(&header).unwrap().len()
    );

    // Transaction with 256-byte payload
    let tx = random_transaction(&mut rng, 256);
    println!("\n=== Transaction (256B payload) Serialized Sizes ===");
    println!("  BCS:      {} bytes", bcs::to_bytes(&tx).unwrap().len());
    println!(
        "  TCS:  {} bytes",
        wincode::serialize(&tx).unwrap().len()
    );
    println!(
        "  Postcard: {} bytes",
        postcard::to_allocvec(&tx).unwrap().len()
    );

    // Batch of 100 transactions
    let batch = random_transaction_batch(&mut rng, 100, 256);
    println!("\n=== TransactionBatch (100 tx, 256B each) Serialized Sizes ===");
    println!("  BCS:      {} bytes", bcs::to_bytes(&batch).unwrap().len());
    println!(
        "  TCS:  {} bytes",
        wincode::serialize(&batch).unwrap().len()
    );
    println!(
        "  Postcard: {} bytes",
        postcard::to_allocvec(&batch).unwrap().len()
    );

    // Just a dummy benchmark to show sizes in the report
    group.bench_function("print_sizes", |b| b.iter(|| 1 + 1));
    group.finish();
}

criterion_group!(
    benches,
    bench_serialize_block_header,
    bench_deserialize_block_header,
    bench_serialize_transaction,
    bench_deserialize_transaction,
    bench_serialize_slice,
    bench_serialize_batch,
    bench_deserialize_batch,
    bench_size_comparison,
);

criterion_main!(benches);
