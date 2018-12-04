use crate::{parse_input, AoC, Day04SolutionPart1, Day04SolutionPart2};

#[derive(Debug)]
pub struct Day04Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day04Initial<'a> {
    type SolutionPart1 = Day04SolutionPart1;
    type SolutionPart2 = Day04SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day04Initial {
        Day04Initial { input }
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
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 240;
                let input = "[1518-11-01 00:00] Guard #10 begins shift
                             [1518-11-01 00:05] falls asleep
                             [1518-11-01 00:25] wakes up
                             [1518-11-01 00:30] falls asleep
                             [1518-11-01 00:55] wakes up
                             [1518-11-01 23:58] Guard #99 begins shift
                             [1518-11-02 00:40] falls asleep
                             [1518-11-02 00:50] wakes up
                             [1518-11-03 00:05] Guard #10 begins shift
                             [1518-11-03 00:24] falls asleep
                             [1518-11-03 00:29] wakes up
                             [1518-11-04 00:02] Guard #99 begins shift
                             [1518-11-04 00:36] falls asleep
                             [1518-11-04 00:46] wakes up
                             [1518-11-05 00:03] Guard #99 begins shift
                             [1518-11-05 00:45] falls asleep
                             [1518-11-05 00:55] wakes up";
                let to_check = Day04Initial::new(input).solution_part1();

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
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day04Initial::new(input).solution_part2();

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
