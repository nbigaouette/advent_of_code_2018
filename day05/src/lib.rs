//! # Day 05: Alchemical Reduction
//!
//! [Benchmarking report](../../../day05/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day05/target/criterion/day05_part1/report/index.html)
//! * [Part 2](../../../day05/target/criterion/day05_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! You've managed to sneak in to the prototype suit manufacturing lab. The Elves
//! are making decent progress, but are still struggling with the suit's size
//! reduction capabilities.
//!
//! While the very latest in 1518 alchemical technology might have solved their
//! problem eventually, you can do better. You scan the chemical composition of
//! the suit's material and discover that it is formed by extremely long
//! [polymers](https://en.wikipedia.org/wiki/Polymer) (one of which is
//! available as your puzzle input).
//!
//! The polymer is formed by smaller _units_ which, when triggered, react with
//! each other such that two adjacent units of the same type and opposite
//! polarity are destroyed. Units' types are represented by letters; units'
//! polarity is represented by capitalization. For instance, `r` and `R` are
//! units with the same type but opposite polarity, whereas `r` and `s` are
//! entirely different types and do not react.
//!
//! For example:
//!
//! * In `aA`, `a` and `A` react, leaving nothing behind.
//! * In `abBA`, `bB` destroys itself, leaving `aA`. As above, this then destroys
//!   itself, leaving nothing.
//! * In `abAB`, no two adjacent units are of the same type, and so nothing happens.
//! * In `aabAAB`, even though `aa` and `AA` are of the same type, their polarities
//!   match, and so nothing happens.
//!
//! Now, consider a larger example, `dabAcCaCBAcCcaDA`:
//!
//! ```text
//! dabAcCaCBAcCcaDA  The first 'cC' is removed.
//! dabAaCBAcCcaDA    This creates 'Aa', which is removed.
//! dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
//! dabCBAcaDA        No further actions can be taken.
//! ```
//!
//! After all possible reactions, the resulting polymer contains _10 units_.
//!
//! _How many units remain after fully reacting the polymer you scanned?_ (Note:
//! in this puzzle and others, the input is large; if you copy/paste your input,
//! make sure you get the whole thing.)
//!
//! ## Part Two
//!
//! Time to improve the polymer.
//!
//! One of the unit types is causing problems; it's preventing the polymer from
//! collapsing as much as it should. Your goal is to figure out which unit type
//! is causing the most problems, remove all instances of it (regardless of
//! polarity), fully react the remaining polymer, and measure its length.
//!
//! For example, again using the polymer `dabAcCaCBAcCcaDA` from above:
//!
//! * Removing all `A`/`a` units produces `dbcCCBcCcD`. Fully reacting this
//!   polymer produces `dbCBcD`, which has length 6.
//! * Removing all `B`/`b` units produces `daAcCaCAcCcaDA`. Fully reacting
//!   this polymer produces `daCAcaDA`, which has length 8.
//! * Removing all `C`/`c` units produces `dabAaBAaDA`. Fully reacting this
//!   polymer produces `daDA`, which has length 4.
//! * Removing all `D`/`d` units produces `abAcCaCBAcCcaA`. Fully reacting
//!   this polymer produces `abCBAc`, which has length 6.
//!
//! In this example, removing all `C`/`c` units was best, producing the answer _4_.
//!
//! _What is the length of the shortest polymer you can produce_ by removing all
//! units of exactly one type and fully reacting the result?
//!

// #[macro_use]
// extern crate log;
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;

extern crate rayon;

use std::fmt::Debug;

pub mod initial;
pub use initial::Day05Initial;

pub mod initial_parallel_part2;
pub use initial_parallel_part2::Day05InitialParallelPart2;

pub mod explicit_loop;
pub use explicit_loop::Day05ExplicitLoop;

pub mod iter_fold;
pub use iter_fold::Day05IteratorFold;

type Day05SolutionPart1 = usize;
type Day05SolutionPart2 = usize;

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

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day05SolutionPart1, SolutionPart2 = Day05SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![
            Box::new(Day05Initial::new(PUZZLE_INPUT)),
            // Box::new(Day05InitialParallelPart2::new(PUZZLE_INPUT)),
            // Box::new(Day05ExplicitLoop::new(PUZZLE_INPUT)),
            // Box::new(Day05IteratorFold::new(PUZZLE_INPUT)),
        ]
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

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
}
