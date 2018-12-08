//! # Day 07:
//!
//! [Benchmarking report](../../../day07/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day07/target/criterion/day07_part1/report/index.html)
//! * [Part 2](../../../day07/target/criterion/day07_part2/report/index.html)
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

use std::collections::HashMap;
use std::fmt::Debug;

pub mod initial;
pub use crate::initial::Day07Initial;

pub type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub type GraphNode = char;
pub type GraphEdge = ();
pub type GraphIdx = petgraph::graph::NodeIndex;
pub type Graph = petgraph::Graph<GraphNode, GraphEdge, petgraph::Directed>;

type Day07SolutionPart1 = String;
type Day07SolutionPart2 = i64;

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

#[derive(Debug, PartialEq)]
pub struct ParsedEdge {
    node: char,
    dependency: char,
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = ParsedEdge> + 'a {
    input.trim().lines().map(|line| {
        let mut chars_iter = line.trim().chars().skip(5);
        let step_dependence = chars_iter.next().unwrap();
        let mut chars_iter = chars_iter.skip(30);
        let step = chars_iter.next().unwrap();
        ParsedEdge {
            dependency: step_dependence,
            node: step,
        }
    })
}

pub fn build_graph(input: impl Iterator<Item = ParsedEdge>) -> Graph {
    let mut graph = Graph::new();

    let mut seen_nodes: HashMap<char, GraphIdx> = HashMap::new();

    for parsed in input {
        let node_entry = seen_nodes
            .entry(parsed.node)
            .or_insert_with(|| {
                // Insert the node in the graph
                graph.add_node(parsed.node)
            })
            .clone();
        let dep_entry = seen_nodes
            .entry(parsed.dependency)
            .or_insert_with(|| {
                // Insert the node in the graph
                graph.add_node(parsed.dependency)
            })
            .clone();
        // Add the edge between the two nodes
        // let edge_from = node_entry;
        // let edge_to = dep_entry;
        let edge_from = dep_entry;
        let edge_to = node_entry;
        graph.add_edge(edge_from, edge_to, ());
    }

    graph
}

pub fn graph_to_dot(graph: &Graph) -> String {
    let dot = petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);
    format!("{:?}", dot)
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day07SolutionPart1, SolutionPart2 = Day07SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day07Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use crate::parse_input;
    use crate::{build_graph, graph_to_dot, ParsedEdge};

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

    #[test]
    fn parse() {
        let input = "Step C must be finished before step A can begin.
                     Step C must be finished before step F can begin.
                     Step A must be finished before step B can begin.
                     Step A must be finished before step D can begin.
                     Step B must be finished before step E can begin.
                     Step D must be finished before step E can begin.
                     Step F must be finished before step E can begin.";
        let parsed: Vec<_> = parse_input(input).collect();
        assert_eq!(
            parsed,
            vec![
                ParsedEdge {
                    dependency: 'C',
                    node: 'A'
                },
                ParsedEdge {
                    dependency: 'C',
                    node: 'F'
                },
                ParsedEdge {
                    dependency: 'A',
                    node: 'B'
                },
                ParsedEdge {
                    dependency: 'A',
                    node: 'D'
                },
                ParsedEdge {
                    dependency: 'B',
                    node: 'E'
                },
                ParsedEdge {
                    dependency: 'D',
                    node: 'E'
                },
                ParsedEdge {
                    dependency: 'F',
                    node: 'E'
                },
            ]
        );
    }

    #[test]
    fn parse_and_build_graph() {
        let input = "Step C must be finished before step A can begin.
                     Step C must be finished before step F can begin.
                     Step A must be finished before step B can begin.
                     Step A must be finished before step D can begin.
                     Step B must be finished before step E can begin.
                     Step D must be finished before step E can begin.
                     Step F must be finished before step E can begin.";
        let graph = build_graph(parse_input(input));

        /*
           -->A--->B--
         /    \      \
        C      -->D----->E
         \           /
          ---->F-----
        */

        assert_eq!(graph.node_count(), 6);
        assert_eq!(graph.edge_count(), 7);

        let graph_dot_string = graph_to_dot(&graph);
        assert_eq!(
            graph_dot_string,
            r#"digraph {
    0 [label="'A'"]
    1 [label="'C'"]
    2 [label="'F'"]
    3 [label="'B'"]
    4 [label="'D'"]
    5 [label="'E'"]
    1 -> 0
    1 -> 2
    0 -> 3
    0 -> 4
    3 -> 5
    4 -> 5
    2 -> 5
}
"#
        );
    }
}
