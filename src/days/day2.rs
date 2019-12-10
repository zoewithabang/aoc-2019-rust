use crate::days::{CommonError, Day};
use anyhow::Result;
use std::fs;
use thiserror::Error;

const OPCODE_ADD: u32 = 1;
const OPCODE_MULT: u32 = 2;
const OPCODE_HALT: u32 = 99;

pub struct Day2 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day2Error {
    #[error("Failed to parse intcode as u32: {0}")]
    IntcodeParseError(String),

    #[error("Timed out after 1,000,000 iterations")]
    IterationTimeoutError,

    #[error("Unexpected end of Intcode")]
    UnexpectedEndOfIntcode,

    #[error("Unknown Opcode: {0}")]
    UnknownOpcode(u32),
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
        let mut intcode = parse_input_to_intcode(&self.input)?;
        let mut pointer = 0;
        intcode[1] = 12;
        intcode[2] = 2;

        while intcode[pointer] != OPCODE_HALT {
            run_intcode(&mut intcode, &mut pointer)?;
        }

        Ok(format!("{}", intcode[0]))
    }

    fn part2(&self) -> Result<String> {
        const DESIRED_OUTPUT: u32 = 19_690_720;
        const MAX_ALLOWED_ITERATIONS: u32 = 1_000_000;

        let initial_intcode = parse_input_to_intcode(&self.input)?;
        let mut noun = 0;
        let mut verb = 0;
        let mut iterations = 0;

        loop {
            let mut intcode = initial_intcode.clone();
            let mut pointer = 0;
            intcode[1] = noun;
            intcode[2] = verb;

            while intcode[pointer] != OPCODE_HALT {
                run_intcode(&mut intcode, &mut pointer)?;
            }

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
                return Err(Day2Error::IterationTimeoutError.into());
            }
        }

        let result = (100 * noun) + verb;

        Ok(format!("{}", result))
    }
}

fn parse_input_to_intcode(input: &str) -> Result<Vec<u32>, Day2Error> {
    input
        .split(",")
        .map(|slice| {
            slice
                .parse::<u32>()
                .map_err(|_| Day2Error::IntcodeParseError(slice.to_string()))
        })
        .collect::<Result<Vec<u32>, Day2Error>>()
}

fn run_intcode(intcode: &mut Vec<u32>, pointer: &mut usize) -> Result<(), Day2Error> {
    let input1_address = intcode[*pointer + 1] as usize;
    let input2_address = intcode[*pointer + 2] as usize;
    let output_address = intcode[*pointer + 3] as usize;

    let output = match intcode[*pointer] {
        OPCODE_ADD => Ok(intcode[input1_address] + intcode[input2_address]),
        OPCODE_MULT => Ok(intcode[input1_address] * intcode[input2_address]),
        _ => Err(Day2Error::UnknownOpcode(intcode[*pointer])),
    }?;

    intcode[output_address] = output;
    *pointer += 4;

    if (*pointer + 3) <= intcode.len() {
        Ok(())
    } else {
        Err(Day2Error::UnexpectedEndOfIntcode.into())
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
