use crate::{parse_input, AoC, DayXXParsed, DayXXSolutionPart1, DayXXSolutionPart2};

#[derive(Debug)]
pub struct DayXXInitial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for DayXXInitial<'a> {
    type SolutionPart1 = DayXXSolutionPart1;
    type SolutionPart2 = DayXXSolutionPart2;
    type Parsed = DayXXParsed<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> DayXXInitial {
        DayXXInitial { input }
    }

    // fn parsed(&self) -> Self::Parsed {
    //     Box::new(parse_input(self.input))
    // }

    // fn solution_part1(&self) -> Self::SolutionPart1 {
    // }

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::DayXXInitial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = DayXXInitial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::DayXXInitial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = DayXXInitial::new(input).solution_part1();

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
            use super::super::super::DayXXInitial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = DayXXInitial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::DayXXInitial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = DayXXInitial::new(input).solution_part2();

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