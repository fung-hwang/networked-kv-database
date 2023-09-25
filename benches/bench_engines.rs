use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use kvs::*;
use rand::prelude::*;
use tempfile::TempDir;

const SEED: u64 = 10;

fn engine_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_write");
    const KV_SIZE: usize = 1000;

    // Preset data
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut key_numbers = vec![0u32; KV_SIZE];
    // We don't care duplicate numbers in the vector
    for num in &mut key_numbers {
        *num = rng.next_u32();
    }

    group.bench_function("kvs", |b| {
        b.iter_batched(
            || {
                let store = KvStore::open(TempDir::new().unwrap().path()).unwrap();
                store
            },
            |mut store| {
                for i in 0..KV_SIZE {
                    store
                        .set(format!("key{}", key_numbers[i]), "value".to_string())
                        .unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("redb", |b| {
        b.iter_batched(
            || {
                let store = Redb::open(TempDir::new().unwrap().path()).unwrap();
                store
            },
            |mut store| {
                for i in 0..KV_SIZE {
                    store
                        .set(format!("key{}", key_numbers[i]), "value".to_string())
                        .unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn engine_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_write");
    const KV_SIZE: usize = 1000;

    // Preset data
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut key_numbers = vec![0u32; KV_SIZE];
    // We don't care duplicate numbers in the vector
    for num in &mut key_numbers {
        *num = rng.next_u32();
    }

    for size in &[100, 500, 1000] {
        group.bench_with_input(format!("kvs_{}", size), size, |b, size| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();
            for key_i in &key_numbers {
                store
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng2 = SmallRng::seed_from_u64(SEED);
            let key_to_read: Vec<_> = vec![0; *size]
                .iter()
                .map(|_| rng2.gen_range(0..KV_SIZE))
                .map(|i| &key_numbers[i])
                .collect();

            b.iter(|| {
                for key_i in &key_to_read {
                    store.get(format!("key{}", key_i)).unwrap();
                }
            })
        });
    }

    for size in &[100, 500, 1000] {
        group.bench_with_input(format!("redb_{}", size), size, |b, size| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = Redb::open(temp_dir.path()).unwrap();
            for key_i in &key_numbers {
                store
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng2 = SmallRng::seed_from_u64(SEED);
            let key_to_read: Vec<_> = vec![0; *size]
                .iter()
                .map(|_| rng2.gen_range(0..KV_SIZE))
                .map(|i| &key_numbers[i])
                .collect();

            b.iter(|| {
                for key_i in &key_to_read {
                    store.get(format!("key{}", key_i)).unwrap();
                }
            })
        });
    }
}

criterion_group!(benches, engine_write, engine_read);
criterion_main!(benches);
