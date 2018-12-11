//! # Day 09:
//!
//! [Benchmarking report](../../../day09/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day09/target/criterion/day09_part1/report/index.html)
//! * [Part 2](../../../day09/target/criterion/day09_part2/report/index.html)
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

use std::fmt::Debug;

#[macro_use]
extern crate pretty_assertions;
use failure::{format_err, Error};

pub mod initial;
pub use crate::initial::Day09Initial;

type Day09SolutionPart1 = u64;
type Day09SolutionPart2 = i64;

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

#[derive(Debug, PartialEq)]
pub struct Input {
    nb_players: usize,
    last_marble_points: usize,
}

pub fn parse_input(input: &str) -> Result<Input, Error> {
    let mut words = input.split_whitespace();
    let nb_players = words
        .next()
        .ok_or_else(|| format_err!("Cannot extract number of players"))?
        .parse()?;
    let last_marble_points = words
        .nth(5)
        .ok_or_else(|| format_err!("Cannot extract last marble's worth"))?
        .parse()?;

    Ok(Input {
        nb_players,
        last_marble_points,
    })
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day09SolutionPart1, SolutionPart2 = Day09SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day09Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use crate::{parse_input, Input};

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            })
            .unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        let input = "9 players; last marble is worth 25 points";
        let expected = Input {
            nb_players: 9,
            last_marble_points: 25,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "10 players; last marble is worth 1618 points";
        let expected = Input {
            nb_players: 10,
            last_marble_points: 1618,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "13 players; last marble is worth 7999 points";
        let expected = Input {
            nb_players: 13,
            last_marble_points: 7999,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "17 players; last marble is worth 1104 points";
        let expected = Input {
            nb_players: 17,
            last_marble_points: 1104,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "21 players; last marble is worth 6111 points";
        let expected = Input {
            nb_players: 21,
            last_marble_points: 6111,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "30 players; last marble is worth 5807 points";
        let expected = Input {
            nb_players: 30,
            last_marble_points: 5807,
        };
        assert_eq!(parse_input(input).unwrap(), expected);
    }
}
