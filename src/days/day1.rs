use crate::days::{CommonError, Day};
use anyhow::Result;
use std::fs;
use thiserror::Error;

pub struct Day1 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day1Error {
    #[error("Failed to parse mass as u32: {0}")]
    MassParseError(String),
}

impl Day1 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day1.txt").map_err(|_| CommonError::InputReadError(1))?;

        Ok(Day1 { input })
    }
}

impl Day for Day1 {
    fn part1(&self) -> Result<String> {
        let fuel = parse_input_to_masses(&self.input)?
            .into_iter()
            .map(|mass| (mass / 3) - 2)
            .sum::<u32>();

        Ok(format!("{}", fuel))
    }

    fn part2(&self) -> Result<String> {
        let mut total_fuel = 0;

        parse_input_to_masses(&self.input)?
            .into_iter()
            .map(|mass| (mass / 3) - 2)
            .for_each(|fuel| {
                total_fuel += fuel;
                let mut mass = fuel;

                loop {
                    mass = (mass / 3).checked_sub(2).unwrap_or(0);

                    if mass == 0 {
                        break;
                    }

                    total_fuel += mass;
                }
            });

        Ok(format!("{}", total_fuel))
    }
}

fn parse_input_to_masses(input: &str) -> Result<Vec<u32>, Day1Error> {
    input
        .lines()
        .map(|slice| {
            slice
                .parse::<u32>()
                .map_err(|_| Day1Error::MassParseError(slice.to_string()))
        })
        .collect::<Result<Vec<u32>, Day1Error>>()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day1::new()?.part1()?, "3443395");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day1::new()?.part2()?, "5162216");

        Ok(())
    }
}
