use anyhow::Result;
use thiserror::Error;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Failed to read input for day {0}")]
    InputReadError(u32),

    #[error("Not done yet!")]
    NotDone,
}

pub trait Day {
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}
