use criterion::{criterion_group, criterion_main, Criterion};
use rustometry::{primitives, theta};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get cloud thetas", |b| {
        b.iter(|| {
            theta::plane_from_points(Vec::from([
                primitives::Vec3 {
                    x: 6.0,
                    y: 4.0,
                    z: -1.0,
                },
                primitives::Vec3 {
                    x: 1.0,
                    y: -8.0,
                    z: 3.0,
                },
                primitives::Vec3 {
                    x: 2.0,
                    y: 1.0,
                    z: -4.0,
                },
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
