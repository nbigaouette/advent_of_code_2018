use std::collections::HashMap;

use crate::GuardId;
use crate::{parse_input, AoC, Day04SolutionPart1, Day04SolutionPart2};

#[derive(Debug)]
pub struct Day04PreParsedFull {
    input: Vec<(GuardId, Vec<i64>)>,
}

impl<'a> AoC<'a> for Day04PreParsedFull {
    type SolutionPart1 = Day04SolutionPart1;
    type SolutionPart2 = Day04SolutionPart2;

    fn description(&self) -> &'static str {
        "Pre-parse full"
    }

    fn new(input: &'a str) -> Day04PreParsedFull {
        Day04PreParsedFull {
            input: parse_input(input)
                .iter()
                .map(|day| {
                    let hours_slept_int: Vec<i64> = day
                        .sleeping
                        .iter()
                        .map(|&is_sleeping| if is_sleeping { 1 } else { 0 })
                        .collect();
                    (day.id, hours_slept_int)
                }).collect(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let mut total_hours_slept: HashMap<GuardId, i64> = HashMap::new();
        for (id, hours) in &self.input {
            let hours_for_id = total_hours_slept.entry(*id).or_insert(0);
            *hours_for_id += hours.iter().sum::<i64>();
        }

        let (most_lazy_id, _most_lazy_hours_count) = total_hours_slept
            .iter()
            .max_by_key(|(_id, hours_slept)| *hours_slept)
            .unwrap();

        let most_lazy_guard_hours: Vec<Vec<i64>> = self
            .input
            .iter()
            .filter_map(|(id, hours)| {
                if id == most_lazy_id {
                    Some(hours)
                } else {
                    None
                }
            }).cloned()
            .collect();

        let most_lazy_guard_count_per_minute =
            most_lazy_guard_hours.iter().fold(vec![0; 60], |acc, day| {
                // Sum the two vectors element-wise
                acc.iter().zip(day.iter()).map(|(l, r)| l + r).collect()
            });
        let most_lazy_minute = most_lazy_guard_count_per_minute
            .iter()
            .enumerate()
            .max_by_key(|(_minute, &sleeping_hour)| sleeping_hour)
            .map(|(minute, _sleeping_hour)| minute as i64)
            .unwrap();

        most_lazy_id * most_lazy_minute
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let mut guards_sleep_pattern: HashMap<GuardId, Vec<i64>> = HashMap::new();
        for (id, hours) in &self.input {
            let guard_sleep_pattern = guards_sleep_pattern
                .entry(*id)
                .or_insert_with(|| vec![0; 60]);
            // Sum the guard's sleep pattern with the one from the vector
            for (l, r) in guard_sleep_pattern.iter_mut().zip(hours.iter()) {
                *l += r;
            }
        }

        // Find the most lazy guard
        let (most_lazy_id, most_lazy_pattern) = guards_sleep_pattern
            .iter()
            .max_by_key(|(_id, pattern)| pattern.iter().max())
            .unwrap();

        // Find the most lazy minute
        let (most_lazy_minute, _most_lazy_minute_count) = most_lazy_pattern
            .iter()
            .enumerate()
            .max_by_key(|(_minute, &slept)| slept)
            .unwrap();

        (most_lazy_minute as i64) * most_lazy_id
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day04PreParsedFull;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 11367;
                let to_check = Day04PreParsedFull::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04PreParsedFull;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 240;
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
                let to_check = Day04PreParsedFull::new(input).solution_part1();

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
            use super::super::super::Day04PreParsedFull;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 36896;
                let to_check = Day04PreParsedFull::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04PreParsedFull;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4455;
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
                let to_check = Day04PreParsedFull::new(input).solution_part2();

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
