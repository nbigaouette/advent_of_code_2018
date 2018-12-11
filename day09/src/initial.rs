use std::ops::{Deref, DerefMut};

use crate::Input;
use crate::{parse_input, AoC, Day09SolutionPart1, Day09SolutionPart2};

#[derive(Debug)]
pub struct Day09Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day09Initial<'a> {
    type SolutionPart1 = Day09SolutionPart1;
    type SolutionPart2 = Day09SolutionPart2;

    fn description(&self) -> &'static str {
        "Double Linked List"
    }

    fn new(input: &'a str) -> Day09Initial<'_> {
        Day09Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1(self.input)
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        solution_part2(self.input)
    }
}

macro_rules! autoderef_newtype {
    ($newtype:ident, $prevtype:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $newtype($prevtype);
        impl Deref for $newtype {
            type Target = $prevtype;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $newtype {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

autoderef_newtype!(Idx, usize);
autoderef_newtype!(PlayerIdx, isize);
// autoderef_newtype!(PlayerScore, Day09SolutionPart1);
pub type PlayerScore = Day09SolutionPart1;

#[derive(Debug)]
pub struct DoubleLinkedList {
    data: Vec<DoubleLinkedListNode>,
    node_count: usize,
    first_node: Idx,
}

#[derive(Debug)]
pub struct DoubleLinkedListNode {
    idx: Idx,
    prev: Idx,
    next: Idx,
}

pub type Marble = DoubleLinkedListNode;

impl DoubleLinkedList {
    pub fn new(nb_nodes: usize) -> DoubleLinkedList {
        let mut data = Vec::with_capacity(nb_nodes);
        data.push(DoubleLinkedListNode {
            idx: Idx(0),
            prev: Idx(0),
            next: Idx(0),
        });
        DoubleLinkedList {
            data,
            first_node: Idx(0),
            node_count: 1,
        }
    }

    pub fn insert_after(&mut self, idx: &Idx, value: Idx) {
        self.insert_n_after(idx, value, 0);
    }

    pub fn insert_n_after(&mut self, idx: &Idx, value: Idx, n: usize) {
        let after_node_idx = self.iter_from(idx).cycle().nth(n).unwrap().idx.clone();

        let new_node_prev = after_node_idx.clone();
        let new_node_next = self.data[after_node_idx.0].next.clone();

        // Insert a new node in the vector
        let new_node = DoubleLinkedListNode {
            idx: value,
            prev: new_node_prev.clone(),
            next: new_node_next.clone(),
        };

        self.data[*new_node_prev].next = new_node.idx.clone();
        self.data[*new_node_next].prev = new_node.idx.clone();

        self.data.push(new_node);

        self.node_count += 1;
    }

    pub fn remove_nth_rev(&mut self, idx: &Idx, n: usize) -> Idx {
        let to_remove_idx = self.iter_rev_from(idx).nth(n).unwrap().idx.clone();
        let node_left_idx = self.data[*to_remove_idx].prev.clone();
        let node_right_idx = self.data[*to_remove_idx].next.clone();
        // Re-link left to right
        self.data[*node_left_idx].next = node_right_idx.clone();
        self.data[*node_right_idx].prev = node_left_idx.clone();

        self.node_count -= 1;

        to_remove_idx
    }

    pub fn iter(&self) -> impl Iterator<Item = &DoubleLinkedListNode> + Clone {
        self.iter_from(&self.first_node)
    }

    pub fn iter_from(&self, idx: &Idx) -> impl Iterator<Item = &DoubleLinkedListNode> + Clone {
        DoubleLinkedListIterator {
            data: &self.data,
            data_length: self.node_count,
            iterator_idx: idx.clone(),
            iterator_count: 0,
        }
    }

    pub fn iter_rev(&self) -> impl Iterator<Item = &DoubleLinkedListNode> + Clone {
        self.iter_rev_from(&self.first_node)
    }

    pub fn iter_rev_from(&self, idx: &Idx) -> impl Iterator<Item = &DoubleLinkedListNode> + Clone {
        DoubleLinkedListRevIterator {
            data: &self.data,
            data_length: self.node_count,
            iterator_idx: idx.clone(),
            iterator_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoubleLinkedListIterator<'a> {
    data: &'a Vec<DoubleLinkedListNode>,
    data_length: usize,
    iterator_idx: Idx,
    iterator_count: usize,
}

impl<'a> Iterator for DoubleLinkedListIterator<'a> {
    type Item = &'a DoubleLinkedListNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_count + 1 > self.data_length {
            None
        } else {
            let node = &self.data[*self.iterator_idx];
            self.iterator_count += 1;
            self.iterator_idx = node.next.clone();
            Some(node)
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoubleLinkedListRevIterator<'a> {
    data: &'a Vec<DoubleLinkedListNode>,
    data_length: usize,
    iterator_idx: Idx,
    iterator_count: usize,
}

impl<'a> Iterator for DoubleLinkedListRevIterator<'a> {
    type Item = &'a DoubleLinkedListNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_count + 1 > self.data_length {
            None
        } else {
            let node = &self.data[*self.iterator_idx];
            self.iterator_count += 1;
            self.iterator_idx = node.prev.clone();
            Some(node)
        }
    }
}

#[derive(Debug)]
struct Player {
    id: PlayerIdx,
    marbles: Vec<Idx>,
}

impl Player {
    fn score(&self) -> PlayerScore {
        self.marbles
            .iter()
            .map(|marble_idx| marble_idx.0 as PlayerScore)
            .sum()
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    marbles: DoubleLinkedList,
    current_player_idx: PlayerIdx,
    current_marble_idx: Idx,
    last_marble_played: Idx,
}

impl Game {
    pub fn new(nb_players: usize, nb_marbles: usize) -> Game {
        Game {
            players: (0..nb_players)
                .map(|i| Player {
                    id: PlayerIdx(i as isize),
                    marbles: Vec::new(),
                })
                .collect(),
            marbles: DoubleLinkedList::new(nb_marbles),
            current_player_idx: PlayerIdx(-1),
            current_marble_idx: Idx(0),
            last_marble_played: Idx(0),
        }
    }

    pub fn state(&self) -> String {
        let marbles_line = self
            .marbles
            .iter()
            .map(|marble| {
                if marble.idx == self.current_marble_idx {
                    format!("({})", *marble.idx)
                } else {
                    format!(" {} ", *marble.idx)
                }
            })
            .collect::<Vec<String>>()
            .concat();

        format!("[{}] {}", *self.current_player_idx + 1, marbles_line)
    }

    pub fn next_player_step(&mut self) {
        *self.last_marble_played += 1;
        let new_marble_idx = *self.last_marble_played;

        if new_marble_idx % 23 != 0 {
            self.marbles
                .insert_n_after(&self.current_marble_idx, Idx(new_marble_idx), 1);
            *self.current_marble_idx = new_marble_idx;
        } else {
            let player = &mut self.players[*self.current_player_idx as usize];
            player.marbles.push(Idx(self.marbles.data.len()));

            // Push a dummy node in the vector
            self.marbles.data.push(Marble {
                idx: Idx(99999),
                next: Idx(99999),
                prev: Idx(99999),
            });

            let removed_marble_idx = self.marbles.remove_nth_rev(&self.current_marble_idx, 7);
            player.marbles.push(removed_marble_idx);
            *self.current_marble_idx = *self
                .marbles
                .iter_rev_from(&self.current_marble_idx)
                .nth(6)
                .unwrap()
                .idx;
        }

        *self.current_player_idx = (*self.current_player_idx + 1) % self.players.len() as isize;
    }
}

fn solution_part1(input: &str) -> Day09SolutionPart1 {
    let Input {
        nb_players,
        last_marble_points,
    } = parse_input(input).unwrap();

    let mut game = Game::new(nb_players, last_marble_points);

    for _ in 0..last_marble_points {
        game.next_player_step();
    }

    game.players
        .iter()
        .map(|player| player.score())
        .max()
        .unwrap()
}

fn solution_part2(input: &str) -> Day09SolutionPart1 {
    let Input {
        nb_players,
        last_marble_points,
    } = parse_input(input).unwrap();

    let last_marble_points = 100 * last_marble_points;

    let mut game = Game::new(nb_players, last_marble_points);

    for _ in 0..last_marble_points {
        game.next_player_step();
    }

    game.players
        .iter()
        .map(|player| player.score())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day09Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 398502;
                let to_check = Day09Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day09Initial;
            use crate::{tests::init_logger, AoC};
            #[test]
            fn ex00() {
                init_logger();

                let expected = 32;
                let input = "9 players; last marble is worth 25 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01() {
                init_logger();

                let expected = 8317;
                let input = "10 players; last marble is worth 1618 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let expected = 146373;
                let input = "13 players; last marble is worth 7999 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let expected = 2764;
                let input = "17 players; last marble is worth 1104 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex04() {
                init_logger();

                let expected = 54718;
                let input = "21 players; last marble is worth 6111 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex05() {
                init_logger();

                let expected = 37305;
                let input = "30 players; last marble is worth 5807 points";
                let to_check = Day09Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

        }

        mod extra {
            use super::super::super::{parse_input, DoubleLinkedList, Game, Idx, Input};
            use crate::tests::init_logger;

            #[test]
            fn double_linked_list() {
                init_logger();

                let mut dll = DoubleLinkedList::new(4);
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0)]
                );

                dll.insert_after(&Idx(0), Idx(1));
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0), &Idx(1)]
                );

                dll.insert_after(&Idx(0), Idx(2));
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0), &Idx(2), &Idx(1)]
                );

                dll.insert_after(&Idx(1), Idx(3));
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0), &Idx(2), &Idx(1), &Idx(3)]
                );

                dll.insert_after(&Idx(2), Idx(4));
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0), &Idx(2), &Idx(4), &Idx(1), &Idx(3)]
                );

                dll.insert_n_after(&Idx(2), Idx(5), 0);
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![&Idx(0), &Idx(2), &Idx(5), &Idx(4), &Idx(1), &Idx(3)]
                );

                dll.insert_n_after(&Idx(2), Idx(6), 1);
                assert_eq!(
                    dll.iter().map(|node| &node.idx).collect::<Vec<_>>(),
                    vec![
                        &Idx(0),
                        &Idx(2),
                        &Idx(5),
                        &Idx(6),
                        &Idx(4),
                        &Idx(1),
                        &Idx(3)
                    ]
                );

                assert_eq!(dll.remove_nth_rev(&Idx(6), 3), Idx(0));
            }

            #[test]
            fn ex00_steps_states() {
                init_logger();

                let expected = r#"
[0] (0)
[1]  0 (1)
[2]  0 (2) 1
[3]  0  2  1 (3)
[4]  0 (4) 2  1  3
[5]  0  4  2 (5) 1  3
[6]  0  4  2  5  1 (6) 3
[7]  0  4  2  5  1  6  3 (7)
[8]  0 (8) 4  2  5  1  6  3  7
[9]  0  8  4 (9) 2  5  1  6  3  7
[1]  0  8  4  9  2 (10) 5  1  6  3  7
[2]  0  8  4  9  2  10  5 (11) 1  6  3  7
[3]  0  8  4  9  2  10  5  11  1 (12) 6  3  7
[4]  0  8  4  9  2  10  5  11  1  12  6 (13) 3  7
[5]  0  8  4  9  2  10  5  11  1  12  6  13  3 (14) 7
[6]  0  8  4  9  2  10  5  11  1  12  6  13  3  14  7 (15)
[7]  0 (16) 8  4  9  2  10  5  11  1  12  6  13  3  14  7  15
[8]  0  16  8 (17) 4  9  2  10  5  11  1  12  6  13  3  14  7  15
[9]  0  16  8  17  4 (18) 9  2  10  5  11  1  12  6  13  3  14  7  15
[1]  0  16  8  17  4  18  9 (19) 2  10  5  11  1  12  6  13  3  14  7  15
[2]  0  16  8  17  4  18  9  19  2 (20) 10  5  11  1  12  6  13  3  14  7  15
[3]  0  16  8  17  4  18  9  19  2  20  10 (21) 5  11  1  12  6  13  3  14  7  15
[4]  0  16  8  17  4  18  9  19  2  20  10  21  5 (22) 11  1  12  6  13  3  14  7  15
[5]  0  16  8  17  4  18 (19) 2  20  10  21  5  22  11  1  12  6  13  3  14  7  15
[6]  0  16  8  17  4  18  19  2 (24) 20  10  21  5  22  11  1  12  6  13  3  14  7  15
[7]  0  16  8  17  4  18  19  2  24  20 (25) 10  21  5  22  11  1  12  6  13  3  14  7  15"#
                    .trim();
                let input = "9 players; last marble is worth 25 points";
                let Input {
                    nb_players,
                    last_marble_points,
                } = parse_input(input).unwrap();

                let mut game = Game::new(nb_players, last_marble_points);
                let mut expected_lines_iter = expected.lines();
                assert_eq!(expected_lines_iter.next().unwrap().trim(), game.state());
                (1..26)
                    .map(|_| {
                        game.next_player_step();
                        game.state().trim().to_string()
                    })
                    .zip(expected_lines_iter)
                    .for_each(|(to_check, expected)| assert_eq!(to_check, expected));
            }

        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day09Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 3352920421;
                let to_check = Day09Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }
    }
}
