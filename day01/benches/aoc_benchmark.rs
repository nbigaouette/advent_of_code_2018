#[macro_use]
extern crate criterion;

extern crate day01;

use criterion::Criterion;

use day01::{
    benchmark::{benchmarking_input_part_1, benchmarking_input_part_2, DayStruct},
    AoC,
};

fn criterion_benchmark_part1(c: &mut Criterion) {
    let v = vec![DayStruct::new(benchmarking_input_part_1())];
    c.bench_function_over_inputs(
        "day01_part1",
        |b, day| {
            b.iter(|| day.solution_part1());
        },
        v,
    );
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let v = vec![DayStruct::new(benchmarking_input_part_2())];
    c.bench_function_over_inputs(
        "day01_part2",
        |b, day| {
            b.iter(|| day.solution_part2());
        },
        v,
    );
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);
