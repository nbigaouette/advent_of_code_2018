use std::mem;

use crate::{AoC, Day05SolutionPart1, Day05SolutionPart2};

// Different cases characters have a distance of 32 in the ASCII table
static ASCII_CAPITAL_DISTANCE: i16 = 32;

#[derive(Debug)]
pub struct Day05IteratorScan<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day05IteratorScan<'a> {
    type SolutionPart1 = Day05SolutionPart1;
    type SolutionPart2 = Day05SolutionPart2;

    fn description(&self) -> &'static str {
        "Loop until length does not change using Iterator scan"
    }

    fn new(input: &'a str) -> Day05IteratorScan {
        Day05IteratorScan {
            input: input.trim(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1_scan(self.input).len()
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
                solution_part1_scan(&stripped_sequence).len()
            })
            .min()
            .unwrap()
    }
}

pub fn solution_part1_scan(input: &str) -> String {
    let mut to_scan = input.trim().as_bytes().to_vec();
    let mut tmp: Vec<u8> = Vec::with_capacity(to_scan.len());
    let mut prev_len = to_scan.len();
    // struct FoldAccumulator {
    //     vec: Vec<u8>,
    //     skip: bool,
    // };
    loop {
        tmp.clear();
        let mut match_found = false;
        let initial_value_skip = false;
        let last_skip = to_scan
            .windows(2)
            .scan(initial_value_skip, |skip, bytes_pair| {
                if *skip {
                    *skip = false;
                } else {
                    let diff = (bytes_pair[0] as i16 - bytes_pair[1] as i16).abs();
                    if diff == ASCII_CAPITAL_DISTANCE {
                        // Byte was not inserted due to match; skip the next one too.
                        match_found = true;
                        *skip = true;
                    } else {
                        tmp.push(bytes_pair[0]);
                    }
                }
                Some(*skip)
            })
            .last()
            .unwrap();

        if !last_skip {
            tmp.push(to_scan[to_scan.len() - 1]);
        }

        mem::swap(&mut tmp, &mut to_scan);
        if to_scan.len() == prev_len {
            break;
        }
        prev_len = to_scan.len();
    }

    String::from_utf8(to_scan).unwrap()
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day05IteratorScan;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 9296;
                let to_check = Day05IteratorScan::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05IteratorScan;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01_solution() {
                init_logger();

                let expected = 10;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05IteratorScan::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod extra {
            use super::super::super::solution_part1_scan;
            use crate::tests::init_logger;

            #[test]
            fn ex03_full() {
                init_logger();

                assert_eq!(solution_part1_scan("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day05IteratorScan;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5534;
                let to_check = Day05IteratorScan::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05IteratorScan;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05IteratorScan::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day05XInitial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
