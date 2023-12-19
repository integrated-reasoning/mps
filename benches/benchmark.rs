use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mps::parser;

fn criterion_benchmark(c: &mut Criterion) {
  let a = "NAME          AFIRO\n";
  c.bench_function("name", |b| b.iter(|| parser::name(black_box(a))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
