use crate::{parse_input, AoC, Day07SolutionPart1, Day07SolutionPart2};

#[derive(Debug)]
pub struct Day07Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day07Initial<'a> {
    type SolutionPart1 = Day07SolutionPart1;
    type SolutionPart2 = Day07SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day07Initial<'_> {
        Day07Initial { input }
    }

    // fn solution_part1(&self) -> Self::SolutionPart1 {
    // }

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day07Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day07Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
