//! # Day 02: Inventory Management System
//!
//! [Benchmarking report](../../../day02/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day02/target/criterion/day02_part1/report/index.html)
//! * [Part 2](../../../day02/target/criterion/day02_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! You stop falling through time, catch your breath, and check the screen on the device.
//! "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet
//! 83N10." You made it! Now, to find those anomalies.
//!
//! Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either.
//! But now that so many people have chimneys, maybe he could sneak in that way?" Another
//! voice responds, "Actually, we've been working on a new kind of _suit_ that would let
//! him fit through tight spaces like that. But, I heard that a few days ago, they lost
//! the prototype fabric, the design plans, everything! Nobody on the team can even seem
//! to remember important details of the project!"
//!
//! "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd
//! be stored together, so the box IDs should be similar. Too bad it would take forever
//! to search the warehouse for _two similar box IDs_..." They walk too far away to hear
//! any more.
//!
//! Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you
//! could cause if you were discovered - and use your fancy wrist device to quickly scan
//! every box and produce a list of the likely candidates (your puzzle input).
//!
//! To make sure you didn't miss any, you scan the likely candidate boxes again, counting
//! the number that have an ID containing _exactly two of any letter_ and then separately
//! counting those with _exactly three of any letter_. You can multiply those two counts
//! together to get a rudimentary [checksum](https://en.wikipedia.org/wiki/Checksum) and
//! compare it to what your device predicts.
//!
//! For example, if you see the following box IDs:
//!
//! * `abcdef` contains no letters that appear exactly two or three times.
//! * `bababc` contains two `a` and three `b`, so it counts for both.
//! * `abbcde` contains two `b`, but no letter appears exactly three times.
//! * `abcccd` contains three `c`, but no letter appears exactly two times.
//! * `aabcdd` contains two `a` and two `d`, but it only counts once.
//! * `abcdee` contains two `e`.
//! * `ababab` contains three `a` and three `b`, but it only counts once.
//!
//! Of these box IDs, four of them contain a letter which appears exactly twice, and three
//! of them contain a letter which appears exactly three times. Multiplying these together
//! produces a checksum of `4 * 3 = 12`.
//!
//! _What is the checksum_ for your list of box IDs?
//!
//! ## Part Two
//!
//! Confident that your list of box IDs is complete, you're ready to find the boxes full of
//! prototype fabric.
//!
//! The boxes will have IDs which differ by exactly one character at the same position in
//! both strings. For example, given the following box IDs:
//!
//! ```
//! abcde
//! fghij
//! klmno
//! pqrst
//! fguij
//! axcye
//! wvxyz
//! ```   
//!
//! The IDs `abcde` and `axcye` are close, but they differ by two characters (the second
//! and fourth). However, the IDs `fghij` and `fguij` differ by exactly one character,
//! the third (`h` and `u`). Those must be the correct boxes.
//!
//! _What letters are common between the two correct box IDs?_ (In the example above, this
//! is found by removing the differing character from either ID, producing `fgij`.)
//!

#[macro_use]
extern crate log;
extern crate strsim;
#[macro_use]
extern crate itertools;

use std::fmt::Debug;

pub mod initial;
pub use initial::Day02Initial;

pub mod bound_check_elision;
pub use bound_check_elision::Day02BoundCheckElision;

pub trait AoC<'a>: Debug {
    type Solution1;
    type Solution2;
    type Parsed;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn parsed(&self) -> Self::Parsed {
        unimplemented!()
    }

    fn solution_part1(&self) -> Self::Solution1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::Solution2 {
        unimplemented!()
    }
}

type Day02SolutionPart1 = i64;
type Day02SolutionPart2 = String;
type Day02Parsed<'a> = Box<Iterator<Item = &'a str> + 'a>;

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<
                    'a,
                    Solution1 = Day02SolutionPart1,
                    Solution2 = Day02SolutionPart2,
                    Parsed = Day02Parsed<'a>,
                > + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![
            Box::new(Day02Initial::new(PUZZLE_INPUT)),
            Box::new(Day02BoundCheckElision::new(PUZZLE_INPUT)),
        ]
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|line| line.trim())
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    use crate::parse_input;

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            }).unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        let input = "abcdef
                     bababc
                     abbcde
                     abcccd
                     aabcdd
                     abcdee
                     ababab";
        let parsed: Vec<_> = parse_input(input).collect();
        assert_eq!(
            parsed,
            vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",]
        );
    }
}
