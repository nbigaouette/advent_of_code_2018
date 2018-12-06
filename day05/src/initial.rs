use std::string::FromUtf8Error;

use crate::{AoC, Day05SolutionPart1, Day05SolutionPart2};

// Different cases characters have a distance of 32 in the ASCII table
static ASCII_CAPITAL_DISTANCE: i16 = 32;

#[derive(Debug)]
pub struct Day05Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day05Initial<'a> {
    type SolutionPart1 = Day05SolutionPart1;
    type SolutionPart2 = Day05SolutionPart2;

    fn description(&self) -> &'static str {
        "Loop until length does not change"
    }

    fn new(input: &'a str) -> Day05Initial {
        Day05Initial {
            input: input.trim(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1_multiple_steps(self.input).len()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let to_remove_pairs = &[
            ('A', 'a'),
            ('B', 'b'),
            ('C', 'c'),
            ('D', 'd'),
            ('E', 'e'),
            ('F', 'f'),
            ('G', 'g'),
            ('H', 'h'),
            ('I', 'i'),
            ('J', 'j'),
            ('K', 'k'),
            ('L', 'l'),
            ('M', 'm'),
            ('N', 'n'),
            ('O', 'o'),
            ('P', 'p'),
            ('Q', 'q'),
            ('R', 'r'),
            ('S', 's'),
            ('T', 't'),
            ('U', 'u'),
            ('V', 'v'),
            ('W', 'w'),
            ('X', 'x'),
            ('Y', 'y'),
            ('Z', 'z'),
        ];

        to_remove_pairs
            .iter()
            .map(|to_remove_pair| {
                let stripped_sequence = self
                    .input
                    .split(to_remove_pair.0)
                    .collect::<Vec<&str>>()
                    .concat()
                    .split(to_remove_pair.1)
                    .collect::<Vec<&str>>()
                    .concat();
                solution_part1_multiple_steps(&stripped_sequence).len()
            })
            .min()
            .unwrap()
    }
}

pub fn solution_part1_multiple_steps(input: &str) -> String {
    let mut bytes = input.trim().as_bytes().to_vec();
    let mut prev_len = bytes.len();
    loop {
        bytes = solution_part1_one_step_bytes(&bytes);
        if bytes.len() == prev_len {
            break;
        }
        prev_len = bytes.len();
    }

    String::from_utf8(bytes).unwrap()
}

pub fn solution_part1_one_step_bytes(input: &[u8]) -> Vec<u8> {
    let mut answer: Vec<u8> = Vec::with_capacity(input.len());

    let mut i = 0;
    while i < input.len() - 1 {
        let diff = (input[i] as i16 - input[i + 1] as i16).abs();
        if diff == ASCII_CAPITAL_DISTANCE {
            i += 1;
        } else {
            answer.push(input[i]);
        }
        i += 1;
    }
    if i + 1 == input.len() {
        answer.push(input[i]);
    }

    answer
}

pub fn solution_part1_one_step(input: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(solution_part1_one_step_bytes(input.trim().as_bytes()))
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day05Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 9296;
                let to_check = Day05Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05Initial;
            use super::super::super::{solution_part1_multiple_steps, solution_part1_one_step};
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01_single_step() {
                init_logger();
                assert_eq!(solution_part1_one_step("aA").unwrap(), "");
                assert_eq!(solution_part1_one_step("abBA").unwrap(), "aA");
                assert_eq!(solution_part1_one_step("abAB").unwrap(), "abAB");
                assert_eq!(solution_part1_one_step("aabAAB").unwrap(), "aabAAB");
            }

            #[test]
            fn ex02_steps() {
                init_logger();

                assert_eq!(
                    solution_part1_one_step("dabAcCaCBAcCcaDA").unwrap(),
                    // "dabAaCBAcCcaDA"
                    "dabAaCBAcaDA"
                );
                assert_eq!(
                    solution_part1_one_step(&solution_part1_one_step("dabAcCaCBAcCcaDA").unwrap())
                        .unwrap(),
                    "dabCBAcaDA"
                );
                assert_eq!(
                    solution_part1_one_step(
                        &solution_part1_one_step(
                            &solution_part1_one_step("dabAcCaCBAcCcaDA").unwrap()
                        )
                        .unwrap()
                    )
                    .unwrap(),
                    "dabCBAcaDA"
                );
            }

            #[test]
            fn ex03_full() {
                init_logger();

                assert_eq!(
                    solution_part1_multiple_steps("dabAcCaCBAcCcaDA"),
                    "dabCBAcaDA"
                );
            }

            #[test]
            fn ex04_solution() {
                init_logger();

                let expected = 10;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
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
            use super::super::super::Day05Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5534;
                let to_check = Day05Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use ::*;
        }
        */
    }
}
