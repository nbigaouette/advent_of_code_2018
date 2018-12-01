#[macro_use]
extern crate criterion;

extern crate day01;

use std::env;

use criterion::Criterion;

use day01::{
    benchmark::{benchmarking_input_part_1, benchmarking_input_part_2, DayStruct},
    AoC,
};

fn criterion_benchmark_part1(c: &mut Criterion) {
    env::set_var("RUST_LOG", "");
    let day01 = DayStruct::new(benchmarking_input_part_1());
    let v = &[day01];
    c.bench_function_over_inputs(
        "day01_part1",
        |b, &day| {
            b.iter(|| day.solution_part1());
        },
        v,
    );
}

// fn criterion_benchmark_part1(c: &mut Criterion) {
//     env::set_var("RUST_LOG", "");
//     c.bench_function("day01_part1", |b| {
//         b.iter(|| DayStruct::new(benchmarking_input_part_1()).solution_part1())
//     });
// }

// fn criterion_benchmark_part2(c: &mut Criterion) {
//     env::set_var("RUST_LOG", "");
//     c.bench_function("day01_part2", |b| {
//         b.iter(|| DayStruct::new(benchmarking_input_part_2()).solution_part2())
//     });
// }

criterion_group!(
    benches,
    criterion_benchmark_part1,
    // criterion_benchmark_part2
);
criterion_main!(benches);
