#[macro_use]
extern crate criterion;

extern crate dayXX;

use criterion::Criterion;

use dayXX::{aoc_dayXX, benchmark::BENCHMARKING_INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dayXX", |b| b.iter(|| aoc_dayXX(BENCHMARKING_INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
