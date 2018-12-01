//! # Day 01:
//!
//! ## Part One
//!
//!
//!
//! ## Part Two
//!
//!
//!

pub struct Solution {
    pub part1: usize,
    pub part2: usize,
}

pub fn aoc_day01(input: &str) -> Solution {
    unimplemented!()

    // Solution {
    //     part1: ,
    //     part2: ,
    // }
}

pub mod benchmark {
    pub const BENCHMARKING_INPUT: &str = "";
}

#[cfg(test)]
mod tests {
    mod aoc2018 {
        mod day01 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        unimplemented!();
                        // let expected = ;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        unimplemented!();
                        // let expected = ;
                        let input = "";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

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
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        unimplemented!();
                        // let expected = ;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        unimplemented!();
                        // let expected = ;
                        let input = "";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day01(input);

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
