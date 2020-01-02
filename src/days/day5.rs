use crate::{
    common::intcode,
    days::{CommonError, Day},
};
use anyhow::Result;
use std::fs;
use thiserror::Error;

pub struct Day5 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day5Error {
    #[error("Intcode did not output a value")]
    OutputNotFoundError,
}

impl Day5 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day5.txt").map_err(|_| CommonError::InputReadError(5))?;

        Ok(Day5 { input })
    }
}

impl Day for Day5 {
    fn part1(&self) -> Result<String> {
        const INPUT: Option<i32> = Some(1);

        let mut intcode = intcode::parse_input_to_intcode(&self.input)?;
        let mut result = intcode::run_intcode_to_halt(&mut intcode, INPUT)?;

        if result.is_empty() {
            return Err(Day5Error::OutputNotFoundError.into());
        };

        Ok(format!("{}", result.pop().unwrap()))
    }

    fn part2(&self) -> Result<String> {
        const INPUT: Option<i32> = Some(5);

        let mut intcode = intcode::parse_input_to_intcode(&self.input)?;
        let mut result = intcode::run_intcode_to_halt(&mut intcode, INPUT)?;

        if result.is_empty() {
            return Err(Day5Error::OutputNotFoundError.into());
        };

        Ok(format!("{}", result.pop().unwrap()))
    }
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day5::new()?.part1()?, "15386262");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day5::new()?.part2()?, "10376124");

        Ok(())
    }
}
