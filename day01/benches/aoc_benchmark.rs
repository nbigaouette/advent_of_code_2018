#[macro_use]
extern crate criterion;

extern crate day01;

use criterion::Criterion;

use day01::{aoc_day01, benchmark::BENCHMARKING_INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| aoc_day01(BENCHMARKING_INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
