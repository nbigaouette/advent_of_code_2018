use std::collections::HashMap;

use strsim::hamming;

use crate::{parse_input, AoC, Day02Parsed, Day02SolutionPart1, Day02SolutionPart2};

#[derive(Debug)]
pub struct Day02Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day02Initial<'a> {
    type Solution1 = Day02SolutionPart1;
    type Solution2 = Day02SolutionPart2;
    type Parsed = Day02Parsed<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day02Initial {
        Day02Initial { input }
    }

    fn solution_part1(&self) -> Self::Solution1 {
        let mut count_two = 0;
        let mut count_three = 0;
        parse_input(self.input).for_each(|line| {
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
        let lines: Vec<_> = parse_input(self.input).collect();
        for i in 0..lines.len() {
            for j in i + 1..lines.len() {
                let distance = hamming(lines[i], lines[j]).unwrap();
                let same_chars = lines[i].len() - distance;
                if same_chars > max_same_chars {
                    max_same_chars = same_chars;
                    matched_lines = Some(CommonLines {
                        line1: lines[i],
                        line2: lines[j],
                    });
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

    mod part1 {
        mod solution {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5000;
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
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
                let to_check = Day02Initial::new(input).solution_part1();

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
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = "ymdrchgpvwfloluktajxijsqb";
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
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
                let to_check = Day02Initial::new(input).solution_part2();

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
