//! # Day 04:
//!
//! [Benchmarking report](../../../day04/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day04/target/criterion/day04_part1/report/index.html)
//! * [Part 2](../../../day04/target/criterion/day04_part2/report/index.html)
//!
//!
//! ## Part One
//!
//!
//!
//! ## Part Two
//!
//!
//!

// #[macro_use]
// extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fmt::Debug;

use regex::Regex;

pub mod initial;
pub use initial::Day04Initial;

type Day04SolutionPart1 = i64;
type Day04SolutionPart2 = i64;

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

type MidnightHour = Vec<bool>;
type GuardId = i64;

#[derive(Debug, Eq, PartialEq)]
pub struct Day {
    year: u64,
    month: u64,
    day: u64,
    id: GuardId,
    sleeping: MidnightHour,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    BeginsShift(GuardId),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    action: Action,
}

pub fn parse_line(input: &str) -> Line {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(
            r"(?x)\[
                (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})
                \s+
                (?P<hour>\d{2}):(?P<minute>\d{2})
                \]
                \s+
                (?P<action>.*)"
        ).unwrap();
        static ref RE_ACTION_BEGINS_SHIFT: Regex =
            Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
        static ref RE_ACTION_FALLS_ASLEEP: Regex = Regex::new("falls asleep").unwrap();
        static ref RE_ACTION_WAKES_UP: Regex = Regex::new("wakes up").unwrap();
    }
    let caps = RE_LINE.captures(input).unwrap();

    let action = {
        let action_str = &caps["action"];
        if RE_ACTION_WAKES_UP.is_match(action_str) {
            Action::WakesUp
        } else if RE_ACTION_FALLS_ASLEEP.is_match(action_str) {
            Action::FallsAsleep
        } else {
            let caps_begins_shift = RE_ACTION_BEGINS_SHIFT.captures(action_str).unwrap();
            Action::BeginsShift(caps_begins_shift["id"].parse().unwrap())
        }
    };

    Line {
        year: caps["year"].parse().unwrap(),
        month: caps["month"].parse().unwrap(),
        day: caps["day"].parse().unwrap(),
        hour: caps["hour"].parse().unwrap(),
        minute: caps["minute"].parse().unwrap(),
        action,
    }
}

pub fn parse_input(input: &str) -> Vec<Day> {
    // Build a vector of `Line`, sorted by timestamp
    const MINUTES_PER_HOUR: u64 = 60;
    const HOURS_PER_DAY: u64 = 24;
    const DAYS_PER_MONTH: u64 = 31;
    const MONTHS_PER_YEAR: u64 = 12;

    const MINUTES_PER_DAY: u64 = HOURS_PER_DAY * MINUTES_PER_HOUR;
    const MINUTES_PER_MONTH: u64 = DAYS_PER_MONTH * MINUTES_PER_DAY;
    const MINUTES_PER_YEAR: u64 = MONTHS_PER_YEAR * MINUTES_PER_MONTH;

    // Sort lines by date, normalizing the guards beginning of shifts
    let lines: Vec<Line> = {
        let mut tmp_lines: Vec<Line> = input.lines().map(|line| parse_line(line)).collect();

        // // Normalize shifts beginning at midnight
        // tmp_lines.iter_mut().for_each(|line| {
        //     match line.action {
        //         Action::BeginsShift(..) => {
        //             // NOTE: The day can "overflow" a real date (say 32nd), but we don't care.
        //             let norm_hour = line.hour * 100 + line.minute;
        //             if 2300 <= norm_hour {
        //                 line.day += 1
        //             }
        //             line.hour = 0;
        //             line.minute = 0;
        //         }
        //         _ => { /* */ }
        //     }
        // });

        // Sort by date
        tmp_lines.as_mut_slice().sort_by_key(|line| {
            line.year * MINUTES_PER_YEAR
                + line.month * MINUTES_PER_MONTH
                + line.day * MINUTES_PER_DAY
                + line.hour * MINUTES_PER_HOUR
                + line.minute
        });

        tmp_lines
    };

    // Loop for every day
    // debug!("lines: {:#?}", lines);

    let mut lines_iter = lines.iter().peekable();
    let mut days: Vec<Day> = Vec::new();
    while !lines_iter.peek().is_none() {
        let mut day = {
            let first_line = lines_iter.next().unwrap();
            // First line should always be a guard shift beginning
            let first_line_id = match first_line.action {
                Action::BeginsShift(id) => id,
                _ => panic!("First line should be a 'Guard #XX begins shift'"),
            };

            Day {
                year: first_line.year,
                month: first_line.month,
                day: first_line.day,
                id: first_line_id,
                sleeping: vec![false; MINUTES_PER_HOUR as usize],
            }
        };

        let mut per_minute_sleep_actions: Vec<i8> = vec![0; MINUTES_PER_HOUR as usize];
        loop {
            match lines_iter.peek() {
                // Stop when no more lines
                None => {
                    break;
                }
                Some(line) => {
                    // if day.day != line.day {
                    //     // We've hit a next day. Break the loop.
                    //     warn!("Next day. Breaking.");
                    //     break;
                    // }
                    match line.action {
                        Action::BeginsShift(..) => {
                            // We've hit a shift change. Break the loop
                            break;
                        }
                        _ => { /* */ }
                    }
                }
            }
            // Safe to unwrap since we peeked for end
            let line = lines_iter.next().unwrap();
            match line.action {
                Action::FallsAsleep => {
                    per_minute_sleep_actions[line.minute as usize] = -1;
                }
                Action::WakesUp => {
                    per_minute_sleep_actions[line.minute as usize] = 1;
                }
                Action::BeginsShift(..) => unreachable!(),
            }
        }

        day.sleeping = per_minute_sleep_actions
            .iter()
            .scan(0, |cum_sum, action| {
                *cum_sum += action;
                Some(*cum_sum)
            }).map(|action| action < 0)
            .collect();

        days.push(day);
    }

    days
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day04SolutionPart1, SolutionPart2 = Day04SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day04Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    use crate::{parse_input, parse_line};

    use crate::{Action, Day, Line};

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
    fn parse_lines_guard_begins_shift() {
        init_logger();

        assert_eq!(
            parse_line("[1518-11-01 00:00] Guard #10 begins shift"),
            Line {
                year: 1518,
                month: 11,
                day: 01,
                hour: 0,
                minute: 0,
                action: Action::BeginsShift(10),
            }
        );
    }

    #[test]
    fn parse_lines_falls_asleep() {
        init_logger();

        assert_eq!(
            parse_line("[1518-11-01 00:05] falls asleep"),
            Line {
                year: 1518,
                month: 11,
                day: 01,
                hour: 0,
                minute: 5,
                action: Action::FallsAsleep,
            }
        );
    }

    #[test]
    fn parse_lines_wakes_up() {
        init_logger();

        assert_eq!(
            parse_line("[1518-11-01 00:25] wakes up"),
            Line {
                year: 1518,
                month: 11,
                day: 01,
                hour: 0,
                minute: 25,
                action: Action::WakesUp,
            }
        );
    }

    #[test]
    fn parse() {
        init_logger();

        let input = "[1518-11-01 00:00] Guard #10 begins shift
                     [1518-11-01 00:05] falls asleep
                     [1518-11-01 00:25] wakes up
                     [1518-11-01 00:30] falls asleep
                     [1518-11-01 00:55] wakes up
                     [1518-11-01 23:58] Guard #99 begins shift
                     [1518-11-02 00:40] falls asleep
                     [1518-11-02 00:50] wakes up
                     [1518-11-03 00:05] Guard #10 begins shift
                     [1518-11-03 00:24] falls asleep
                     [1518-11-03 00:29] wakes up
                     [1518-11-04 00:02] Guard #99 begins shift
                     [1518-11-04 00:36] falls asleep
                     [1518-11-04 00:46] wakes up
                     [1518-11-05 00:03] Guard #99 begins shift
                     [1518-11-05 00:45] falls asleep
                     [1518-11-05 00:55] wakes up";
        let parsed: Vec<Day> = parse_input(input);
        assert_eq!(
            parsed,
            vec![
                Day {
                    year: 1518,
                    month: 11,
                    day: 1,
                    id: 10,
                    sleeping: vec![
                        false, false, false, false, false, true, true, true, true, true, true,
                        true, true, true, true, true, true, true, true, true, true, true, true,
                        true, true, false, false, false, false, false, true, true, true, true,
                        true, true, true, true, true, true, true, true, true, true, true, true,
                        true, true, true, true, true, true, true, true, true, false, false, false,
                        false, false
                    ]
                },
                Day {
                    year: 1518,
                    month: 11,
                    day: 1,
                    id: 99,
                    sleeping: vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false, true,
                        true, true, true, true, true, true, true, true, true, false, false, false,
                        false, false, false, false, false, false, false
                    ]
                },
                Day {
                    year: 1518,
                    month: 11,
                    day: 3,
                    id: 10,
                    sleeping: vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, true, true, true, true, true, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false
                    ]
                },
                Day {
                    year: 1518,
                    month: 11,
                    day: 4,
                    id: 99,
                    sleeping: vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, true, true, true, true, true,
                        true, true, true, true, true, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false
                    ]
                },
                Day {
                    year: 1518,
                    month: 11,
                    day: 5,
                    id: 99,
                    sleeping: vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, true, true, true, true, true, true,
                        true, true, true, true, false, false, false, false, false
                    ]
                }
            ]
        );
    }
}
