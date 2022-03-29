use std::hash::Hasher;
use core::time::Duration;
use std::hash::BuildHasher;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use ahash::*;

fn hash_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("hash");
    group.measurement_time(Duration::from_secs(60));
    group.throughput(Throughput::Elements(1));

    let hash_builder = RandomState::with_seeds(
        0xbb8c484891ec6c86,
        0x0522a25ae9c769f9,
        0xeed2797b9571bc75,
        0x4feb29c1fbbd59d0,
    );

    let mut v: u64 = 0;

    group.bench_function("u64", |b| {
        b.iter(|| {
        	let mut hasher = hash_builder.build_hasher();
        	hasher.write_u64(v);
        	v = hasher.finish();
        })
    });
}

criterion_group!(benches, hash_benchmark,);
criterion_main!(benches);