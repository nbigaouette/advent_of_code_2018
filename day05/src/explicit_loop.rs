use std::mem;

use crate::{AoC, Day05SolutionPart1, Day05SolutionPart2};

// Different cases characters have a distance of 32 in the ASCII table
static ASCII_CAPITAL_DISTANCE: i16 = 32;

#[derive(Debug)]
pub struct Day05ExplicitLoop<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day05ExplicitLoop<'a> {
    type SolutionPart1 = Day05SolutionPart1;
    type SolutionPart2 = Day05SolutionPart2;

    fn description(&self) -> &'static str {
        "Explicit loop"
    }

    fn new(input: &'a str) -> Day05ExplicitLoop {
        Day05ExplicitLoop {
            input: input.trim(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1_explicit_loop(self.input).len()
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
                solution_part1_explicit_loop(&stripped_sequence).len()
            })
            .min()
            .unwrap()
    }
}

pub fn solution_part1_explicit_loop(input: &str) -> String {
    let mut to_scan = input.trim().as_bytes().to_vec();
    let mut answer: Vec<u8> = Vec::with_capacity(to_scan.len());
    loop {
        let mut match_found = false;
        let mut i = 1;
        while i < to_scan.len() {
            let diff = (to_scan[i - 1] as i16 - to_scan[i] as i16).abs();
            if diff == ASCII_CAPITAL_DISTANCE {
                // Match
                match_found = true;
                i += 2;
            } else {
                // No match
                answer.push(to_scan[i - 1]);
                i += 1;
            }
        }
        if i == to_scan.len() {
            answer.push(to_scan[i - 1]);
        }
        mem::swap(&mut answer, &mut to_scan);
        if !match_found {
            break;
        }
        answer.clear();
    }

    String::from_utf8(answer).unwrap()
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day05ExplicitLoop;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 9296;
                let to_check = Day05ExplicitLoop::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05ExplicitLoop;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex04_solution() {
                init_logger();

                let expected = 10;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05ExplicitLoop::new(input).solution_part1();

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
            use super::super::super::Day05ExplicitLoop;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5534;
                let to_check = Day05ExplicitLoop::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05ExplicitLoop;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05ExplicitLoop::new(input).solution_part2();

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
