use crate::days::{CommonError, Day};
use anyhow::Result;
use std::{collections::HashMap, fs};
use thiserror::Error;

pub struct Day4 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day4Error {
    #[error("Failed to parse as u32: {0}")]
    RangeValueParseError(String),

    #[error("Failed to parse range from String: {0}")]
    RangeParseError(String),
}

impl Day4 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day4.txt").map_err(|_| CommonError::InputReadError(4))?;

        Ok(Day4 { input })
    }
}

impl Day for Day4 {
    fn part1(&self) -> Result<String> {
        let (start, end) = parse_input(&self.input)?;

        let result = (start..=end)
            .map(|password| {
                let digits = parse_digits(password);

                if digits.first() > digits.last() {
                    return 0;
                }

                let mut latest_digit = 0;
                let mut has_adjacent_double = false;
                let mut does_not_decrease = true;

                digits.into_iter().for_each(|d| {
                    if d < latest_digit {
                        does_not_decrease = false;

                        return;
                    }

                    if d == latest_digit {
                        has_adjacent_double = true;
                    }

                    latest_digit = d;
                });

                if has_adjacent_double && does_not_decrease {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>();

        Ok(format!("{}", result))
    }

    fn part2(&self) -> Result<String> {
        let (start, end) = parse_input(&self.input)?;

        let result = (start..=end)
            .map(|password| {
                let digits = parse_digits(password);

                if digits.first() > digits.last() {
                    return 0;
                }

                let mut latest_digit = 0;
                let mut adjacent_multiples = HashMap::new();
                let mut does_not_decrease = true;

                digits.into_iter().for_each(|d| {
                    if d < latest_digit {
                        does_not_decrease = false;

                        return;
                    }

                    if d == latest_digit {
                        adjacent_multiples
                            .entry(d)
                            .and_modify(|count| *count += 1)
                            .or_insert(2);
                    }

                    latest_digit = d;
                });

                if adjacent_multiples.values().any(|x| *x == 2) && does_not_decrease {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>();

        Ok(format!("{}", result))
    }
}

fn parse_input(input: &str) -> Result<(u32, u32), Day4Error> {
    let values = input
        .split("-")
        .map(|num| {
            num.parse::<u32>()
                .map_err(|_| Day4Error::RangeValueParseError(num.to_string()))
        })
        .collect::<Result<Vec<u32>, Day4Error>>()?;

    if values.len() == 2 {
        Ok((values[0], values[1]))
    } else {
        Err(Day4Error::RangeParseError(input.to_string()))
    }
}

fn parse_digits(password: u32) -> Vec<u32> {
    password
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day4::new()?.part1()?, "1955");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day4::new()?.part2()?, "1319");

        Ok(())
    }
}
