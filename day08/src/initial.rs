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

        parse_tree_slice(&data).metadata_sum
    }

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
}

#[derive(Debug)]
struct RecurseResult {
    slice_length: usize,
    metadata_sum: usize,
}

fn parse_tree_slice(slice: &[usize]) -> RecurseResult {
    let nb_child = slice[0];
    let nb_metadata = slice[1];

    if nb_child == 0 {
        // NOTE: We can't rely on the size of the slice
        //       since the recursion cannot cut the end
        //       of the slice
        let metadata_sum: usize = slice.iter().skip(2).take(nb_metadata).sum();
        // The child's slice contains:
        //  1) Number of children
        //  2) Number of metata
        //  3) The meta data
        //  4) The remaining of the tree info (the slice if not cut at the end)
        let slice_length = 1 + 1 + nb_metadata;

        RecurseResult {
            slice_length,
            metadata_sum,
        }
    } else {
        // Calculate the children's total length
        let children_results = (0..nb_child).fold(
            RecurseResult {
                slice_length: 0,
                metadata_sum: 0,
            },
            |mut acc, _| {
                let child_result = parse_tree_slice(&slice[(2 + acc.slice_length)..]);
                acc.slice_length += child_result.slice_length;
                acc.metadata_sum += child_result.metadata_sum;
                acc
            },
        );

        let slice_length = 2 + children_results.slice_length + nb_metadata;
        let metadata_sum = slice
            .iter()
            .skip(2 + children_results.slice_length)
            .take(nb_metadata)
            .sum::<usize>()
            + children_results.metadata_sum;

        RecurseResult {
            slice_length,
            metadata_sum,
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

                unimplemented!();

                let expected = 0;
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

                unimplemented!();

                let expected = 0;
                let input = "";
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
