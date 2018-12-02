use std::collections::HashSet;

use crate::{parse_input, AoC, Day01Data, Day01Solution};

#[derive(Debug)]
pub struct Day01PreParseHashSet {
    input: Vec<i64>,
}

impl<'a> AoC<'a> for Day01PreParseHashSet {
    type Solution = Day01Solution;
    type Data = Day01Data<'a>;
    // type Data = Box<Iterator<Item = Day01Solution> + 'a>;

    fn description(&self) -> &'static str {
        "Pre-Parse string HashSet"
    }

    fn new(input: &'a str) -> Day01PreParseHashSet {
        Day01PreParseHashSet {
            input: parse_input(input).collect(),
        }
    }

    // fn parsed(&self) -> Self::Data {
    fn parsed(&self) -> Box<Iterator<Item = i64> + 'a> {
        // let i = self.input.iter().cloned();
        // let d = Box::new(i);
        // d
        unimplemented!()
    }

    fn solution_part1(&self) -> Self::Solution {
        self.input.iter().sum()
    }

    fn solution_part2(&self) -> Self::Solution {
        let mut seen_frequencies = HashSet::new();
        let mut frequency = 0;

        // Insert initial point
        seen_frequencies.insert(frequency);

        while self.input.iter().cycle().find(|&&i| {
            frequency += i;
            if seen_frequencies.contains(&frequency) {
                true
            } else {
                seen_frequencies.insert(frequency);
                false
            }
        }) == None
        {}

        frequency
    }
}

#[cfg(test)]
mod tests {

    mod aoc2018 {
        mod day01 {
            use crate::{AoC, Day01PreParseHashSet};

            #[test]
            #[ignore]
            fn parse() {
                let parsed: Vec<_> = Day01PreParseHashSet::new("+1, -2, +3, +1")
                    .parsed()
                    .collect();
                assert_eq!(parsed, vec![1, -2, 3, 1]);
            }

            mod part1 {

                mod solution {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01PreParseHashSet, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        let expected = 408;
                        let to_check = Day01PreParseHashSet::new(PUZZLE_INPUT).solution_part1();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01PreParseHashSet};

                    #[test]
                    fn ex01() {
                        init_logger();

                        let expected = 3;
                        let input = "+1, -2, +3, +1";
                        let to_check = Day01PreParseHashSet::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        init_logger();

                        let expected = 3;
                        let input = "+1, +1, +1";
                        let to_check = Day01PreParseHashSet::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        init_logger();

                        let expected = 0;
                        let input = "+1, +1, -2";
                        let to_check = Day01PreParseHashSet::new(input).solution_part1();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        init_logger();

                        let expected = -6;
                        let input = "-1, -2, -3";
                        let to_check = Day01PreParseHashSet::new(input).solution_part1();

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
                    use crate::{AoC, Day01PreParseHashSet, PUZZLE_INPUT};

                    #[test]
                    fn solution() {
                        init_logger();

                        let expected = 55250;
                        let to_check = Day01PreParseHashSet::new(PUZZLE_INPUT).solution_part2();

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use crate::tests::init_logger;
                    use crate::{AoC, Day01PreParseHashSet};

                    #[test]
                    fn ex01() {
                        init_logger();

                        let expected = 0;
                        let input = "+1, -1";
                        let to_check = Day01PreParseHashSet::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02() {
                        init_logger();

                        let expected = 10;
                        let input = "+3, +3, +4, -2, -4";
                        let to_check = Day01PreParseHashSet::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03() {
                        init_logger();

                        let expected = 5;
                        let input = "-6, +3, +8, +5, -6";
                        let to_check = Day01PreParseHashSet::new(input).solution_part2();

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04() {
                        init_logger();

                        let expected = 14;
                        let input = "+7, +7, -2, -7, -4";
                        let to_check = Day01PreParseHashSet::new(input).solution_part2();

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
