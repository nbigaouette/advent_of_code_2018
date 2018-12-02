//! # Day XX:
//!
//! [Benchmarking report](../../../dayXX/target/criterion/report/index.html):
//!
//! * [Part 1](../../../dayXX/target/criterion/dayXX_part1/report/index.html)
//! * [Part 2](../../../dayXX/target/criterion/dayXX_part2/report/index.html)
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

pub trait AoC<'a>: Debug {
    type Solution;
    type Data;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn parsed(&self) -> Self::Data;

    fn solution_part1(&self) -> Self::Solution {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::Solution {
        unimplemented!()
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    unimplemented!();
    vec![].into_iter()
}

type DayXXSolution = i64;
type DayXXData<'a> = Box<Iterator<Item = DayXXSolution> + 'a>;

static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type ToBenchmark<'a> = DayXXBuildIter<'a>;
    pub type BenchmarkVector<'a> =
        Vec<Box<dyn AoC<'a, Solution = DayXXSolution, Data = DayXXData<'a>> + 'a>>;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(DayXXBuildIter::new(PUZZLE_INPUT)),
        // ]
    }
}

#[derive(Debug)]
pub struct DayXXBuildIter<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for DayXXBuildIter<'a> {
    type Solution = DayXXSolution;
    type Data = DayXXData<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> DayXXBuildIter {
        DayXXBuildIter { input }
    }

    fn parsed(&self) -> Self::Data {
        Box::new(parse_input(self.input))
    }

    // fn solution_part1(&self) -> Self::Solution {
    // }

    // fn solution_part2(&self) -> Self::Solution {
    // }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    use crate::parse_input;

    fn init_logger() {
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

    mod aoc2018 {
        mod dayXX {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use crate::tests::init_logger;
                    use crate::{AoC, DayXXBuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let to_check = DayXXBuildIter::new(PUZZLE_INPUT).solution_part1();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, DayXXBuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let input = "";
                        let to_check = DayXXBuildIter::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }
                }

                /*
                mod extra {
                    use ::*;
                }
                */
            }

            mod part2 {

                mod solution {
                    use crate::tests::init_logger;
                    use crate::{AoC, DayXXBuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let to_check = DayXXBuildIter::new(PUZZLE_INPUT).solution_part2();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, DayXXBuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let input = "";
                        let to_check = DayXXBuildIter::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }
                }

                /*
                mod extra {
                    use ::*;
                }
                */
            }
        }
    }
}
