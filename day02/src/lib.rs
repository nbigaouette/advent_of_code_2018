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

use std::fmt::Debug;

use std::collections::HashMap;

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

type Day02Solution = i64;
type Day02Data<'a> = Box<Iterator<Item = Day02Solution> + 'a>;

static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type ToBenchmark<'a> = Day02BuildIter<'a>;
    pub type BenchmarkVector<'a> =
        Vec<Box<dyn AoC<'a, Solution = Day02Solution, Data = Day02Data<'a>> + 'a>>;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day02BuildIter::new(PUZZLE_INPUT)),
        // ]
    }
}

#[derive(Debug)]
pub struct Day02BuildIter<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day02BuildIter<'a> {
    type Solution = Day02Solution;
    type Data = Day02Data<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day02BuildIter {
        Day02BuildIter { input }
    }

    fn parsed(&self) -> Self::Data {
        // Box::new(parse_input(self.input))
        unimplemented!()
    }

    fn solution_part1(&self) -> Self::Solution {
        let mut count_two = 0;
        let mut count_three = 0;
        self.input.lines().for_each(|line| {
            let line = line.trim();
            println!(
                "line: {}   count_two: {}   count_three: {}",
                line, count_two, count_three
            );
            let mut seen = HashMap::new();
            let mut line_count_two = 0;
            let mut line_count_three = 0;
            line.chars().for_each(|c| {
                let n = seen.entry(c).or_insert(0);
                *n += 1;
                println!("    c: {}  n: {}", c, n);
                if *n == 2 {
                    line_count_two += 1;
                    println!(
                        "        --> line_count_two +=1 --> {}  line_count_three: {}",
                        line_count_two, line_count_three
                    );
                } else if *n == 3 {
                    line_count_two -= 1;
                    line_count_three += 1;
                    println!(
                        "        --> line_count_two -=1 --> {}  line_count_three +=1 --> {}",
                        line_count_two, line_count_three
                    );
                }
            });
            count_two += line_count_two.min(1);
            count_three += line_count_three.min(1);

            println!(
                "  --> count_two: {}   count_three: {}",
                count_two, count_three
            );
        });

        count_two * count_three
    }

    // fn solution_part2(&self) -> Self::Solution {
    // }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

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

    mod aoc2018 {
        mod day02 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day02BuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        let expected = 5000;
                        let to_check = Day02BuildIter::new(PUZZLE_INPUT).solution_part1();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day02BuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        let expected = 12;
                        let input = "abcdef
                                     bababc
                                     abbcde
                                     abcccd
                                     aabcdd
                                     abcdee
                                     ababab";
                        let to_check = Day02BuildIter::new(input).solution_part1();

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
                    use crate::{AoC, Day02BuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let to_check = Day02BuildIter::new(PUZZLE_INPUT).solution_part2();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day02BuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        unimplemented!();

                        let expected = 0;
                        let input = "";
                        let to_check = Day02BuildIter::new(input).solution_part2();

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
