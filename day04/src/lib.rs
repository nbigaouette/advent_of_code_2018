//! # Day 04: Repose Record
//!
//! [Benchmarking report](../../../day04/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day04/target/criterion/day04_part1/report/index.html)
//! * [Part 2](../../../day04/target/criterion/day04_part2/report/index.html)
//!
//!
//! ## Part One
//!
//! You've sneaked into another supply closet - this time, it's across from the prototype
//! suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but
//! there's a guard stationed outside the lab, so this is as close as you can safely get.
//!
//! As you search the closet for anything that might help, you discover that you're not
//! the first person to want to sneak in. Covering the walls, someone has spent an
//! hour starting every midnight for the past few months secretly observing this guard
//! post! They've been writing down the ID of _the one guard on duty that night_ - the
//! Elves seem to have decided that one guard was enough for the overnight shift - as
//! well as when they fall asleep or wake up while at their post (your puzzle input).
//!
//! For example, consider the following records, which have already been organized
//! into chronological order:
//!
//! ```text
//! [1518-11-01 00:00] Guard #10 begins shift
//! [1518-11-01 00:05] falls asleep
//! [1518-11-01 00:25] wakes up
//! [1518-11-01 00:30] falls asleep
//! [1518-11-01 00:55] wakes up
//! [1518-11-01 23:58] Guard #99 begins shift
//! [1518-11-02 00:40] falls asleep
//! [1518-11-02 00:50] wakes up
//! [1518-11-03 00:05] Guard #10 begins shift
//! [1518-11-03 00:24] falls asleep
//! [1518-11-03 00:29] wakes up
//! [1518-11-04 00:02] Guard #99 begins shift
//! [1518-11-04 00:36] falls asleep
//! [1518-11-04 00:46] wakes up
//! [1518-11-05 00:03] Guard #99 begins shift
//! [1518-11-05 00:45] falls asleep
//! [1518-11-05 00:55] wakes up
//! ```
//!
//! Timestamps are written using `year-month-day hour:minute` format. The guard falling
//! asleep or waking up is always the one whose shift most recently started. Because all
//! asleep/awake times are during the midnight hour (`00:00` - `00:59`), only the minute
//! portion (`00` - `59`) is relevant for those events.
//!
//! Visually, these records show that the guards are asleep at these times:
//!
//! ```text
//! Date   ID   Minute
//!             000000000011111111112222222222333333333344444444445555555555
//!             012345678901234567890123456789012345678901234567890123456789
//! 11-01  #10  .....####################.....#########################.....
//! 11-02  #99  ........................................##########..........
//! 11-03  #10  ........................#####...............................
//! 11-04  #99  ....................................##########..............
//! 11-05  #99  .............................................##########.....
//! ```
//!
//! The columns are Date, which shows the month-day portion of the relevant day;
//! ID, which shows the guard on duty that day; and Minute, which shows the minutes
//! during which the guard was asleep within the midnight hour. (The Minute column's
//! header shows the minute's ten's digit in the first row and the one's digit in
//! the second row.) Awake is shown as `.`, and asleep is shown as `#`.
//!
//! Note that guards count as asleep on the minute they fall asleep, and they
//! count as awake on the minute they wake up. For example, because Guard #10
//! wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
//!
//! If you can figure out the guard most likely to be asleep at a specific time,
//! you might be able to trick that guard into working tonight so you can have
//! the best chance of sneaking in. You have two strategies for choosing the best
//! guard/minute combination.
//!
//! _Strategy 1:_ Find the guard that has the most minutes asleep. What minute
//! does that guard spend asleep the most?
//!
//! In the example above, Guard #10 spent the most minutes asleep, a total of 50
//! minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
//! (10+10+10). Guard #_10_ was asleep most during minute _24_ (on two days,
//! whereas any other minute the guard was asleep was only seen on one day).
//!
//! While this example listed the entries in chronological order, your entries
//! are in the order you found them. You'll need to organize them before they
//! can be analyzed.
//!
//! _What is the ID of the guard you chose multiplied by the minute you chose?_
//! (In the above example, the answer would be `10 * 24 = 240`.)
//!
//!
//! ## Part Two
//!
//! _Strategy 2:_ Of all guards, which guard is most frequently asleep on the same minute?
//!
//! In the example above, Guard #_99_ spent minute _45_ asleep more than any other
//! guard or minute - three times in total. (In all other cases, any guard spent any
//! minute asleep at most twice.)
//!
//! _What is the ID of the guard you chose multiplied by the minute you chose?_ (In
//! the above example, the answer would be `99 * 45 = 4455`.)
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

pub mod preparsed;
pub use preparsed::Day04PreParsed;

pub mod preparsed_full;
pub use preparsed_full::Day04PreParsedFull;

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
    }
    let caps = RE_LINE.captures(input).unwrap();

    let action = {
        let action_str = &caps["action"];
        if action_str.contains("wakes up") {
            Action::WakesUp
        } else if action_str.contains("falls asleep") {
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
    while lines_iter.peek().is_some() {
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
                    if let Action::BeginsShift(..) = line.action {
                        // We've hit a shift change. Break the loop
                        break;
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
        vec![
            Box::new(Day04Initial::new(PUZZLE_INPUT)),
            Box::new(Day04PreParsed::new(PUZZLE_INPUT)),
            Box::new(Day04PreParsedFull::new(PUZZLE_INPUT)),
        ]
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
