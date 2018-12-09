use crate::{parse_input, AoC, Day08SolutionPart1, Day08SolutionPart2};

#[derive(Debug)]
pub struct Day08Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day08Initial<'a> {
    type SolutionPart1 = Day08SolutionPart1;
    type SolutionPart2 = Day08SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day08Initial<'_> {
        Day08Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let data: Vec<_> = parse_input(self.input).collect();

        parse_tree_slice(&data).node_value_part1
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let data: Vec<_> = parse_input(self.input).collect();

        parse_tree_slice(&data).node_value_part2
    }
}

#[derive(Debug)]
struct RecurseResult<'a> {
    slice_length: usize,
    node_value_part1: usize,
    node_value_part2: usize,
    metadata: &'a [usize],
}

fn parse_tree_slice<'a>(slice: &'a [usize]) -> RecurseResult<'a> {
    let nb_child = slice[0];
    let nb_metadata = slice[1];

    if nb_child == 0 {
        // NOTE: We can't rely on the size of the slice
        //       since the recursion cannot cut the end
        //       of the slice
        let metadata = &slice[2..(2 + nb_metadata)];
        // let metadata_sum: usize = slice.iter().skip(2).take(nb_metadata).sum();
        let metadata_sum: usize = metadata.iter().sum();
        // The child's slice contains:
        //  1) Number of children
        //  2) Number of metata
        //  3) The meta data
        //  4) The remaining of the tree info (the slice if not cut at the end)
        let slice_length = 1 + 1 + nb_metadata;

        RecurseResult {
            slice_length,
            node_value_part1: metadata_sum,
            node_value_part2: metadata_sum,
            metadata,
        }
    } else {
        // Calculate the children's total length
        let mut children_length = 0;
        let children_results: Vec<_> = (0..nb_child)
            .map(|_| {
                let child_result = parse_tree_slice(&slice[(2 + children_length)..]);
                children_length += child_result.slice_length;
                child_result
            })
            .collect();

        let i0 = 2 + children_length;
        let i1 = i0 + nb_metadata;
        let metadata = &slice[i0..i1];

        let slice_length = 2 + children_length + nb_metadata;

        let node_value_part1 = slice
            .iter()
            .skip(2 + children_length)
            .take(nb_metadata)
            .sum::<usize>()
            + children_results
                .iter()
                .map(|r| r.node_value_part1)
                .sum::<usize>();

        let node_value_part2: usize = metadata
            .iter()
            .map(|child_id| {
                if *child_id == 0 {
                    0
                } else if *child_id > children_results.len() {
                    0
                } else {
                    children_results[*child_id - 1].node_value_part2
                }
            })
            .sum();

        let i0 = 2 + children_length;
        let i1 = i0 + nb_metadata;
        RecurseResult {
            slice_length,
            node_value_part1,
            node_value_part2,
            metadata: &slice[i0..i1],
        }
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 42196;
                let to_check = Day08Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 138;
                let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
                let to_check = Day08Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 33649;
                let to_check = Day08Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 66;
                let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
                let to_check = Day08Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
