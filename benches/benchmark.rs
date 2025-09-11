use criterion::{criterion_group, criterion_main, Criterion};

mod benchmark_poseidon;
mod benchmark_poseidon_top_level;
mod benchmark_sha;

use benchmark_poseidon::bench_function_poseidon;
use benchmark_poseidon_top_level::bench_function_poseidon_top_level;
use benchmark_sha::bench_function_sha;

criterion_group!(
    name = benches;
    config = Criterion::default()
            .sample_size(1000)
            .warm_up_time(std::time::Duration::from_secs(5));
    targets = bench_function_poseidon_top_level,
              bench_function_sha,
              bench_function_poseidon
);
criterion_main!(benches);
