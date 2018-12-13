//! # Day 12:
//!
//! [Benchmarking report](../../../day12/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day12/target/criterion/day12_part1/report/index.html)
//! * [Part 2](../../../day12/target/criterion/day12_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! The year 518 is significantly more underground than your history books implied.
//! Either that, or you've arrived in a vast cavern network under the North Pole.
//!
//! After exploring a little, you discover a long tunnel that contains a row of small
//! pots as far as you can see to your left and right. A few of them contain plants -
//! someone is trying to grow things in these geothermally-heated caves.
//!
//! The pots are numbered, with `0` in front of you. To the left, the pots are
//! numbered `-1`, `-2`, `-3`, and so on; to the right, `1`, `2`, `3`.... Your
//! puzzle input contains a list of pots from `0` to the right and whether they
//! do (`#`) or do not (`.`) currently contain a plant, the _initial state_.
//! (No other pots currently contain plants.) For example, an initial state
//! of `#..##....` indicates that pots `0`, `3`, and `4` currently contain plants.
//!
//! Your puzzle input also contains some notes you find on a nearby table: someone
//! has been trying to figure out how these plants _spread_ to nearby pots. Based
//! on the notes, for each generation of plants, a given pot has or does not have
//! a plant based on whether that pot (and the two pots on either side of it) had
//! a plant in the last generation. These are written as `LLCRR => N`, where `L`
//! are pots to the left, `C` is the current pot being considered, `R` are the pots
//! to the right, and `N` is whether the current pot will have a plant in the next
//! generation. For example:
//!
//! *   A note like `..#.. => .` means that a pot that contains a plant but with
//! no plants within two pots of it will not have a plant in it during the next
//! generation.
//! *   A note like `##.## => .` means that an empty pot with two plants on each
//! side of it will remain empty in the next generation.
//! *   A note like `.##.# => #` means that a pot has a plant in a given generation
//! if, in the previous generation, there were plants in that pot, the one immediately
//! to the left, and the one two pots to the right, but not in the ones immediately
//!  to the right and two to the left.
//!
//! It's not clear what these plants are for, but you're sure it's important, so you'd
//! like to make sure the current configuration of plants is sustainable by determining
//! what will happen after _`20` generations_.
//!
//! For example, given the following input:
//!
//! ```text
//! initial state: #..#.#..##......###...###
//!
//! ...## => #
//! ..#.. => #
//! .#... => #
//! .#.#. => #
//! .#.## => #
//! .##.. => #
//! .#### => #
//! #.#.# => #
//! #.### => #
//! ##.#. => #
//! ##.## => #
//! ###.. => #
//! ###.# => #
//! ####. => #
//! ```
//!
//! For brevity, in this example, only the combinations which do produce a plant are
//! listed. (Your input includes all possible combinations.) Then, the next 20
//! generations will look like this:
//!
//! ```text
//!                  1         2         3     
//!        0         0         0         0     
//!  0: ...#..#.#..##......###...###...........
//!  1: ...#...#....#.....#..#..#..#...........
//!  2: ...##..##...##....#..#..#..##..........
//!  3: ..#.#...#..#.#....#..#..#...#..........
//!  4: ...#.#..#...#.#...#..#..##..##.........
//!  5: ....#...##...#.#..#..#...#...#.........
//!  6: ....##.#.#....#...#..##..##..##........
//!  7: ...#..###.#...##..#...#...#...#........
//!  8: ...#....##.#.#.#..##..##..##..##.......
//!  9: ...##..#..#####....#...#...#...#.......
//! 10: ..#.#..#...#.##....##..##..##..##......
//! 11: ...#...##...#.#...#.#...#...#...#......
//! 12: ...##.#.#....#.#...#.#..##..##..##.....
//! 13: ..#..###.#....#.#...#....#...#...#.....
//! 14: ..#....##.#....#.#..##...##..##..##....
//! 15: ..##..#..#.#....#....#..#.#...#...#....
//! 16: .#.#..#...#.#...##...#...#.#..##..##...
//! 17: ..#...##...#.#.#.#...##...#....#...#...
//! 18: ..##.#.#....#####.#.#.#...##...##..##..
//! 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
//! 20: .#....##....#####...#######....#.#..##.
//! ```
//!
//! The generation is shown along the left, where `0` is the initial state. The pot
//! numbers are shown along the top, where `0` labels the center pot, negative-numbered
//! pots extend to the left, and positive pots extend toward the right. Remember, the
//! initial state begins at pot `0`, which is not the leftmost pot used in this example.
//!
//! After one generation, only seven plants remain. The one in pot `0` matched the rule
//! looking for `..#..`, the one in pot `4` matched the rule looking for `.#.#.`, pot
//!  `9` matched `.##..`, and so on.
//!
//! In this example, after 20 generations, the pots shown as `#` contain plants, the
//! furthest left of which is pot `-2`, and the furthest right of which is pot `34`.
//! Adding up all the numbers of plant-containing pots after the 20th generation
//! produces `_325_`.
//!
//! _After `20` generations, what is the sum of the numbers of all pots which
//! contain a plant?_
//!
//! ## Part Two
//!
//! You realize that 20 generations aren't enough. After all, these plants will need
//! to last another 1500 years to even reach your timeline, not to mention your future.
//!
//! _After fifty billion (`50000000000`) generations, what is the sum of the numbers
//! of all pots which contain a plant?_
//!

// #[macro_use]
// extern crate log;

use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod initial;
pub use crate::initial::Day12Initial;

type Result<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Day12SolutionPart1 = i64;
type Day12SolutionPart2 = i64;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Result<Self::SolutionPart1> {
        unimplemented!()
    }

    fn solution_part2(&self) -> Result<Self::SolutionPart2> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PotState {
    SomePlant,
    NoPlant,
}

impl std::fmt::Display for PotState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PotState::NoPlant => write!(f, "."),
            PotState::SomePlant => write!(f, "#"),
        }
    }
}

fn pot_slice_to_string(state: &[PotState]) -> String {
    state.iter().map(|pot| format!("{}", pot)).collect()
}

fn from_char(c: char) -> Result<PotState> {
    if c == '#' {
        Ok(PotState::SomePlant)
    } else if c == '.' {
        Ok(PotState::NoPlant)
    } else {
        err!("Cannot parse character: {}", c)
    }
}

#[derive(Debug, PartialEq)]
pub struct Note {
    neighborhood: [PotState; 5],
    result: PotState,
}

impl FromStr for Note {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Note> {
        let s = s.trim();
        let mut char_iter = s.chars();
        let left_1 = char_iter
            .next()
            .ok_or_else(|| format!("failed to take first char in '{}'", s))?;
        let left_0 = char_iter
            .next()
            .ok_or_else(|| format!("failed to take second char in '{}'", s))?;
        let center = char_iter
            .next()
            .ok_or_else(|| format!("failed to take third char in '{}'", s))?;
        let right_0 = char_iter
            .next()
            .ok_or_else(|| format!("failed to take fourth char in '{}'", s))?;
        let right_1 = char_iter
            .next()
            .ok_or_else(|| format!("failed to take fifth char in '{}'", s))?;
        let result = char_iter
            .skip(4)
            .next()
            .ok_or_else(|| format!("failed to take final char in '{}'", s))?;

        Ok(Note {
            neighborhood: [
                from_char(left_1)?,
                from_char(left_0)?,
                from_char(center)?,
                from_char(right_0)?,
                from_char(right_1)?,
            ],
            result: from_char(result)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct InitialState {
    state: Vec<PotState>,
}

impl FromStr for InitialState {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<InitialState> {
        let state: Vec<_> = s
            .trim()
            .chars()
            .map(|c| from_char(c))
            .collect::<Result<Vec<_>>>()?;
        Ok(InitialState { state })
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    input: &'a str,
    initial_state: InitialState,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Result<Input<'a>> {
        let (_header, input) = input.trim().split_at(15);
        let initial_state = input
            .lines()
            .nth(0)
            .ok_or_else(|| format!("failed to take first line in '{}'", input))?
            .parse()?;

        Ok(Input {
            initial_state,
            input,
        })
    }

    pub fn iter_notes(&self) -> impl Iterator<Item = Note> + 'a {
        self.input
            .lines()
            .skip(2)
            .filter_map(|line| line.parse().ok())
    }
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day12SolutionPart1, SolutionPart2 = Day12SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day12Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use crate::*;

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String> {
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
    fn parse_note() {
        let parsed: Note = "..#.. => .".parse().unwrap();
        let expected = Note {
            neighborhood: [
                PotState::NoPlant,
                PotState::NoPlant,
                PotState::SomePlant,
                PotState::NoPlant,
                PotState::NoPlant,
            ],
            result: PotState::NoPlant,
        };
        assert_eq!(parsed, expected);

        let parsed: Note = "##.## => .".parse().unwrap();
        let expected = Note {
            neighborhood: [
                PotState::SomePlant,
                PotState::SomePlant,
                PotState::NoPlant,
                PotState::SomePlant,
                PotState::SomePlant,
            ],
            result: PotState::NoPlant,
        };
        assert_eq!(parsed, expected);

        let parsed: Note = ".##.# => #".parse().unwrap();
        let expected = Note {
            neighborhood: [
                PotState::NoPlant,
                PotState::SomePlant,
                PotState::SomePlant,
                PotState::NoPlant,
                PotState::SomePlant,
            ],
            result: PotState::SomePlant,
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_state() {
        let parsed: InitialState = "#..##....".parse().unwrap();
        let expected = InitialState {
            state: vec![
                PotState::SomePlant,
                PotState::NoPlant,
                PotState::NoPlant,
                PotState::SomePlant,
                PotState::SomePlant,
                PotState::NoPlant,
                PotState::NoPlant,
                PotState::NoPlant,
                PotState::NoPlant,
            ],
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse() {
        let input = "initial state: #..#.#..##......###...###

                     ...## => #
                     ..#.. => #
                     .#... => #
                     .#.#. => #
                     .#.## => #
                     .##.. => #
                     .#### => #";
        //  #.#.# => #
        //  #.### => #
        //  ##.#. => #
        //  ##.## => #
        //  ###.. => #
        //  ###.# => #
        //  ####. => #

        let parsed_input = Input::new(&input).unwrap();

        assert_eq!(
            parsed_input.initial_state,
            InitialState {
                state: vec![
                    PotState::SomePlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::SomePlant,
                    PotState::NoPlant,
                    PotState::SomePlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::SomePlant,
                    PotState::SomePlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::SomePlant,
                    PotState::SomePlant,
                    PotState::SomePlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::NoPlant,
                    PotState::SomePlant,
                    PotState::SomePlant,
                    PotState::SomePlant,
                ],
            }
        );

        let notes: Vec<Note> = parsed_input.iter_notes().collect();
        assert_eq!(
            notes,
            vec![
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::NoPlant,
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::SomePlant
                    ],
                    // neighborhood_right: [PotState::SomePlant, PotState::SomePlant],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::NoPlant,
                        PotState::NoPlant
                    ],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::NoPlant,
                        PotState::NoPlant,
                        PotState::NoPlant
                    ],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::NoPlant
                    ],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::SomePlant
                    ],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::SomePlant,
                        PotState::NoPlant,
                        PotState::NoPlant
                    ],
                    result: PotState::SomePlant,
                },
                Note {
                    neighborhood: [
                        PotState::NoPlant,
                        PotState::SomePlant,
                        PotState::SomePlant,
                        PotState::SomePlant,
                        PotState::SomePlant
                    ],
                    result: PotState::SomePlant,
                },
            ]
        );
    }
}
