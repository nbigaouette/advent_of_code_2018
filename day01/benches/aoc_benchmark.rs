#[macro_use]
extern crate criterion;

extern crate day01;

use criterion::Criterion;

use day01::{
    aoc_day01_part1, aoc_day01_part2,
    benchmark::{benchmarking_input_part_1, benchmarking_input_part_2},
};

fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day01_part1", |b| {
        b.iter(|| aoc_day01_part1(benchmarking_input_part_1()))
    });
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day01_part2", |b| {
        b.iter(|| aoc_day01_part2(benchmarking_input_part_2()))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);