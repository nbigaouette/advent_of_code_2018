use std::collections::HashSet;

use itertools::iproduct;
use ndarray::Array2;

use crate::{manhattan_distance, Position, Result};
use crate::{parse_input, AoC, Day06SolutionPart1, Day06SolutionPart2};

const SAFE_CELL_MARKER: CoordinateId = CoordinateId::max_value();

#[derive(Debug)]
pub struct Day06Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day06Initial<'a> {
    type SolutionPart1 = Day06SolutionPart1;
    type SolutionPart2 = Day06SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day06Initial {
        Day06Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1(self.input).unwrap()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        solution_part2(self.input, 10000).unwrap()
    }
}

pub fn solution_part1(input: &str) -> Result<Day06SolutionPart1> {
    // Find the size of the circle surrounding _all_ points.
    // This will be the max iteration since the two most distant
    // point's growing neighborhood will have reach the other point.
    let positions = parse_input(&input)?;
    let bounding_box = find_bounding_box(&positions)?;
    let (mut grid, grid_offset) = new_grid(&bounding_box);

    for (id, pos) in positions.iter().enumerate() {
        grid[pos.ij(&grid_offset)] = Cell::C(id as CoordinateId);
    }

    // Fill grid
    fill_grid_part1(&mut grid, &bounding_box, &positions);

    let ids_at_boundary = boundaries(&grid);

    let max_count = positions
        .iter()
        .enumerate()
        .filter(|(id, _pos)| !ids_at_boundary.contains(&(*id as u32)))
        .map(|(id, _pos)| {
            grid.iter()
                .filter(move |cell| match cell {
                    Cell::C(cell_id) => cell_id == &(id as u32),
                    _ => false,
                })
                .count()
        })
        .max_by_key(|count| *count)
        .unwrap();

    Ok(max_count as i64)
}

pub fn solution_part2(input: &str, max_distance: i64) -> Result<Day06SolutionPart2> {
    // Find the size of the circle surrounding _all_ points.
    // This will be the max iteration since the two most distant
    // point's growing neighborhood will have reach the other point.
    let positions = parse_input(&input)?;
    let bounding_box = find_bounding_box(&positions)?;
    let (mut grid, grid_offset) = new_grid(&bounding_box);

    for (id, pos) in positions.iter().enumerate() {
        grid[pos.ij(&grid_offset)] = Cell::C(id as CoordinateId);
    }

    // Fill grid
    fill_grid_part2(&mut grid, &bounding_box, &positions, max_distance);

    Ok(grid
        .iter()
        .filter(|&cell| *cell == Cell::C(SAFE_CELL_MARKER))
        .count() as Day06SolutionPart2)
}

pub type CoordinateId = u32;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    U___,
    C(CoordinateId),
    T___,
}
pub type Grid = Array2<Cell>;

pub fn boundaries(grid: &Grid) -> HashSet<CoordinateId> {
    let row_first: Vec<_> = grid.slice(s![0, ..]).into_iter().collect();
    let row_last: Vec<_> = grid.slice(s![-1, ..]).into_iter().collect();
    let col_first: Vec<_> = grid.slice(s![.., 0]).into_iter().collect();
    let col_last: Vec<_> = grid.slice(s![.., -1]).into_iter().collect();
    let boundary_elements_dup = [row_first, row_last, col_first, col_last].concat();
    let mut boundary_elements = HashSet::new();
    for element in boundary_elements_dup {
        match element {
            Cell::C(id) => {
                boundary_elements.insert(*id);
            }
            _ => { /* */ }
        }
    }
    boundary_elements
}

fn fill_grid_part1(grid: &mut Grid, bounding_box: &BoundingBox, positions: &[Position]) {
    let mut closest_id: Vec<CoordinateId> = Vec::with_capacity(64);
    for (j_i64, i_i64) in iproduct!(0..bounding_box.height(), 0..bounding_box.width()) {
        let i = i_i64 as usize;
        let j = j_i64 as usize;

        closest_id.clear();
        let mut min_dist = i64::max_value();

        let cell = &mut grid[[j, i]];

        let cell_pos = Position {
            x: i_i64 + bounding_box.xmin,
            y: j_i64 + bounding_box.ymin,
        };
        if *cell == Cell::U___ {
            for (id, pos) in positions.iter().enumerate() {
                let distance = manhattan_distance(pos, &cell_pos);
                if min_dist == distance {
                    closest_id.push(id as u32);
                } else if distance < min_dist {
                    closest_id.clear();
                    closest_id.push(id as u32);
                    min_dist = distance;
                }
            }
            assert!(!closest_id.is_empty());

            if closest_id.len() == 1 {
                *cell = Cell::C(closest_id[0]);
            } else {
                *cell = Cell::T___;
            }
        }
    }
}

fn fill_grid_part2(
    grid: &mut Grid,
    bounding_box: &BoundingBox,
    positions: &[Position],
    max_distance_sum: i64,
) {
    for (j_i64, i_i64) in iproduct!(0..bounding_box.height(), 0..bounding_box.width()) {
        let i = i_i64 as usize;
        let j = j_i64 as usize;

        let cell = &mut grid[[j, i]];

        let cell_pos = Position {
            x: i_i64 + bounding_box.xmin,
            y: j_i64 + bounding_box.ymin,
        };
        let distances_sum: i64 = positions
            .iter()
            .map(|pos| manhattan_distance(pos, &cell_pos))
            .sum();
        if distances_sum < max_distance_sum {
            *cell = Cell::C(SAFE_CELL_MARKER);
        }
    }
}

pub fn new_grid(bounding_box: &BoundingBox) -> (Grid, [i64; 2]) {
    let grid_offset = [bounding_box.xmin, bounding_box.ymin];

    (
        Grid::from_elem(
            (
                bounding_box.height() as usize,
                bounding_box.width() as usize,
            ),
            Cell::U___,
        ),
        grid_offset,
    )
}

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl BoundingBox {
    pub fn width(&self) -> i64 {
        self.xmax - self.xmin + 1
    }
    pub fn height(&self) -> i64 {
        self.ymax - self.ymin + 1
    }
}

fn find_bounding_box(input: &[Position]) -> Result<BoundingBox> {
    let xmin = input
        .iter()
        .min_by_key(|pos| pos.x)
        .ok_or_else(|| format!("Can't find minimum x value"))?
        .x;
    let xmax = input
        .iter()
        .max_by_key(|pos| pos.x)
        .ok_or_else(|| format!("Can't find maximum x value"))?
        .x;

    let ymin = input
        .iter()
        .min_by_key(|pos| pos.y)
        .ok_or_else(|| format!("Can't find minimum y value"))?
        .y;
    let ymax = input
        .iter()
        .max_by_key(|pos| pos.y)
        .ok_or_else(|| format!("Can't find maximum y value"))?
        .y;

    Ok(BoundingBox {
        xmin,
        xmax,
        ymin,
        ymax,
    })
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 3290;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 17;
                let input = "1, 1
                             1, 6
                             8, 3
                             3, 4
                             5, 5
                             8, 9";
                let to_check = Day06Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod extra {
            use super::super::super::{
                fill_grid_part1, find_bounding_box, new_grid, BoundingBox, Cell,
            };
            use crate::{parse_input, tests::init_logger};
            use ndarray::arr2;

            #[test]
            fn bounding_box() {
                init_logger();
                let input = "1, 1
                             1, 6
                             8, 3
                             3, 4
                             5, 5
                             8, 9";
                let parsed = parse_input(&input).unwrap();
                let bounding_box = find_bounding_box(&parsed).unwrap();
                assert_eq!(
                    bounding_box,
                    BoundingBox {
                        xmin: 1,
                        xmax: 8,
                        ymin: 1,
                        ymax: 9
                    }
                );
            }

            #[test]
            fn grid_filling() {
                init_logger();
                let input = "1, 1
                             1, 6
                             8, 3
                             3, 4
                             5, 5
                             8, 9";
                let positions = parse_input(&input).unwrap();
                let bounding_box = find_bounding_box(&positions).unwrap();
                let (mut grid, grid_offset) = new_grid(&bounding_box);
                for (id, pos) in positions.iter().enumerate() {
                    grid[pos.ij(&grid_offset)] = Cell::C(id as u32);
                }
                let expected_initial = arr2(&[
                    [
                        Cell::C(0),
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::C(2),
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::C(3),
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::C(4),
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::C(1),
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                    ],
                    [
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::U___,
                        Cell::C(5),
                    ],
                ]);
                assert_eq!(grid, expected_initial);
                fill_grid_part1(&mut grid, &bounding_box, &positions);
                let expected_filed = arr2(&[
                    [
                        Cell::C(0),
                        Cell::C(0),
                        Cell::C(0),
                        Cell::C(0),
                        Cell::T___,
                        Cell::C(2),
                        Cell::C(2),
                        Cell::C(2),
                    ],
                    [
                        Cell::C(0),
                        Cell::C(0),
                        Cell::C(3),
                        Cell::C(3),
                        Cell::C(4),
                        Cell::C(2),
                        Cell::C(2),
                        Cell::C(2),
                    ],
                    [
                        Cell::C(0),
                        Cell::C(3),
                        Cell::C(3),
                        Cell::C(3),
                        Cell::C(4),
                        Cell::C(2),
                        Cell::C(2),
                        Cell::C(2),
                    ],
                    [
                        Cell::T___,
                        Cell::C(3),
                        Cell::C(3),
                        Cell::C(3),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(2),
                        Cell::C(2),
                    ],
                    [
                        Cell::C(1),
                        Cell::T___,
                        Cell::C(3),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(2),
                    ],
                    [
                        Cell::C(1),
                        Cell::C(1),
                        Cell::T___,
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::T___,
                    ],
                    [
                        Cell::C(1),
                        Cell::C(1),
                        Cell::T___,
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(5),
                        Cell::C(5),
                    ],
                    [
                        Cell::C(1),
                        Cell::C(1),
                        Cell::T___,
                        Cell::C(4),
                        Cell::C(4),
                        Cell::C(5),
                        Cell::C(5),
                        Cell::C(5),
                    ],
                    [
                        Cell::C(1),
                        Cell::C(1),
                        Cell::T___,
                        Cell::C(5),
                        Cell::C(5),
                        Cell::C(5),
                        Cell::C(5),
                        Cell::C(5),
                    ],
                ]);
                assert_eq!(grid, expected_filed);
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 45602;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            // use super::super::super::Day06Initial;
            use super::super::super::solution_part2;

            use crate::tests::init_logger;

            #[test]
            fn ex01() {
                init_logger();

                let expected = 16;
                let input = "1, 1
                             1, 6
                             8, 3
                             3, 4
                             5, 5
                             8, 9";
                // let to_check = Day06Initial::new(input).solution_part2();
                let to_check = solution_part2(input, 32).unwrap();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */

    }
}
