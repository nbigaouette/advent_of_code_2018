//! # Day 07:
//!
//! [Benchmarking report](../../../day07/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day07/target/criterion/day07_part1/report/index.html)
//! * [Part 2](../../../day07/target/criterion/day07_part2/report/index.html)
//!
//!
//! ## Part One
//!
//!
//!
//! ## Part Two
//!
//!
//!

// #[macro_use]
// extern crate log;

use std::fmt::Debug;

pub mod initial;
pub use crate::initial::Day07Initial;

type Day07SolutionPart1 = i64;
type Day07SolutionPart2 = i64;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    unimplemented!();
    vec![].into_iter()
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day07SolutionPart1, SolutionPart2 = Day07SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day07Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use crate::parse_input;

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            }).unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        unimplemented!();
        let parsed: Vec<_> = parse_input("").collect();
        assert_eq!(parsed, vec![]);
    }
}
