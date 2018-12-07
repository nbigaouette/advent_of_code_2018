#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::{Bencher, Fun};

use dayXX::benchmark::to_benchmark;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let functions: Vec<_> = to_benchmark()
        .into_iter()
        .map(|s| {
            Fun::new(
                &s.description().replace(" ", "_"),
                move |b: &mut Bencher, _: &()| b.iter(|| s.solution_part1()),
            )
        }).collect();

    c.bench_functions("dayXX_part1", functions, ());
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let functions: Vec<_> = to_benchmark()
        .into_iter()
        .map(|s| {
            Fun::new(
                &s.description().replace(" ", "_"),
                move |b: &mut Bencher, _: &()| b.iter(|| s.solution_part2()),
            )
        }).collect();

    c.bench_functions("dayXX_part2", functions, ());
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);
