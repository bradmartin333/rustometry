use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse point cloud", |b| {
        b.iter(|| rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
