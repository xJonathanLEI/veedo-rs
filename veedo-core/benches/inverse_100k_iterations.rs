use criterion::{black_box, criterion_group, criterion_main, Criterion};
use veedo_core::inverse_delay_function;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("inverse_100k_iterations", |b| {
        b.iter(|| {
            black_box(inverse_delay_function(
                100_000,
                0x211dadda75e061abcecee71984cff62e_u128,
                0xc6b3e7be7a6b9627af264ab9a2e5ce5_u128,
            ));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
