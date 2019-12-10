use crate::days::{CommonError, Day};
use anyhow::Result;
use std::fs;
use thiserror::Error;

pub struct Day0 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day0Error {
    #[error("Failed to parse: {0}")]
    ParseError(String),
}

impl Day0 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day0.txt").map_err(|_| CommonError::InputReadError(0))?;

        Ok(Day0 { input })
    }
}

impl Day for Day0 {
    fn part1(&self) -> Result<String> {
        Err(CommonError::NotDone.into())
    }

    fn part2(&self) -> Result<String> {
        Err(CommonError::NotDone.into())
    }
}

#[cfg(test)]
mod day0_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day0::new()?.part1()?, "");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day0::new()?.part2()?, "");

        Ok(())
    }
}
