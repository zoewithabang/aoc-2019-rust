use std::convert::TryFrom;
use thiserror::Error;

const OPCODE_ADD: i32 = 1;
const OPCODE_MULTIPLY: i32 = 2;
const OPCODE_INPUT: i32 = 3;
const OPCODE_OUTPUT: i32 = 4;
const OPCODE_JUMP_IF_TRUE: i32 = 5;
const OPCODE_JUMP_IF_FALSE: i32 = 6;
const OPCODE_LESS_THAN: i32 = 7;
const OPCODE_EQUALS: i32 = 8;
const OPCODE_HALT: i32 = 99;
const MODE_POSITION: i32 = 0;
const MODE_IMMEDIATE: i32 = 1;

#[derive(Debug, Error)]
pub enum IntcodeError {
    #[error("Failed to parse intcode as u32: {0}")]
    IntcodeParseError(String),

    #[error("Invalid Mode {0} for Opcode {1}")]
    InvalidModeForOpcode(String, String),

    #[error("First value of instruction cannot be negative")]
    NegativeInstruction,

    #[error("Input expected but was not found")]
    NoInputFound,

    #[error("Unexpected end of Intcode")]
    UnexpectedEndOfIntcode,

    #[error("Unknown Opcode: {0}")]
    UnknownOpcode(i32),

    #[error("Unknown Mode: {0}")]
    UnknownMode(i32),
}

#[derive(Debug)]
enum Opcode {
    Add([Mode; 3]),
    Multiply([Mode; 3]),
    Input([Mode; 1]),
    Output([Mode; 1]),
    JumpIfTrue([Mode; 2]),
    JumpIfFalse([Mode; 2]),
    LessThan([Mode; 3]),
    Equals([Mode; 3]),
    Halt,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i32> for Mode {
    type Error = IntcodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            MODE_POSITION => Ok(Mode::Position),
            MODE_IMMEDIATE => Ok(Mode::Immediate),
            _ => Err(IntcodeError::UnknownMode(value)),
        }
    }
}

pub fn parse_input_to_intcode(input: &str) -> Result<Vec<i32>, IntcodeError> {
    input
        .split(",")
        .map(|slice| {
            slice
                .parse::<i32>()
                .map_err(|_| IntcodeError::IntcodeParseError(slice.to_string()))
        })
        .collect::<Result<Vec<i32>, IntcodeError>>()
}

pub fn run_intcode_to_halt(
    intcode: &mut Vec<i32>,
    input: Option<i32>,
) -> Result<Vec<i32>, IntcodeError> {
    if intcode.len() == 0 {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let mut pointer = 0;
    let mut outputs = Vec::new();
    while run_intcode(intcode, &mut pointer, input, &mut outputs)? {}

    Ok(outputs)
}

fn run_intcode(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    input: Option<i32>,
    output: &mut Vec<i32>,
) -> Result<bool, IntcodeError> {
    if *pointer >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let instruction = parse_instruction(intcode[*pointer])?;

    if let Opcode::Halt = instruction {
        return Ok(false);
    }

    match instruction {
        Opcode::Add(modes) => run_add(intcode, pointer, modes),
        Opcode::Multiply(modes) => run_mult(intcode, pointer, modes),
        Opcode::Input(modes) => run_input(intcode, pointer, modes, input),
        Opcode::Output(modes) => run_output(intcode, pointer, modes, output),
        Opcode::JumpIfTrue(modes) => run_jump_if_true(intcode, pointer, modes),
        Opcode::JumpIfFalse(modes) => run_jump_if_false(intcode, pointer, modes),
        Opcode::LessThan(modes) => run_less_than(intcode, pointer, modes),
        Opcode::Equals(modes) => run_equals(intcode, pointer, modes),
        _ => Err(IntcodeError::UnknownOpcode(intcode[*pointer])),
    }?;

    Ok(true)
}

fn parse_instruction(instruction: i32) -> Result<Opcode, IntcodeError> {
    if instruction < 0 {
        return Err(IntcodeError::NegativeInstruction);
    }

    let opcode = instruction % 100;
    let mode_1 = Mode::try_from(instruction % 1_000 / 100)?; // hundreds
    let mode_2 = Mode::try_from(instruction % 10_000 / 1_000)?; // thousands
    let mode_3 = Mode::try_from(instruction % 100_000 / 10_000)?; // ten thousands

    match opcode {
        OPCODE_ADD => Ok(Opcode::Add([mode_1, mode_2, mode_3])),
        OPCODE_MULTIPLY => Ok(Opcode::Multiply([mode_1, mode_2, mode_3])),
        OPCODE_INPUT => Ok(Opcode::Input([mode_1])),
        OPCODE_OUTPUT => Ok(Opcode::Output([mode_1])),
        OPCODE_JUMP_IF_TRUE => Ok(Opcode::JumpIfTrue([mode_1, mode_2])),
        OPCODE_JUMP_IF_FALSE => Ok(Opcode::JumpIfFalse([mode_1, mode_2])),
        OPCODE_LESS_THAN => Ok(Opcode::LessThan([mode_1, mode_2, mode_3])),
        OPCODE_EQUALS => Ok(Opcode::Equals([mode_1, mode_2, mode_3])),
        OPCODE_HALT => Ok(Opcode::Halt),
        _ => Err(IntcodeError::UnknownOpcode(opcode)),
    }
}

fn run_add(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 3],
) -> Result<(), IntcodeError> {
    if (*pointer + 3) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode.into());
    }

    let input1 = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize],
        Mode::Immediate => intcode[*pointer + 1 as usize],
    };

    let input2 = match modes[1] {
        Mode::Position => intcode[intcode[*pointer + 2] as usize],
        Mode::Immediate => intcode[*pointer + 2 as usize],
    };

    match modes[2] {
        Mode::Position => {
            let output_address = intcode[*pointer + 3] as usize;
            intcode[output_address] = input1 + input2;
        }
        Mode::Immediate => intcode[*pointer + 3 as usize] = input1 + input2,
    }

    *pointer += 4;

    Ok(())
}

fn run_mult(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 3],
) -> Result<(), IntcodeError> {
    if (*pointer + 3) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode.into());
    }

    let input1 = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize],
        Mode::Immediate => intcode[*pointer + 1 as usize],
    };

    let input2 = match modes[1] {
        Mode::Position => intcode[intcode[*pointer + 2] as usize],
        Mode::Immediate => intcode[*pointer + 2 as usize],
    };

    match modes[2] {
        Mode::Position => {
            let output_address = intcode[*pointer + 3] as usize;
            intcode[output_address] = input1 * input2;
        }
        Mode::Immediate => intcode[*pointer + 3 as usize] = input1 * input2,
    }

    *pointer += 4;

    Ok(())
}

fn run_input(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 1],
    input: Option<i32>,
) -> Result<(), IntcodeError> {
    if (*pointer + 1) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    if input.is_none() {
        return Err(IntcodeError::NoInputFound);
    }

    match modes[0] {
        Mode::Position => {
            let output_address = intcode[*pointer + 1] as usize;
            intcode[output_address] = input.unwrap();

            Ok(())
        }
        Mode::Immediate => {
            intcode[*pointer + 1] = input.unwrap();

            Ok(())
        }
    }?;

    *pointer += 2;

    Ok(())
}

fn run_output(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 1],
    outputs: &mut Vec<i32>,
) -> Result<(), IntcodeError> {
    if (*pointer + 1) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    match modes[0] {
        Mode::Position => outputs.push(intcode[intcode[*pointer + 1] as usize]),
        Mode::Immediate => outputs.push(intcode[*pointer + 1]),
    };

    *pointer += 2;

    Ok(())
}

fn run_jump_if_true(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 2],
) -> Result<(), IntcodeError> {
    if (*pointer + 1) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let is_true = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize] != 0,
        Mode::Immediate => intcode[*pointer + 1] != 0,
    };

    if !is_true {
        *pointer += 3;

        return Ok(());
    }

    match modes[1] {
        Mode::Position => {
            *pointer = intcode[intcode[*pointer + 2] as usize] as usize;
        }
        Mode::Immediate => {
            *pointer = intcode[*pointer + 2] as usize;
        }
    };

    Ok(())
}

fn run_jump_if_false(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 2],
) -> Result<(), IntcodeError> {
    if (*pointer + 2) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let is_false = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize] == 0,
        Mode::Immediate => intcode[*pointer + 1] == 0,
    };

    if !is_false {
        *pointer += 3;

        return Ok(());
    }

    match modes[1] {
        Mode::Position => {
            *pointer = intcode[intcode[*pointer + 2] as usize] as usize;
        }
        Mode::Immediate => {
            *pointer = intcode[*pointer + 2] as usize;
        }
    };

    Ok(())
}

fn run_less_than(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 3],
) -> Result<(), IntcodeError> {
    if (*pointer + 3) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let input1 = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize],
        Mode::Immediate => intcode[*pointer + 1 as usize],
    };

    let input2 = match modes[1] {
        Mode::Position => intcode[intcode[*pointer + 2] as usize],
        Mode::Immediate => intcode[*pointer + 2 as usize],
    };

    let output = if input1 < input2 { 1 } else { 0 };

    match modes[2] {
        Mode::Position => {
            let output_address = intcode[*pointer + 3] as usize;
            intcode[output_address] = output;
        }
        Mode::Immediate => intcode[*pointer + 3 as usize] = output,
    }

    *pointer += 4;

    Ok(())
}

fn run_equals(
    intcode: &mut Vec<i32>,
    pointer: &mut usize,
    modes: [Mode; 3],
) -> Result<(), IntcodeError> {
    if (*pointer + 3) >= intcode.len() {
        return Err(IntcodeError::UnexpectedEndOfIntcode);
    }

    let input1 = match modes[0] {
        Mode::Position => intcode[intcode[*pointer + 1] as usize],
        Mode::Immediate => intcode[*pointer + 1 as usize],
    };

    let input2 = match modes[1] {
        Mode::Position => intcode[intcode[*pointer + 2] as usize],
        Mode::Immediate => intcode[*pointer + 2 as usize],
    };

    let output = if input1 == input2 { 1 } else { 0 };

    match modes[2] {
        Mode::Position => {
            let output_address = intcode[*pointer + 3] as usize;
            intcode[output_address] = output;
        }
        Mode::Immediate => intcode[*pointer + 3 as usize] = output,
    }

    *pointer += 4;

    Ok(())
}
