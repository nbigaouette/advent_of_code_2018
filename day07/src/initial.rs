use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use petgraph::visit::IntoNodeReferences;

use crate::{parse_input, AoC, Day07SolutionPart1, Day07SolutionPart2, Result};

use crate::{build_graph, Graph, GraphIdx, GraphNode};

#[derive(Debug)]
pub struct Day07Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day07Initial<'a> {
    type SolutionPart1 = Day07SolutionPart1;
    type SolutionPart2 = Day07SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day07Initial<'_> {
        Day07Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        solution_part1(self.input).unwrap()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let nb_workers = 5;
        let step_baseline = 60;
        solution_part2(self.input, nb_workers, step_baseline).unwrap()
    }
}

#[derive(Debug, Eq)]
struct GraphNodeAndIdx {
    node: GraphNode,
    idx: GraphIdx,
}

impl GraphNodeAndIdx {
    pub fn new(node: GraphNode, idx: GraphIdx) -> GraphNodeAndIdx {
        GraphNodeAndIdx { node, idx }
    }
}

// NOTE: To perform a min-heap (instead of max-heap), we need to
// inverse the order of the cmp() function (and hence cannot `#derive[Ord]`)
impl Ord for GraphNodeAndIdx {
    fn cmp(&self, other: &GraphNodeAndIdx) -> Ordering {
        // self.node.cmp(&other.node)
        other.node.cmp(&self.node)
    }
}

impl PartialOrd for GraphNodeAndIdx {
    fn partial_cmp(&self, other: &GraphNodeAndIdx) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GraphNodeAndIdx {
    fn eq(&self, other: &GraphNodeAndIdx) -> bool {
        self.node == other.node
    }
}

fn solution_part1(input: &str) -> Result<Day07SolutionPart1> {
    let graph = build_graph(parse_input(input));
    let nb_nodes = graph.node_count();
    let mut solution = String::with_capacity(nb_nodes);

    let mut dependencies: HashSet<GraphNode> = HashSet::new();
    let mut heap: BinaryHeap<GraphNodeAndIdx> = BinaryHeap::new();

    while solution.len() != nb_nodes {
        find_nodes_ready(&graph, &dependencies, &mut heap);
        let node = heap.pop().unwrap();
        dependencies.insert(node.node);
        solution.push(node.node);
    }

    assert_eq!(solution.len(), nb_nodes);

    Ok(solution)
}

fn find_nodes_ready(
    graph: &Graph,
    dependencies: &HashSet<GraphNode>,
    heap: &mut BinaryHeap<GraphNodeAndIdx>,
) {
    graph.node_references().for_each(|(node_idx, node)| {
        if graph
            .neighbors_directed(node_idx, petgraph::Direction::Incoming)
            .all(|parent_idx| {
                let parent_node = graph[parent_idx];
                let parent_seen = dependencies.contains(&parent_node);
                parent_seen
            })
        {
            if !dependencies.contains(&node) {
                if heap.iter().all(|node_and_idx| node_idx != node_and_idx.idx) {
                    heap.push(GraphNodeAndIdx::new(*node, node_idx));
                }
            }
        }
    });
}

fn char_to_duration(c: char, step_baseline: u8) -> u8 {
    (c as u8 - b'A') + step_baseline + 1
}

#[derive(Debug, PartialEq)]
pub struct Busy {
    steps_remaining: u8,
    c: char,
}

#[derive(Debug, PartialEq)]
pub enum WorkerState {
    Idle,
    Busy(Busy),
}

#[derive(Debug)]
pub struct Workers {
    states: Vec<WorkerState>,
}

impl Workers {
    pub fn new(nb_workers: usize) -> Workers {
        Workers {
            states: (0..nb_workers).map(|_| WorkerState::Idle).collect(),
        }
    }

    pub fn idle(&mut self) -> impl Iterator<Item = &mut WorkerState> {
        self.states
            .iter_mut()
            .filter(|state| **state == WorkerState::Idle)
    }

    pub fn step(&mut self) {
        self.states.iter_mut().for_each(|state| {
            if let WorkerState::Busy(busy) = state {
                busy.steps_remaining -= 1;
            }
        })
    }

    pub fn ready(&mut self) -> impl Iterator<Item = &mut WorkerState> {
        self.states.iter_mut().filter(|state| match state {
            WorkerState::Busy(busy) => busy.steps_remaining == 0,
            _ => false,
        })
    }
}

fn solution_part2(input: &str, nb_workers: usize, step_baseline: u8) -> Result<Day07SolutionPart2> {
    let graph = build_graph(parse_input(input));
    let nb_nodes = graph.node_count();

    let mut done: HashSet<GraphNode> = HashSet::new();
    let mut heap: BinaryHeap<GraphNodeAndIdx> = BinaryHeap::new();
    let mut being_worked_on: HashSet<GraphNode> = HashSet::new();

    let mut workers = Workers::new(nb_workers);

    let mut solution = String::with_capacity(nb_nodes);

    let mut duration = 0;

    while solution.len() != nb_nodes {
        // Distribute work to the workers
        find_nodes_ready(&graph, &done, &mut heap);
        for worker in workers.idle() {
            while let Some(node) = heap.pop() {
                if !being_worked_on.contains(&node.node) && !done.contains(&node.node) {
                    being_worked_on.insert(node.node);
                    *worker = WorkerState::Busy(Busy {
                        steps_remaining: char_to_duration(node.node, step_baseline),
                        c: node.node,
                    });
                    break;
                }
            }
        }

        // Advance the workers
        workers.step();

        // Check workers who are done
        for worker in workers.ready() {
            if let WorkerState::Busy(busy) = worker {
                being_worked_on.remove(&busy.c);
                done.insert(busy.c);
                solution.push(busy.c);
            } else {
                unreachable!();
            }

            *worker = WorkerState::Idle;
        }

        duration += 1;
    }

    Ok(duration as i64)
}

#[cfg(test)]
mod tests {

    static EX_INPUT: &str = "Step C must be finished before step A can begin.
                             Step C must be finished before step F can begin.
                             Step A must be finished before step B can begin.
                             Step A must be finished before step D can begin.
                             Step B must be finished before step E can begin.
                             Step D must be finished before step E can begin.
                             Step F must be finished before step E can begin.";

    mod part1 {

        mod solution {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part1();

                let expected = "EPWCFXKISTZVJHDGNABLQYMORU";
                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day07Initial;
            use super::super::EX_INPUT;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = "CABDFE";
                let to_check = Day07Initial::new(EX_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod extra {
            use super::super::super::solution_part1;
            use super::super::EX_INPUT;
            use crate::{tests::init_logger, PUZZLE_INPUT};

            #[test]
            fn ex01() {
                init_logger();

                let expected = "CABDFE";
                let to_check = solution_part1(EX_INPUT).unwrap();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01_wrong_answer() {
                init_logger();
                let wrong_answer = "XSEPZKIWTFCVNABJHQDYGLMORU";
                let to_check = solution_part1(PUZZLE_INPUT).unwrap();
                assert_ne!(to_check, wrong_answer);

                let wrong_answer = "PWXTFSCEZVNKIABJHDGLYMQORU";
                let to_check = solution_part1(PUZZLE_INPUT).unwrap();
                assert_ne!(to_check, wrong_answer);
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 952;
                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            // use super::super::super::Day07Initial;
            use super::super::super::solution_part2;
            use super::super::EX_INPUT;
            use crate::tests::init_logger;

            #[test]
            fn ex01() {
                init_logger();

                let nb_workers = 2;
                let step_baseline = 0;
                let expected = 15;

                // let to_check = Day07Initial::new(EX_INPUT).solution_part2();
                let to_check = solution_part2(EX_INPUT, nb_workers, step_baseline).unwrap();

                assert_eq!(to_check, expected);
            }
        }

        mod extra {
            use super::super::super::char_to_duration;
            // use super::super::super::Day07Initial;
            // use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn durations() {
                let step_baseline = 60;
                assert_eq!(char_to_duration('A', step_baseline), 61);
                assert_eq!(char_to_duration('B', step_baseline), 62);
                assert_eq!(char_to_duration('C', step_baseline), 63);
                assert_eq!(char_to_duration('Z', step_baseline), 86);

                let step_baseline = 0;
                assert_eq!(char_to_duration('A', step_baseline), 1);
                assert_eq!(char_to_duration('B', step_baseline), 2);
                assert_eq!(char_to_duration('C', step_baseline), 3);
                assert_eq!(char_to_duration('Z', step_baseline), 26);
            }
        }
    }
}
