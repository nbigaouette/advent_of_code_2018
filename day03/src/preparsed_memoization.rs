use std::collections::{HashMap, HashSet};

use crate::{parse_input, AoC, Day03SolutionPart1, Day03SolutionPart2, Input};

#[derive(Debug)]
pub struct Day03PreparsedMemoization {
    input: Vec<Input>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    i: usize,
    j: usize,
}

fn solution(
    input: &[Input],
) -> (
    Day03SolutionPart1,
    HashMap<Coord, Vec<usize>>,
    HashSet<usize>,
) {
    let mut count = 0;
    let mut seen = HashMap::new();
    let mut overlap_ids = HashSet::new();
    for claim in input {
        for i in claim.left..(claim.left + claim.wide) {
            for j in claim.top..(claim.top + claim.tall) {
                let coord = Coord { i, j };
                let ids_at_coord = seen.entry(coord).or_insert_with(Vec::new);
                ids_at_coord.push(claim.id);
                if ids_at_coord.len() == 2 {
                    count += 1;
                }
                if ids_at_coord.len() >= 2 {
                    for id_at_coord in ids_at_coord {
                        overlap_ids.insert(*id_at_coord);
                    }
                }
            }
        }
    }
    (count, seen, overlap_ids)
}

impl<'a> AoC<'a> for Day03PreparsedMemoization {
    type SolutionPart1 = Day03SolutionPart1;
    type SolutionPart2 = Day03SolutionPart2;

    fn description(&self) -> &'static str {
        "Pre-parsed string and memoization"
    }

    fn new(input: &'a str) -> Day03PreparsedMemoization {
        Day03PreparsedMemoization {
            input: parse_input(input).collect(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let (count, _seen, _overlap_ids) = solution(&self.input);
        count
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let (_count, _seen, overlap_ids) = solution(&self.input);

        let mut claims: HashSet<usize> = self.input.iter().map(|claim| claim.id).collect();

        for overlapping_id in overlap_ids {
            claims.remove(&overlapping_id);
        }

        assert_eq!(claims.len(), 1);

        *claims.iter().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day03PreparsedMemoization;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 100595;
                let to_check = Day03PreparsedMemoization::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03PreparsedMemoization;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03PreparsedMemoization::new(input).solution_part1();

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
            use super::super::super::Day03PreparsedMemoization;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 415;
                let to_check = Day03PreparsedMemoization::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03PreparsedMemoization;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 3;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03PreparsedMemoization::new(input).solution_part2();

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
