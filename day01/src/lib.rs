//! # Day 01: Chronal Calibration
//!
//! ## Part One
//!
//! "We've detected some temporal anomalies," one of Santa's Elves at the Temporal
//! Anomaly Research and Detection Instrument Station tells you. She sounded pretty worried
//! when she called you down here. "At 500-year intervals into the past, someone has been
//! changing Santa's history!"
//!
//! "The good news is that the changes won't propagate to our time stream for another 25
//! days, and we have a device" - she attaches something to your wrist - "that will let
//! you fix the changes with no such propagation delay. It's configured to send you 500
//! years further into the past every few days; that was the best we could do on such
//! short notice."
//! "The bad news is that we are detecting roughly _fifty_ anomalies throughout time;
//! the device will indicate fixed anomalies with _stars_. The other bad news is that
//! we only have one device and you're the best person for the job! Good lu--" She taps
//! a button on the device and you suddenly feel like you're falling. To save Christmas,
//! you need to get all _fifty stars_ by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day
//! in the advent calendar; the second puzzle is unlocked when you complete the first.
//! Each puzzle grants _one star_. Good luck!
//!
//! After feeling like you've been falling for a few minutes, you look at the device's
//! tiny screen. "Error: Device must be calibrated before first use. Frequency drift
//! detected. Cannot maintain destination lock." Below the message, the device shows
//! a sequence of changes in frequency (your puzzle input). A value like `+6` means
//! the current frequency increases by `6`; a value like `-3` means the current
//! frequency decreases by `3`.
//!
//! For example, if the device displays frequency changes of `+1, -2, +3, +1`, then
//! starting from a frequency of zero, the following changes would occur:
//!
//! * Current frequency ` 0`, change of `+1`; resulting frequency ` 1`.
//! * Current frequency ` 1`, change of `-2`; resulting frequency `-1`.
//! * Current frequency `-1`, change of `+3`; resulting frequency ` 2`.
//! * Current frequency ` 2`, change of `+1`; resulting frequency ` 3`.
//!
//! In this example, the resulting frequency is `3`.
//!
//! Here are other example situations:
//!
//! * `+1, +1, +1` results in ` 3`
//! * `+1, +1, -2` results in ` 0`
//! * `-1, -2, -3` results in `-6`
//!
//! Starting with a frequency of zero, _what is the resulting frequency_ after all
//! of the changes in frequency have been applied?
//!
//!
//! ## Part Two
//!
//!

type SolutionPart1 = i64;
type SolutionPart2 = i64;

pub struct Solution {
    pub part1: SolutionPart1,
    pub part2: SolutionPart2,
}

pub fn parse_part1(input: &'static str) -> impl Iterator<Item = SolutionPart1> {
    input
        .split(|c| c == ',' || c == '\n')
        .filter_map(|p| match p.trim().parse::<SolutionPart1>() {
            Ok(i) => Some(i),
            Err(e) => {
                println!("Can't parse {:?}: {:?}", p, e);
                None
            }
        })
}

pub fn aoc_day01_part1(input: &'static str) -> SolutionPart1 {
    parse_part1(input).sum()
}

pub fn aoc_day01(input: &'static str) -> Solution {
    Solution {
        part1: aoc_day01_part1(input),
        part2: 0,
    }
}

pub mod benchmark {
    pub const BENCHMARKING_INPUT: &str = "";
}

#[cfg(test)]
mod tests {
    mod aoc2018 {
        mod day01 {
            use crate::parse_part1;

            const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn parse() {
                let parsed: Vec<_> = parse_part1("+1, -2, +3, +1").collect();
                assert_eq!(parsed, vec![1, -2, 3, 1]);
            }

            mod part1 {

                mod solution {
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        let expected = 408;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        let expected = 3;
                        let input = "+1, -2, +3, +1";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        let expected = 3;
                        let input = "+1, +1, +1";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        let expected = 0;
                        let input = "+1, +1, -2";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        let expected = -6;
                        let input = "-1, -2, -3";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

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
                    // use super::super::PUZZLE_INPUT;
                    // use *;

                    // #[test]
                    // fn solution() {
                    //     unimplemented!();
                    //     // let expected = ;
                    //     let Solution {
                    //         part1: to_check,
                    //         part2: _,
                    //     } = aoc_day01(PUZZLE_INPUT);

                    //     assert_eq!(expected, to_check);
                    // }
                }

                mod given {
                    // use *;

                    // #[test]
                    // fn ex01() {
                    //     unimplemented!();
                    //     // let expected = ;
                    //     let input = "";
                    //     let Solution {
                    //         part1: to_check,
                    //         part2: _,
                    //     } = aoc_day01(input);

                    //     assert_eq!(expected, to_check);
                    // }
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
