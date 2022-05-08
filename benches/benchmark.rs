use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rgeo::search;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| b.iter(|| search(black_box(44.353_34_f32), black_box(-72.740_23_f32))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);