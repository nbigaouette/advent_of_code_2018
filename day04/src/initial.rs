use std::collections::HashMap;

use crate::{parse_input, AoC, Day04SolutionPart1, Day04SolutionPart2};
use crate::{Day, GuardId};

#[derive(Debug)]
pub struct Day04Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day04Initial<'a> {
    type SolutionPart1 = Day04SolutionPart1;
    type SolutionPart2 = Day04SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day04Initial {
        Day04Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let parsed: Vec<Day> = parse_input(self.input);

        let hours_slept: Vec<(GuardId, Vec<i64>)> = parsed
            .iter()
            .map(|day| {
                let hours_slept_int: Vec<i64> = day
                    .sleeping
                    .iter()
                    .map(|&is_sleeping| if is_sleeping { 1 } else { 0 })
                    .collect();
                (day.id, hours_slept_int)
            }).collect();

        let mut total_hours_slept: HashMap<GuardId, i64> = HashMap::new();
        for (id, hours) in &hours_slept {
            let hours_for_id = total_hours_slept.entry(*id).or_insert(0);
            *hours_for_id += hours.iter().sum::<i64>();
        }

        let (most_lazy_id, _most_lazy_hours_count) = total_hours_slept
            .iter()
            .max_by_key(|(_id, hours_slept)| *hours_slept)
            .unwrap();

        let most_lazy_guard_hours: Vec<Vec<i64>> = hours_slept
            .into_iter()
            .filter_map(|(id, hours)| {
                if id == *most_lazy_id {
                    Some(hours)
                } else {
                    None
                }
            }).collect();

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

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 11367;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
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
                let to_check = Day04Initial::new(input).solution_part1();

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
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(expected, to_check);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day04Initial::new(input).solution_part2();

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
