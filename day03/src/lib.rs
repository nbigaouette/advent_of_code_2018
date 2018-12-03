//! # Day 03:
//!
//! [Benchmarking report](../../../day03/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day03/target/criterion/day03_part1/report/index.html)
//! * [Part 2](../../../day03/target/criterion/day03_part2/report/index.html)
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
pub use initial::Day03Initial;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;
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

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

pub fn parse_input_str<'a>(input: &'a str) -> impl Iterator<Item = InputStr<'a>> + 'a {
    input.lines().map(move |line| {
        let mut s = line.trim().split(' ');
        // Id
        let id = &s.next().unwrap()[1..];
        // Skip @
        let _ = s.next().unwrap();
        // Position
        let mut position = s.next().unwrap().split(',');
        let left = position.next().unwrap();
        let top = position.next().unwrap().trim_end_matches(':');
        // Size
        let mut size = s.next().unwrap().split('x');
        let wide = size.next().unwrap();
        let tall = size.next().unwrap();
        InputStr {
            id,
            left,
            top,
            wide,
            tall,
        }
    })
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Input> + 'a {
    parse_input_str(input).map(|i| Input::from(i))
}

type Day03SolutionPart1 = u64;
type Day03SolutionPart2 = u64;
type Day03Parsed<'a> = Box<Iterator<Item = Day03SolutionPart1> + 'a>;

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<
                    'a,
                    SolutionPart1 = Day03SolutionPart1,
                    SolutionPart2 = Day03SolutionPart2,
                    Parsed = Day03Parsed<'a>,
                > + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day03Initial::new(PUZZLE_INPUT))]
    }
}

#[derive(Debug, PartialEq)]
pub struct InputStr<'a> {
    id: &'a str,
    left: &'a str,
    top: &'a str,
    wide: &'a str,
    tall: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Input {
    id: u64,
    left: u64,
    top: u64,
    wide: u64,
    tall: u64,
}

impl<'a> From<InputStr<'a>> for Input {
    fn from(input: InputStr<'a>) -> Self {
        Input {
            id: input.id.parse().unwrap(),
            left: input.left.parse().unwrap(),
            top: input.top.parse().unwrap(),
            wide: input.wide.parse().unwrap(),
            tall: input.tall.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    use crate::{parse_input, parse_input_str, Input, InputStr};

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
    fn parse_ex01() {
        init_logger();

        let input = "#123 @ 3,2: 5x4";
        let parsed: Vec<InputStr> = parse_input_str(input).collect();
        assert_eq!(
            parsed,
            vec![InputStr {
                id: "123",
                left: "3",
                top: "2",
                wide: "5",
                tall: "4",
            }]
        );
    }

    #[test]
    fn parse_ex02() {
        init_logger();

        let input = "#1 @ 1,3: 4x4
                     #2 @ 3,1: 4x4
                     #3 @ 5,5: 2x2";
        let parsed: Vec<InputStr> = parse_input_str(input).collect();
        assert_eq!(
            parsed,
            vec![
                InputStr {
                    id: "1",
                    left: "1",
                    top: "3",
                    wide: "4",
                    tall: "4",
                },
                InputStr {
                    id: "2",
                    left: "3",
                    top: "1",
                    wide: "4",
                    tall: "4",
                },
                InputStr {
                    id: "3",
                    left: "5",
                    top: "5",
                    wide: "2",
                    tall: "2",
                },
            ]
        );
    }

    #[test]
    fn parse() {
        init_logger();

        let input = "#123 @ 3,2: 5x4";
        let parsed: Vec<Input> = parse_input(input).collect();
        assert_eq!(
            parsed,
            vec![Input {
                id: 123,
                left: 3,
                top: 2,
                wide: 5,
                tall: 4,
            }]
        );
    }
}
