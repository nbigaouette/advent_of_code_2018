use std::collections::HashMap;

use crate::{parse_input, AoC, Day03Parsed, Day03SolutionPart1, Day03SolutionPart2, Input};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    i: u64,
    j: u64,
}

#[derive(Debug)]
pub struct Day03Initial<'a> {
    input: &'a str,

    seen: HashMap<Coord, u64>,
    // seen: HashMap<Coord, Vec<u64>>,
}

impl<'a> AoC<'a> for Day03Initial<'a> {
    type SolutionPart1 = Day03SolutionPart1;
    type SolutionPart2 = Day03SolutionPart2;
    type Parsed = Day03Parsed<'a>;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day03Initial {
        Day03Initial {
            input,
            seen: HashMap::new(),
        }
    }

    // fn parsed(&self) -> Self::Parsed {
    //     Box::new(parse_input(self.input))
    // }

    fn solution_part1(&mut self) -> Self::SolutionPart1 {
        let mut count = 0;
        for claim in parse_input(self.input).map(|i| Input::from(i)) {
            for i in claim.left..(claim.left + claim.wide) {
                for j in claim.top..(claim.top + claim.tall) {
                    let coord = Coord { i, j };
                    let point = self.seen.entry(coord).or_insert(0);
                    *point += 1;
                    if *point == 2 {
                        count += 1;
                        // println!(
                        //     "Coord: {:?}  point: {}  count: {}",
                        //     Coord { i, j },
                        //     point,
                        //     count
                        // );
                    }
                    // let point = self.seen.entry(coord).or_insert(vec![]);
                    // point.push(claim.id);
                    // if point.len() == 2 {
                    //     count += 1;
                    // }
                }
            }
        }
        count
    }

    fn solution_part2(&mut self) -> Self::SolutionPart2 {
        // Calculate the hashmap
        // let _ = self.solution_part1();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn bugged_solution_01() {
                init_logger();

                let not_expected = 128626;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part1();

                assert_ne!(not_expected, to_check);
            }

            #[test]
            fn solution() {
                init_logger();

                let expected = 100595;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03Initial::new(input).solution_part1();

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
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 0;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 3;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03Initial::new(input).solution_part2();

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
