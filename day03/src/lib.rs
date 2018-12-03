//! # Day 03: No Matter How You Slice It
//!
//! [Benchmarking report](../../../day03/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day03/target/criterion/day03_part1/report/index.html)
//! * [Part 2](../../../day03/target/criterion/day03_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks
//! to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of
//! the night). Unfortunately, anomalies are still affecting them - nobody can even agree on
//! how to _cut_ the fabric.
//!
//! The whole piece of fabric they're working on is a very large square - at least `1000`
//! inches on each side.
//!
//! Each Elf has made a _claim_ about which area of fabric would be ideal for Santa's
//! suit. All claims have an ID and consist of a single rectangle with edges parallel
//! to the edges of the fabric. Each claim's rectangle is defined as follows:
//!
//! * The number of inches between the left edge of the fabric and the left edge of
//!   the rectangle.
//! * The number of inches between the top edge of the fabric and the top edge of
//!   the rectangle.
//! * The width of the rectangle in inches.
//! * The height of the rectangle in inches.
//!
//! A claim like `#123 @ 3,2: 5x4` means that claim ID `123` specifies a rectangle `3`
//! inches from the left edge, `2` inches from the top edge, `5` inches wide, and `4`
//! inches tall. Visually, it claims the square inches of fabric represented by `#` (and
//! ignores the square inches of fabric represented by `.`) in the diagram below:
//!
//! ```text
//! ...........
//! ...........
//! ...#####...
//! ...#####...
//! ...#####...
//! ...#####...
//! ...........
//! ...........
//! ...........
//! ```
//!
//! The problem is that many of the claims _overlap_, causing two or more claims to
//! cover part of the same areas. For example, consider the following claims:
//!
//! ```text
//! #1 @ 1,3: 4x4
//! #2 @ 3,1: 4x4
//! #3 @ 5,5: 2x2
//! ```
//!
//! Visually, these claim the following areas:
//!
//! ```text
//! ........
//! ...2222.
//! ...2222.
//! .11XX22.
//! .11XX22.
//! .111133.
//! .111133.
//! ........
//! ```
//!
//! The four square inches marked with `X` are claimed by _both `1` and `2`_. (Claim `3`,
//! while adjacent to the others, does not overlap either of them.)
//!
//! If the Elves all proceed with their own plans, none of them will have enough fabric. _How
//! many square inches of fabric are within two or more claims?_
//!
//!
//! ## Part Two
//!
//! Amidst the chaos, you notice that exactly one claim doesn't overlap by even a
//! single square inch of fabric with any other claim. If you can somehow draw
//! attention to it, maybe the Elves will be able to make Santa's suit after all!
//!
//! For example, in the claims above, only claim `3` is intact after all claims
//! are made.
//!
//! _What is the ID of the only claim that doesn't overlap?_
//!
//!

// #[macro_use]
// extern crate log;
extern crate ndarray;

use std::fmt::Debug;

pub mod initial;
pub use initial::Day03Initial;

pub mod preparsed;
pub use preparsed::Day03Preparsed;

pub mod preparsed_ndarray;
pub use preparsed_ndarray::Day03PreparsedNdarray;

type Day03SolutionPart1 = usize;
type Day03SolutionPart2 = usize;

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

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day03SolutionPart1, SolutionPart2 = Day03SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![
            Box::new(Day03Initial::new(PUZZLE_INPUT)),
            Box::new(Day03Preparsed::new(PUZZLE_INPUT)),
            Box::new(Day03PreparsedNdarray::new(PUZZLE_INPUT)),
        ]
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
    id: usize,
    left: usize,
    top: usize,
    wide: usize,
    tall: usize,
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
