use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use kvs::*;
use std::{
    env::current_dir,
    sync::{Arc, RwLock},
    thread, time,
};
use tempfile::TempDir;
extern crate num_cpus;

const ADDR: &str = "127.0.0.1:7878";

fn get_thread_sizes() -> Vec<usize> {
    let mut thread_sizes = vec![];
    let mut size = 1;
    while size <= num_cpus::get() * 2 {
        thread_sizes.push(size);
        size *= 2;
    }
    thread_sizes
}

fn write_queued_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");

    for size in get_thread_sizes() {
        // Server
        // let engine = KvStore::open(TempDir::new().unwrap().path()).unwrap();
        let engine = KvStore::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = SharedQueueThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        thread::sleep(time::Duration::from_millis(5000)); // Waiting server start...

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap();
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.set(&format!("key{}", idx), "value") {
                                    Ok(a) => assert_eq!(a, ()),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

fn read_queued_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_queued_kvstore");

    for size in get_thread_sizes() {
        let engine = KvStore::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = SharedQueueThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        // Waiting server start...
        thread::sleep(time::Duration::from_millis(5000));
        // Write preset data
        let client = KvsClient::new(ADDR);
        for idx in 1..=1000 {
            client.set(&format!("key{}", idx), "value").unwrap();
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap();
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.get(&format!("key{}", idx)) {
                                    Ok(a) => assert_eq!(a, Some("value".to_string())),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

fn write_rayon_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_kvstore");

    for size in get_thread_sizes() {
        // Server
        // let engine = KvStore::open(TempDir::new().unwrap().path()).unwrap();
        let engine = KvStore::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = RayonThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        thread::sleep(time::Duration::from_millis(5000)); // Waiting server start...

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap(); // Don't new RayonThreadPool here
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.set(&format!("key{}", idx), "value") {
                                    Ok(a) => assert_eq!(a, ()),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

// FIXME: Cannot be executed correctly, but I dont know why
fn read_rayon_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_queued_kvstore");

    for size in get_thread_sizes() {
        let engine = KvStore::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = RayonThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        // Waiting server start...
        thread::sleep(time::Duration::from_millis(5000));
        // Write preset data
        let client = KvsClient::new(ADDR);
        for idx in 1..=1000 {
            client.set(&format!("key{}", idx), "value").unwrap();
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap();
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.get(&format!("key{}", idx)) {
                                    Ok(a) => assert_eq!(a, Some("value".to_string())),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

fn write_queued_jammdb(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_queued_jammdb");

    for size in get_thread_sizes() {
        let engine = Jammdb::open(TempDir::new().unwrap().path().join("bench_storage")).unwrap();
        // let engine = Jammdb::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = SharedQueueThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        thread::sleep(time::Duration::from_millis(5000)); // Waiting server start...

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap();
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.set(&format!("key{}", idx), "value") {
                                    Ok(a) => assert_eq!(a, ()),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

fn read_queued_jammdb(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_queued_kvstore");

    for size in get_thread_sizes() {
        let engine = Jammdb::open(TempDir::new().unwrap().path().join("bench_storage")).unwrap();
        // let engine = Jammdb::open(current_dir().unwrap().join("bench_storage")).unwrap();
        let threadpool = SharedQueueThreadPool::new(size).unwrap();
        let server = Arc::new(RwLock::new(KvsServer::new(engine, threadpool).unwrap()));
        let server_clone = server.clone();
        thread::spawn(move || {
            server_clone.read().unwrap().start(ADDR).unwrap();
        });
        // Waiting server start...
        thread::sleep(time::Duration::from_millis(5000));
        // Write preset data
        let client = KvsClient::new(ADDR);
        for idx in 1..=1000 {
            client.set(&format!("key{}", idx), "value").unwrap();
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("thread_size = {}", size)),
            &size,
            |b, &_size| {
                b.iter_batched_ref(
                    || {
                        let client_threadpool = SharedQueueThreadPool::new(10).unwrap();
                        client_threadpool
                    },
                    |client_threadpool| {
                        for idx in 1..=1000 {
                            client_threadpool.spawn(move || {
                                // KvsClient is blocking
                                let client = KvsClient::new(ADDR);
                                match client.get(&format!("key{}", idx)) {
                                    Ok(a) => assert_eq!(a, Some("value".to_string())),
                                    Err(err) => eprintln!("{:?}", err),
                                }
                            })
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        let s = server.read().unwrap();
        s.shutdown().unwrap();
    }

    group.finish();
}

criterion_group!(
    benches,
    write_queued_kvstore,
    read_queued_kvstore,
    write_rayon_kvstore,
    read_rayon_kvstore,
    write_queued_jammdb,
    read_queued_jammdb,
);
criterion_main!(benches);
