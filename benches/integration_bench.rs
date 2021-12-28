use criterion::{criterion_group, criterion_main, Criterion};
use rustometry;

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("1 plus 2", |b| b.iter(|| rustometry::test_both()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);