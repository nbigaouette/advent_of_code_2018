// use std::collections::VecDeque;
use slice_deque::SliceDeque;

use crate::{pot_slice_to_string, Input, Note, PotState, Result};
use crate::{AoC, Day12SolutionPart1, Day12SolutionPart2};

#[derive(Debug)]
pub struct Day12Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day12Initial<'a> {
    type SolutionPart1 = Day12SolutionPart1;
    type SolutionPart2 = Day12SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day12Initial<'_> {
        Day12Initial { input }
    }

    fn solution_part1(&self) -> Result<Self::SolutionPart1> {
        let nb_generations = 20;
        solution_part1(&self.input, nb_generations)
    }

    fn solution_part2(&self) -> Result<Self::SolutionPart2> {
        let nb_generations = 50_000_000_000;
        solution_part1(&self.input, nb_generations)
    }
}

struct PleaseExtend {
    left: bool,
    right: bool,
}

fn part1_step(
    state: &mut SliceDeque<PotState>,
    next_state: &mut SliceDeque<PotState>,
    notes: &[Note],
    i0: &mut i64,
) -> bool {
    let mut please_extend = PleaseExtend {
        left: false,
        right: false,
    };

    for i in 2..(state.len() - 2) {
        // NOTE: The example input does not contain the rules "killing" the
        //       plant. Keep a flag to kill a plant.
        let mut match_found = false;

        for note in notes {
            // Compare state with all notes
            if &state[(i - 2)..(i + 2 + 1)] == &note.neighborhood[..] {
                next_state[i] = note.result.clone();
                if i == 2 && next_state[i] == PotState::SomePlant {
                    please_extend.left = true;
                }
                if i == next_state.len() - 1 - 2 && next_state[i] == PotState::SomePlant {
                    please_extend.right = true;
                }
                match_found = true;
                break;
            }
        }
        // Kill the plant
        if !match_found {
            next_state[i] = PotState::NoPlant;
        }
    }

    if please_extend.left {
        state.push_front(PotState::NoPlant);
        state.push_front(PotState::NoPlant);
        next_state.push_front(PotState::NoPlant);
        next_state.push_front(PotState::NoPlant);

        *i0 -= 2;
    }
    if please_extend.right {
        state.push_back(PotState::NoPlant);
        state.push_back(PotState::NoPlant);
        next_state.push_back(PotState::NoPlant);
        next_state.push_back(PotState::NoPlant);
    }

    if next_state[1..] == state[0..state.len() - 1] {
        // println!("Found repeating pattern:");
        std::mem::swap(state, next_state);
        return true;
    }

    const TRUNCATE_LEN: usize = 10;
    // Truncate front
    if next_state
        .iter()
        .take(TRUNCATE_LEN + 3)
        .all(|pot| pot == &PotState::NoPlant)
    {
        state.truncate_front(state.len() - TRUNCATE_LEN);
        next_state.truncate_front(next_state.len() - TRUNCATE_LEN);
        *i0 += TRUNCATE_LEN as i64;
    }
    // Truncate back
    if next_state
        .iter()
        .skip(next_state.len() - (TRUNCATE_LEN + 3))
        .all(|pot| pot == &PotState::NoPlant)
    {
        state.truncate_back(state.len() - TRUNCATE_LEN);
        next_state.truncate_back(next_state.len() - TRUNCATE_LEN);
    }

    std::mem::swap(state, next_state);

    // dpne == false
    false
}

fn solution_part1(input: &str, nb_generations: usize) -> Result<Day12SolutionPart1> {
    let parsed_input = Input::new(input)?;
    let notes: Vec<Note> = parsed_input.iter_notes().collect();

    let mut state: SliceDeque<PotState> = parsed_input
        .initial_state
        .state
        .iter()
        .map(|pot| pot.clone())
        .collect();

    // Add three empty pots at beginning and end so checking the first and last
    // pots in the initial data does not overflows.
    state.push_front(PotState::NoPlant);
    state.push_front(PotState::NoPlant);
    state.push_front(PotState::NoPlant);
    state.push_back(PotState::NoPlant);
    state.push_back(PotState::NoPlant);
    state.push_back(PotState::NoPlant);

    // We pushed 3 empty pots at the beginning.
    let mut i0 = -3;

    let mut next_state = state.clone();

    // println!("{:2}: {}", 0, pot_slice_to_string(state.as_slice()));

    let mut early_break = None;
    for _gen in 1..=nb_generations {
        let done = part1_step(&mut state, &mut next_state, &notes, &mut i0);

        // println!("{:2}: {}", _gen, pot_slice_to_string(next_state.as_slice()));

        if done {
            early_break = Some(_gen);
            break;
        }
    }

    if let Some(early_break) = early_break {
        Ok(state
            .iter()
            .enumerate()
            .filter(|(_i, c)| **c == PotState::SomePlant)
            .map(|(i, _c)| (i + (nb_generations - early_break)) as i64 + i0)
            .sum())
    } else {
        Ok(state
            .iter()
            .enumerate()
            .filter(|(_i, pot)| pot == &&PotState::SomePlant)
            .map(|(i, _pot)| i as i64 + i0)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day12Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 2040;
                let to_check = Day12Initial::new(PUZZLE_INPUT).solution_part1().unwrap();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex00_steps() {
                init_logger();

                let input = "initial state: #..#.#..##......###...###

                             ...## => #
                             ..#.. => #
                             .#... => #
                             .#.#. => #
                             .#.## => #
                             .##.. => #
                             .#### => #
                             #.#.# => #
                             #.### => #
                             ##.#. => #
                             ##.## => #
                             ###.. => #
                             ###.# => #
                             ####. => #";
                let parsed_input = Input::new(input).unwrap();
                let notes: Vec<Note> = parsed_input.iter_notes().collect();

                let mut state: SliceDeque<PotState> = parsed_input
                    .initial_state
                    .state
                    .iter()
                    .map(|pot| pot.clone())
                    .collect();
                state.push_front(PotState::NoPlant);
                state.push_front(PotState::NoPlant);
                state.push_front(PotState::NoPlant);
                state.push_back(PotState::NoPlant);
                state.push_back(PotState::NoPlant);
                state.push_back(PotState::NoPlant);

                let mut i0 = -3;

                let mut next_state = state.clone();

                assert_eq!(
                    pot_slice_to_string(next_state.as_slice()),
                    "...#..#.#..##......###...###..."
                );
                assert_eq!(i0, -3);

                part1_step(&mut state, &mut next_state, &notes, &mut i0);
                assert_eq!(
                    pot_slice_to_string(state.as_slice()),
                    "...#...#....#.....#..#..#..#..."
                );
                assert_eq!(i0, -3);

                part1_step(&mut state, &mut next_state, &notes, &mut i0);
                assert_eq!(
                    pot_slice_to_string(state.as_slice()),
                    "...##..##...##....#..#..#..##...."
                );
                assert_eq!(i0, -3);

                part1_step(&mut state, &mut next_state, &notes, &mut i0);
                assert_eq!(
                    pot_slice_to_string(state.as_slice()),
                    "....#.#...#..#.#....#..#..#...#...."
                );
                assert_eq!(i0, -5);

                part1_step(&mut state, &mut next_state, &notes, &mut i0);
                assert_eq!(
                    pot_slice_to_string(state.as_slice()),
                    ".....#.#..#...#.#...#..#..##..##..."
                );
                assert_eq!(i0, -5);

                part1_step(&mut state, &mut next_state, &notes, &mut i0);
                assert_eq!(
                    pot_slice_to_string(state.as_slice()),
                    "......#...##...#.#..#..#...#...#..."
                );
                assert_eq!(i0, -5);
            }

            #[test]
            fn ex00() {
                init_logger();

                let input = "initial state: #..#.#..##......###...###

                             ...## => #
                             ..#.. => #
                             .#... => #
                             .#.#. => #
                             .#.## => #
                             .##.. => #
                             .#### => #
                             #.#.# => #
                             #.### => #
                             ##.#. => #
                             ##.## => #
                             ###.. => #
                             ###.# => #
                             ####. => #";

                let to_check = solution_part1(input, 0).unwrap();
                let expected = 145;
                assert_eq!(to_check, expected);

                let to_check = solution_part1(input, 1).unwrap();
                let expected = 91;
                assert_eq!(to_check, expected);

                let to_check = solution_part1(input, 20).unwrap();
                let expected = 325;
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01() {
                init_logger();

                let expected = 325;
                let input = "initial state: #..#.#..##......###...###

                             ...## => #
                             ..#.. => #
                             .#... => #
                             .#.#. => #
                             .#.## => #
                             .##.. => #
                             .#### => #
                             #.#.# => #
                             #.### => #
                             ##.#. => #
                             ##.## => #
                             ###.. => #
                             ###.# => #
                             ####. => #";
                let to_check = Day12Initial::new(input).solution_part1().unwrap();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day12Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day12Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 1700000000011;
                let to_check = Day12Initial::new(PUZZLE_INPUT).solution_part2().unwrap();

                assert_eq!(to_check, expected);
            }
        }
    }
}
