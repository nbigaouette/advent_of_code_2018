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
//! You notice that the device repeats the same frequency change list over and over.
//! To calibrate the device, you need to find the first frequency it reaches _twice_.
//!
//! For example, using the same list of changes above, the device would loop as follows:
//!
//! * Current frequency ` 0`, change of `+1`; resulting frequency ` 1`.
//! * Current frequency ` 1`, change of `-2`; resulting frequency `-1`.
//! * Current frequency `-1`, change of `+3`; resulting frequency ` 2`.
//! * Current frequency ` 2`, change of `+1`; resulting frequency ` 3`.
//! * (At this point, the device continues from the start of the list.)
//! * Current frequency ` 3`, change of `+1`; resulting frequency ` 4`.
//! * Current frequency ` 4`, change of `-2`; resulting frequency ` 2`, which has
//! already been seen.
//!
//! In this example, the first frequency reached twice is `2`. Note that your device
//! might need to repeat its list of frequency changes many times before a duplicate
//! frequency is found, and that duplicates might be found while in the middle of
//! processing the list.
//!
//! Here are other examples:
//!
//! * `+1, -1` first reaches `0` twice.
//! * `+3, +3, +4, -2, -4` first reaches `10` twice.
//! * `-6, +3, +8, +5, -6` first reaches `5` twice.
//! * `+7, +7, -2, -7, -4` first reaches `14` twice.
//!
//! _What is the first frequency your device reaches twice?_

use std::collections::HashMap;

pub fn parse_input(input: &'static str) -> impl Iterator<Item = i64> {
    input
        .split(|c| c == ',' || c == '\n')
        .filter_map(|p| match p.trim().parse::<i64>() {
            Ok(i) => Some(i),
            Err(e) => {
                // println!("ERROR: Can't parse {:?}: {:?}", p, e);
                None
            }
        })
}

pub fn aoc_day01_part1<I>(inputs: I) -> i64
where
    I: Iterator<Item = i64>,
{
    inputs.sum()
}

pub fn aoc_day01_part2<I>(inputs: I) -> i64
where
    I: Iterator<Item = i64>,
{
    let mut seen_frequencies = HashMap::new();
    let mut frequency = 0;

    // Insert initial point
    seen_frequencies.insert(frequency, 1);

    let inputs: Vec<_> = inputs.collect();

    while inputs.iter().cycle().find(|&&i| {
        frequency += i;
        let freq_count = seen_frequencies.entry(frequency).or_insert(0);
        *freq_count += 1;
        *freq_count == 2
    }) == None
    {}

    frequency
}

const PUZZLE_INPUT: &'static str = include_str!("../input");

pub mod benchmark {
    use crate::{parse_input, PUZZLE_INPUT};
    pub fn benchmarking_input_part_1() -> impl Iterator<Item = i64> {
        parse_input(PUZZLE_INPUT)
    }
    pub fn benchmarking_input_part_2() -> impl Iterator<Item = i64> {
        parse_input(PUZZLE_INPUT)
    }
}

#[cfg(test)]
mod tests {
    mod aoc2018 {
        mod day01 {
            use crate::parse_input;

            #[test]
            fn parse() {
                let parsed: Vec<_> = parse_input("+1, -2, +3, +1").collect();
                assert_eq!(parsed, vec![1, -2, 3, 1]);
            }

            mod part1 {

                mod solution {
                    use crate::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        let expected = 408;
                        let to_check = aoc_day01_part1(parse_input(PUZZLE_INPUT));

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        let expected = 3;
                        let input = "+1, -2, +3, +1";
                        let to_check = aoc_day01_part1(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        let expected = 3;
                        let input = "+1, +1, +1";
                        let to_check = aoc_day01_part1(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        let expected = 0;
                        let input = "+1, +1, -2";
                        let to_check = aoc_day01_part1(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        let expected = -6;
                        let input = "-1, -2, -3";
                        let to_check = aoc_day01_part1(parse_input(input));

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
                    use crate::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        let expected = 55250;
                        let to_check = aoc_day01_part2(parse_input(PUZZLE_INPUT));

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        let expected = 0;
                        let input = "+1, -1";
                        let to_check = aoc_day01_part2(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        let expected = 10;
                        let input = "+3, +3, +4, -2, -4";
                        let to_check = aoc_day01_part2(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        let expected = 5;
                        let input = "-6, +3, +8, +5, -6";
                        let to_check = aoc_day01_part2(parse_input(input));

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        let expected = 14;
                        let input = "+7, +7, -2, -7, -4";
                        let to_check = aoc_day01_part2(parse_input(input));

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
