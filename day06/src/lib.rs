//! # Day 06: Chronal Coordinates
//!
//! [Benchmarking report](../../../day06/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day06/target/criterion/day06_part1/report/index.html)
//! * [Part 2](../../../day06/target/criterion/day06_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! The device on your wrist beeps several times, and once again you feel like you're falling.
//!
//! "Situation critical," the device announces. "Destination indeterminate. Chronal
//! interference detected. Please specify new target coordinates."
//!
//! The device then produces a list of coordinates (your puzzle input). Are they places it
//! thinks are safe or dangerous? It recommends you check manual page 729. The Elves did
//! not give you a manual.
//!
//! _If they're dangerous,_ maybe you can minimize the danger by finding the coordinate
//! that gives the largest distance from the other points.
//!
//! Using only the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry),
//! determine the _area_ around each coordinate by counting the number of
//! [integer](https://en.wikipedia.org/wiki/Integer) X,Y locations that are _closest_
//! to that coordinate (and aren't _tied in distance_ to any other coordinate).
//!
//! Your goal is to find the size of the _largest area_ that isn't infinite. For
//! example, consider the following list of coordinates:
//!
//! ```text
//! 1, 1
//! 1, 6
//! 8, 3
//! 3, 4
//! 5, 5
//! 8, 9
//! ```
//!
//! If we name these coordinates `A` through `F`, we can draw them on a grid,
//! putting `0,0` at the top left:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ........C.
//! ...D......
//! .....E....
//! .B........
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! This view is partial - the actual grid extends infinitely in all directions.
//! Using the Manhattan distance, each location's closest coordinate can be determined,
//! shown here in lowercase:
//!
//! ```text
//! aaaaa.cccc
//! aAaaa.cccc
//! aaaddecccc
//! aadddeccCc
//! ..dDdeeccc
//! bb.deEeecc
//! bBb.eeee..
//! bbb.eeefff
//! bbb.eeffff
//! bbb.ffffFf
//! ```
//!
//! Locations shown as `.` are equally far from two or more coordinates, and so they
//! don't count as being closest to any.
//!
//! In this example, the areas of coordinates A, B, C, and F are infinite - while not
//! shown here, their areas extend forever outside the visible grid. However, the
//! areas of coordinates D and E are finite: D is closest to 9 locations, and E is
//! closest to 17 (both including the coordinate's location itself). Therefore, in
//! this example, the size of the largest area is _17_.
//!
//! _What is the size of the largest area_ that isn't infinite?
//!
//! ## Part Two
//!
//! On the other hand, _if the coordinates are safe_, maybe the best you can do is try
//! to find a _region_ near as many coordinates as possible.
//!
//! For example, suppose you want the sum of the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) to all of the coordinates to be
//! _less than 32_. For each location, add up the distances to all of the given
//! coordinates; if the total of those distances is less than 32, that location is
//! within the desired region. Using the same coordinates as above, the resulting
//! region looks like this:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ...#$#..C.
//! ..#D###...
//! ..###E#...
//! .B.###....
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! In particular, consider the highlighted location `4,3` located at the top middle
//! of the region. Its calculation is as follows, where `abs()` is the
//! [absolute value](https://en.wikipedia.org/wiki/Absolute_value) function:
//!
//! * Distance to coordinate A: `abs(4-1) + abs(3-1) =  5`
//! * Distance to coordinate B: `abs(4-1) + abs(3-6) =  6`
//! * Distance to coordinate C: `abs(4-8) + abs(3-3) =  4`
//! * Distance to coordinate D: `abs(4-3) + abs(3-4) =  2`
//! * Distance to coordinate E: `abs(4-5) + abs(3-5) =  3`
//! * Distance to coordinate F: `abs(4-8) + abs(3-9) = 10`
//! * Total distance: `5 + 6 + 4 + 2 + 3 + 10 = 30`
//!
//! Because the total distance to all coordinates (`30`) is less than 32, the location
//! is _within_ the region.
//!
//! This region, which also includes coordinates D and E, has a total size of _16_.
//!
//! Your actual region will need to be much larger than this example, though, instead
//! including all locations with a total distance of less than _10000_.
//!
//! _What is the size of the region containing all locations which have a total
//! distance to all given coordinates of less than 10000?_
//!

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
// #[macro_use]
// extern crate log;
extern crate itertools;
#[macro_use]
extern crate ndarray;

use std::fmt::Debug;

use std::error::Error;
use std::result;
use std::str::FromStr;

pub mod initial;
pub use initial::Day06Initial;

type Day06SolutionPart1 = i64;
type Day06SolutionPart2 = i64;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn ij(&self, grid_offset: &[i64; 2]) -> [usize; 2] {
        [
            (self.y - grid_offset[1]) as usize,
            (self.x - grid_offset[0]) as usize,
        ]
    }
}

type Result<T> = result::Result<T, Box<Error>>;

impl FromStr for Position {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Position> {
        let mut pos_str = s.split(", ");

        Ok(Position {
            x: pos_str
                .next()
                .ok_or_else(|| format!("failed to get first element of '{}'", s))?
                .parse()?,
            y: pos_str
                .next()
                .ok_or_else(|| format!("failed to get second element of '{}'", s))?
                .parse()?,
        })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Position>> {
    input.lines().map(|line| line.trim().parse()).collect()
}

pub fn manhattan_distance(pos1: &Position, pos2: &Position) -> i64 {
    let delta_x = (pos1.x - pos2.x).abs();
    let delta_y = (pos1.y - pos2.y).abs();

    delta_x + delta_y
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day06SolutionPart1, SolutionPart2 = Day06SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day06Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    use crate::{manhattan_distance, parse_input, Position};

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            })
            .unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        assert_eq!("1, 1".parse::<Position>().unwrap(), Position { x: 1, y: 1 });
        assert_eq!("1, 2".parse::<Position>().unwrap(), Position { x: 1, y: 2 });
        assert_eq!("2, 1".parse::<Position>().unwrap(), Position { x: 2, y: 1 });

        let input = "1, 1
                     1, 6
                     8, 3
                     3, 4
                     5, 5
                     8, 9";
        assert_eq!(
            parse_input(&input).unwrap(),
            vec![
                Position { x: 1, y: 1 },
                Position { x: 1, y: 6 },
                Position { x: 8, y: 3 },
                Position { x: 3, y: 4 },
                Position { x: 5, y: 5 },
                Position { x: 8, y: 9 },
            ]
        )
    }

    #[test]
    fn calculate_manhattan_distance() {
        assert_eq!(
            manhattan_distance(&Position { x: 1, y: 1 }, &Position { x: 1, y: 6 }),
            5
        );
        assert_eq!(
            manhattan_distance(&Position { x: 1, y: 1 }, &Position { x: 8, y: 3 }),
            9
        );
    }
}
