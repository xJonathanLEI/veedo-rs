use criterion::{black_box, criterion_group, criterion_main, Criterion};
use veedo_core::compute_delay_function;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("compute_100k_iterations", |b| {
        b.iter(|| {
            black_box(compute_delay_function(100_000, 1, 1));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
