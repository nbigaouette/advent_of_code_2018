//! # Day 09:
//!
//! [Benchmarking report](../../../day09/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day09/target/criterion/day09_part1/report/index.html)
//! * [Part 2](../../../day09/target/criterion/day09_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! You talk to the Elves while you wait for your navigation system to initialize. To
//! pass the time, they introduce you to their favorite
//! [marble](https://en.wikipedia.org/wiki/Marble_(toy)) game.
//!
//! The Elves play this game by taking turns arranging the marbles in a _circle_
//! according to very particular rules. The marbles are numbered starting with
//! `0` and increasing by `1` until every marble has a number.
//!
//! First, the marble numbered `0` is placed in the circle. At this point, while it
//! contains only a single marble, it is still a circle: the marble is both clockwise
//! from itself and counter-clockwise from itself. This marble is designated the
//! _current marble_.
//!
//! Then, each Elf takes a turn placing the _lowest-numbered remaining marble_ into
//! the circle between the marbles that are `1` and `2` marbles _clockwise_ of the
//! current marble. (When the circle is large enough, this means that there is one
//! marble between the marble that was just placed and the current marble.) The
//! marble that was just placed then becomes the _current marble_.
//!
//! However, if the marble that is about to be placed has a number which is a multiple
//! of `23`, _something entirely different happens_. First, the current player keeps
//! the marble they would have placed, adding it to their _score_. In addition, the
//! marble `7` marbles _counter-clockwise_ from the current marble is _removed_ from
//! the circle and _also_ added to the current player's score. The marble located
//! immediately _clockwise_ of the marble that was removed becomes the new _current marble_.
//!
//! For example, suppose there are 9 players. After the marble with value `0` is
//! placed in the middle, each player (shown in square brackets) takes a turn. The
//! result of each of those turns would produce circles of marbles like this, where
//! clockwise is to the right and the resulting current marble is in parentheses:
//!
//! ```text
//! [-] (0)
//! [1]  0 (1)
//! [2]  0 (2) 1
//! [3]  0  2  1 (3)
//! [4]  0 (4) 2  1  3
//! [5]  0  4  2 (5) 1  3
//! [6]  0  4  2  5  1 (6) 3
//! [7]  0  4  2  5  1  6  3 (7)
//! [8]  0 (8) 4  2  5  1  6  3  7
//! [9]  0  8  4 (9) 2  5  1  6  3  7
//! [1]  0  8  4  9  2(10) 5  1  6  3  7
//! [2]  0  8  4  9  2 10  5(11) 1  6  3  7
//! [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
//! [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
//! [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
//! [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
//! [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
//! [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
//! [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
//! [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
//! [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
//! [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
//! ```
//!
//! The goal is to be the _player with the highest score_ after the last marble is
//! used up. Assuming the example above ends after the marble numbered `25`, the
//! winning score is `23+9=_32_` (because player 5 kept marble `23` and removed
//! marble `9`, while no other player got any points in this very short example
//! game).
//!
//! Here are a few more examples:
//!
//! * `10` players; last marble is worth `1618` points: high score is _`8317`_
//! * `13` players; last marble is worth `7999` points: high score is _`146373`_
//! * `17` players; last marble is worth `1104` points: high score is _`2764`_
//! * `21` players; last marble is worth `6111` points: high score is _`54718`_
//! * `30` players; last marble is worth `5807` points: high score is _`37305`_
//!
//! _What is the winning Elf's score?_
//!
//! ## Part Two
//!
//! Amused by the speed of your answer, the Elves are curious:
//!
//! _What would the new winning Elf's score be if the number of the last
//! marble were 100 times larger?_
//!

#[macro_use]
extern crate log;

use std::fmt::Debug;

#[macro_use]
extern crate pretty_assertions;
use failure::{format_err, Error};

pub mod initial;
pub use crate::initial::Day09Initial;

type Day09SolutionPart1 = u64;
type Day09SolutionPart2 = u64;

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

#[derive(Debug, PartialEq)]
pub struct Input {
    nb_players: usize,
    last_marble_points: usize,
}

pub fn parse_input(input: &str) -> Result<Input, Error> {
    let mut words = input.split_whitespace();
    let nb_players = words
        .next()
        .ok_or_else(|| format_err!("Cannot extract number of players"))?
        .parse()?;
    let last_marble_points = words
        .nth(5)
        .ok_or_else(|| format_err!("Cannot extract last marble's worth"))?
        .parse()?;

    Ok(Input {
        nb_players,
        last_marble_points,
    })
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day09SolutionPart1, SolutionPart2 = Day09SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day09Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use crate::{parse_input, Input};

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
        let input = "9 players; last marble is worth 25 points";
        let expected = Input {
            nb_players: 9,
            last_marble_points: 25,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "10 players; last marble is worth 1618 points";
        let expected = Input {
            nb_players: 10,
            last_marble_points: 1618,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "13 players; last marble is worth 7999 points";
        let expected = Input {
            nb_players: 13,
            last_marble_points: 7999,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "17 players; last marble is worth 1104 points";
        let expected = Input {
            nb_players: 17,
            last_marble_points: 1104,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "21 players; last marble is worth 6111 points";
        let expected = Input {
            nb_players: 21,
            last_marble_points: 6111,
        };
        assert_eq!(parse_input(input).unwrap(), expected);

        let input = "30 players; last marble is worth 5807 points";
        let expected = Input {
            nb_players: 30,
            last_marble_points: 5807,
        };
        assert_eq!(parse_input(input).unwrap(), expected);
    }
}
