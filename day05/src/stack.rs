use rayon::prelude::*;

use crate::{AoC, Day05SolutionPart1, Day05SolutionPart2};

#[derive(Debug)]
pub struct Day05Stack<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day05Stack<'a> {
    type SolutionPart1 = Day05SolutionPart1;
    type SolutionPart2 = Day05SolutionPart2;

    fn description(&self) -> &'static str {
        "Stack"
    }

    fn new(input: &'a str) -> Day05Stack {
        Day05Stack {
            input: input.trim(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        stack(self.input.chars(), self.input.len())
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let input_len = self.input.len();
        "abcdefghijklmnopqrstuvwxyz"
            .par_chars()
            .map(|c| {
                stack(
                    self.input.chars().filter(|i| i.to_ascii_lowercase() != c),
                    input_len,
                )
            })
            .min()
            .unwrap()
    }
}

fn stack(input: impl Iterator<Item = char>, len: usize) -> Day05SolutionPart1 {
    let mut stack: Vec<char> = Vec::with_capacity(len);

    for c in input {
        if !stack.is_empty()
            && stack[stack.len() - 1] != c
            && stack[stack.len() - 1].to_ascii_lowercase() == c.to_ascii_lowercase()
        {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.len()
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day05Stack;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 9296;
                let to_check = Day05Stack::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05Stack;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex04_solution() {
                init_logger();

                let expected = 10;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05Stack::new(input).solution_part1();

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
            use super::super::super::Day05Stack;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5534;
                let to_check = Day05Stack::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05Stack;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "dabAcCaCBAcCcaDA";
                let to_check = Day05Stack::new(input).solution_part2();

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
