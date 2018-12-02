//! # Day 02:
//!
//! [Benchmarking report](../../../day02/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day02/target/criterion/day02_part1/report/index.html)
//! * [Part 2](../../../day02/target/criterion/day02_part2/report/index.html)
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

#[macro_use]
extern crate log;
extern crate strsim;
#[macro_use]
extern crate itertools;

use std::fmt::Debug;

pub mod initial;
pub use initial::Day02Initial;

pub mod bound_check_elision;
pub use bound_check_elision::Day02BoundCheckElision;

pub trait AoC<'a>: Debug {
    type Solution1;
    type Solution2;
    type Parsed;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn parsed(&self) -> Self::Parsed {
        unimplemented!()
    }

    fn solution_part1(&self) -> Self::Solution1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::Solution2 {
        unimplemented!()
    }
}

type Day02SolutionPart1 = i64;
type Day02SolutionPart2 = String;
type Day02Parsed<'a> = Box<Iterator<Item = &'a str> + 'a>;

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<
                    'a,
                    Solution1 = Day02SolutionPart1,
                    Solution2 = Day02SolutionPart2,
                    Parsed = Day02Parsed<'a>,
                > + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![
            Box::new(Day02Initial::new(PUZZLE_INPUT)),
            Box::new(Day02BoundCheckElision::new(PUZZLE_INPUT)),
        ]
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|line| line.trim())
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
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
        let input = "abcdef
                     bababc
                     abbcde
                     abcccd
                     aabcdd
                     abcdee
                     ababab";
        let parsed: Vec<_> = parse_input(input).collect();
        assert_eq!(
            parsed,
            vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",]
        );
    }
}
