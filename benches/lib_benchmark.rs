use criterion::{criterion_group, criterion_main, Criterion};
use rust_tsp::run_ga;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run_ga", |b| b.iter(|| run_ga()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
