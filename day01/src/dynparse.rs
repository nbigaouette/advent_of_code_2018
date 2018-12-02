use std::collections::HashMap;

use crate::{parse_input, AoC, Day01Data, Day01Solution};

#[derive(Debug)]
pub struct Day01BuildIter<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day01BuildIter<'a> {
    type Solution = Day01Solution;
    type Data = Day01Data<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day01BuildIter {
        Day01BuildIter { input }
    }

    fn parsed(&self) -> Self::Data {
        Box::new(parse_input(self.input))
    }

    fn solution_part1(&self) -> Self::Solution {
        self.parsed().sum()
    }

    fn solution_part2(&self) -> Self::Solution {
        let mut seen_frequencies = HashMap::new();
        let mut frequency = 0;

        // Insert initial point
        seen_frequencies.insert(frequency, 1);

        let inputs: Vec<_> = self.parsed().collect();

        while inputs.iter().cycle().find(|&&i| {
            frequency += i;
            let freq_count = seen_frequencies.entry(frequency).or_insert(0);
            *freq_count += 1;
            *freq_count == 2
        }) == None
        {}

        frequency
    }
}

#[cfg(test)]
mod tests {

    mod aoc2018 {
        mod day01 {
            use crate::{AoC, Day01BuildIter};

            #[test]
            fn parse() {
                let parsed: Vec<_> = Day01BuildIter::new("+1, -2, +3, +1").parsed().collect();
                assert_eq!(parsed, vec![1, -2, 3, 1]);
            }

            mod part1 {

                mod solution {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01BuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        let expected = 408;
                        let to_check = Day01BuildIter::new(PUZZLE_INPUT).solution_part1();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01BuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        let expected = 3;
                        let input = "+1, -2, +3, +1";
                        let to_check = Day01BuildIter::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        init_logger();

                        let expected = 3;
                        let input = "+1, +1, +1";
                        let to_check = Day01BuildIter::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        init_logger();

                        let expected = 0;
                        let input = "+1, +1, -2";
                        let to_check = Day01BuildIter::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        init_logger();

                        let expected = -6;
                        let input = "-1, -2, -3";
                        let to_check = Day01BuildIter::new(input).solution_part1();

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
                    use crate::{AoC, Day01BuildIter, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        let expected = 55250;
                        let to_check = Day01BuildIter::new(PUZZLE_INPUT).solution_part2();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01BuildIter};

                    #[test]
                    fn ex01() {
                        init_logger();

                        let expected = 0;
                        let input = "+1, -1";
                        let to_check = Day01BuildIter::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        init_logger();

                        let expected = 10;
                        let input = "+3, +3, +4, -2, -4";
                        let to_check = Day01BuildIter::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        init_logger();

                        let expected = 5;
                        let input = "-6, +3, +8, +5, -6";
                        let to_check = Day01BuildIter::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        init_logger();

                        let expected = 14;
                        let input = "+7, +7, -2, -7, -4";
                        let to_check = Day01BuildIter::new(input).solution_part2();

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
