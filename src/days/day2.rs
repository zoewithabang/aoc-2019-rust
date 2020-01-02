use crate::{
    common::intcode,
    days::{CommonError, Day},
};
use anyhow::Result;
use std::fs;
use thiserror::Error;

pub struct Day2 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day2Error {
    #[error("Timed out after {0} iterations")]
    IterationTimeoutError(u32),
}

impl Day2 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day2.txt").map_err(|_| CommonError::InputReadError(2))?;

        Ok(Day2 { input })
    }
}

impl Day for Day2 {
    fn part1(&self) -> Result<String> {
        let mut intcode = intcode::parse_input_to_intcode(&self.input)?;
        intcode[1] = 12;
        intcode[2] = 2;
        intcode::run_intcode_to_halt(&mut intcode, None)?;

        Ok(format!("{}", intcode[0]))
    }

    fn part2(&self) -> Result<String> {
        const DESIRED_OUTPUT: i32 = 19_690_720;
        const MAX_ALLOWED_ITERATIONS: u32 = 1_000_000;

        let initial_intcode = intcode::parse_input_to_intcode(&self.input)?;
        let mut noun = 0;
        let mut verb = 0;
        let mut iterations = 0;

        loop {
            let mut intcode = initial_intcode.clone();
            intcode[1] = noun;
            intcode[2] = verb;
            intcode::run_intcode_to_halt(&mut intcode, None)?;

            if intcode[0] == DESIRED_OUTPUT {
                break;
            }

            if noun == verb {
                noun += 1;
                verb = 0;
            } else {
                verb += 1;
            }

            iterations += 1;

            if iterations == MAX_ALLOWED_ITERATIONS {
                return Err(Day2Error::IterationTimeoutError(iterations).into());
            }
        }

        let result = (100 * noun) + verb;

        Ok(format!("{}", result))
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day2::new()?.part1()?, "2842648");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day2::new()?.part2()?, "9074");

        Ok(())
    }
}
