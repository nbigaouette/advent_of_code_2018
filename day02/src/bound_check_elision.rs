use std::collections::HashMap;

use strsim::hamming;

use crate::{parse_input, AoC, Day02SolutionPart1, Day02SolutionPart2};

#[derive(Debug)]
pub struct Day02BoundCheckElision<'a> {
    lines: Vec<&'a str>,
}

impl<'a> AoC<'a> for Day02BoundCheckElision<'a> {
    type Solution1 = Day02SolutionPart1;
    type Solution2 = Day02SolutionPart2;
    type Parsed = Box<Iterator<Item = &'a str> + 'a>;

    fn description(&self) -> &'static str {
        "Pre-parsed and bound check elision"
    }

    fn new(input: &'a str) -> Day02BoundCheckElision {
        Day02BoundCheckElision {
            lines: parse_input(input).collect(),
        }
    }

    // fn parsed(&self) -> Self::Parsed {
    //     Box::new(self.lines.iter())
    // }

    fn solution_part1(&self) -> Self::Solution1 {
        let mut count_two = 0;
        let mut count_three = 0;
        self.lines.iter().for_each(|line| {
            let mut seen = HashMap::new();
            let mut line_count_two = 0;
            let mut line_count_three = 0;
            line.chars().for_each(|c| {
                let n = seen.entry(c).or_insert(0);
                *n += 1;
                if *n == 2 {
                    line_count_two += 1;
                } else if *n == 3 {
                    line_count_two -= 1;
                    line_count_three += 1;
                }
            });
            count_two += line_count_two.min(1);
            count_three += line_count_three.min(1);
        });

        count_two * count_three
    }

    fn solution_part2(&self) -> Self::Solution2 {
        let mut max_same_chars = 0;
        #[derive(Debug)]
        struct CommonLines<'a> {
            line1: &'a str,
            line2: &'a str,
        }
        let mut matched_lines = None;
        let lines1_it = self.lines.iter();
        for (l1, line1) in lines1_it.enumerate() {
            for line2 in self.lines.iter().skip(l1 + 1) {
                let distance = hamming(line1, line2).unwrap();
                let same_chars = line1.len() - distance;
                if same_chars > max_same_chars {
                    max_same_chars = same_chars;
                    matched_lines = Some(CommonLines { line1, line2 });
                }
            }
        }
        let matched_lines = matched_lines.unwrap();
        let same_chars: String = matched_lines
            .line1
            .chars()
            .zip(matched_lines.line2.chars())
            .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
            .collect();
        same_chars
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    mod part1 {
        mod solution {
            use super::super::super::Day02BoundCheckElision;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5000;
                let to_check = Day02BoundCheckElision::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day02BoundCheckElision;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 12;
                let input = "abcdef
                             bababc
                             abbcde
                             abcccd
                             aabcdd
                             abcdee
                             ababab";
                let to_check = Day02BoundCheckElision::new(input).solution_part1();

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
            use super::super::super::Day02BoundCheckElision;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = "ymdrchgpvwfloluktajxijsqb";
                let to_check = Day02BoundCheckElision::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day02BoundCheckElision;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = "fgij";
                let input = "abcde
                                     fghij
                                     klmno
                                     pqrst
                                     fguij
                                     axcye
                                     wvxyz";
                let to_check = Day02BoundCheckElision::new(input).solution_part2();

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
