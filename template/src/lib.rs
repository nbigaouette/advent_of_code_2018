//! # Day XX:
//!
//! [Benchmarking report](../../../dayXX/target/criterion/report/index.html):
//!
//! * [Part 1](../../../dayXX/target/criterion/dayXX_part1/report/index.html)
//! * [Part 2](../../../dayXX/target/criterion/dayXX_part2/report/index.html)
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

#[macro_use]
extern crate log;

pub struct Solution {
    pub part1: usize,
    pub part2: usize,
}

pub fn aoc_dayXX(input: &str) -> Solution {
    unimplemented!()

    // Solution {
    //     part1: ,
    //     part2: ,
    // }
}

pub mod benchmark {
    pub const BENCHMARKING_INPUT: &str = "";
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use std::env;

    fn init_logger() {
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

    mod aoc2018 {
        mod dayXX {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        unimplemented!();
                        // let expected = ;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_dayXX(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        unimplemented!();
                        // let expected = ;
                        let input = "";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_dayXX(input);

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
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        unimplemented!();
                        // let expected = ;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_dayXX(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use *;

                    #[test]
                    fn ex01() {
                        unimplemented!();
                        // let expected = ;
                        let input = "";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_dayXX(input);

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
    }
}
