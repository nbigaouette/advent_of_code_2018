use std::cmp;
use std::collections::HashSet;

use crate::{parse_input, AoC, Day03SolutionPart1, Day03SolutionPart2, Input};

use ndarray::Array2;

type Grid = Array2<Vec<usize>>;

#[derive(Debug)]
pub struct Day03PreparsedNdarray {
    input: Vec<Input>,
    grid_size_width: usize,
    grid_size_height: usize,
}

fn build_grid(input: &[Input], grid_size_width: usize, grid_size_height: usize) -> Grid {
    let mut grid = Array2::<Vec<usize>>::from_elem((grid_size_width, grid_size_height), Vec::new());

    for claim in input {
        for i in claim.left..(claim.left + claim.wide) {
            for j in claim.top..(claim.top + claim.tall) {
                grid[[i, j]].push(claim.id);
            }
        }
    }

    grid
}

impl<'a> AoC<'a> for Day03PreparsedNdarray {
    type SolutionPart1 = Day03SolutionPart1;
    type SolutionPart2 = Day03SolutionPart2;

    fn description(&self) -> &'static str {
        "Pre-parsed string and ndarray"
    }

    fn new(input: &'a str) -> Day03PreparsedNdarray {
        let input: Vec<_> = parse_input(input).collect();
        let grid_size_width = input.iter().fold(0, |acc, claim| {
            let width = claim.left + claim.wide;
            cmp::max(width, acc)
        });
        let grid_size_height = input.iter().fold(0, |acc, claim| {
            let height = claim.top + claim.tall;
            cmp::max(height, acc)
        });
        Day03PreparsedNdarray {
            input,
            grid_size_width,
            grid_size_height,
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let grid = build_grid(&self.input, self.grid_size_width, self.grid_size_height);

        grid.iter()
            .filter_map(|p| if p.len() >= 2 { Some(1) } else { None })
            .count()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let grid = build_grid(&self.input, self.grid_size_width, self.grid_size_height);

        let mut claims: HashSet<usize> = self.input.iter().map(|claim| claim.id).collect();

        for claim_ids in grid.iter() {
            if claim_ids.len() >= 2 {
                for claim_id in claim_ids {
                    claims.remove(claim_id);
                }
            }
        }

        assert_eq!(claims.len(), 1);

        *claims.iter().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day03PreparsedNdarray;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 100595;
                let to_check = Day03PreparsedNdarray::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03PreparsedNdarray;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03PreparsedNdarray::new(input).solution_part1();

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
            use super::super::super::Day03PreparsedNdarray;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 415;
                let to_check = Day03PreparsedNdarray::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day03PreparsedNdarray;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 3;
                let input = "#1 @ 1,3: 4x4
                             #2 @ 3,1: 4x4
                             #3 @ 5,5: 2x2";
                let to_check = Day03PreparsedNdarray::new(input).solution_part2();

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
