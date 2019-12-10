use crate::days::{CommonError, Day};
use anyhow::Result;
use std::{
    cmp::{Ord, Ordering},
    collections::HashSet,
    fs,
    hash::{Hash, Hasher},
    result::Result as StdResult,
    str::FromStr,
};
use thiserror::Error;

pub struct Day3 {
    input: String,
}

#[derive(Debug, Error)]
pub enum Day3Error {
    #[error("Failed to parse direction: {0}")]
    DirectionParseError(String),

    #[error("Failed to parse distance as i32: {0}")]
    DistanceParseError(String),

    #[error("Expected to find two wire paths, found: {0}")]
    InvalidWirePathCount(usize),

    #[error("There were no points where the two wire paths intersected")]
    NoIntersectionsForWiresFound,
}

pub struct Instruction {
    direction: Direction,
    distance: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct Point {
    x: i32,
    y: i32,
    steps_from_origin: u32,
}

impl Day3 {
    pub fn new() -> Result<Self> {
        let input =
            fs::read_to_string("res/day3.txt").map_err(|_| CommonError::InputReadError(3))?;

        // let input =
        //     "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        //         .to_string();

        //let input = "R1,U1\nU1,R1,U1,D1,U1,D1".to_string();

        //let input = "D1,R5,L4,U2\nU1,R1,D4,U3".to_string();

        Ok(Day3 { input })
    }
}

impl Day for Day3 {
    fn part1(&self) -> Result<String> {
        let paths = parse_input_to_paths(&self.input)?;
        let wire_1_points = find_wire_points_for_instructions(&paths[0]);
        let wire_2_points = find_wire_points_for_instructions(&paths[1]);

        let closest_point = wire_1_points
            .intersection(&wire_2_points)
            .min_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())))
            .ok_or(Day3Error::NoIntersectionsForWiresFound)?;

        let result = closest_point.x.abs() + closest_point.y.abs();

        Ok(format!("{}", result))
    }

    fn part2(&self) -> Result<String> {
        let paths = parse_input_to_paths(&self.input)?;
        let wire_1_points = find_wire_points_for_instructions(&paths[0]);
        let wire_2_points = find_wire_points_for_instructions(&paths[1]);

        let result = wire_1_points
            .iter()
            .filter_map(|point| {
                let wire_2_point = wire_2_points.get(&point)?;

                Some(point.steps_from_origin + wire_2_point.steps_from_origin)
            })
            .min()
            .ok_or(Day3Error::NoIntersectionsForWiresFound)?;

        Ok(format!("{}", result))
    }
}

impl FromStr for Instruction {
    type Err = Day3Error;

    fn from_str(input: &str) -> StdResult<Instruction, Self::Err> {
        let (direction, distance) = input.split_at(1);
        let direction = Direction::from_str(direction)?;

        let distance = distance
            .parse::<i32>()
            .map_err(|_| Day3Error::DistanceParseError(distance.to_string()))?;

        Ok(Instruction {
            direction,
            distance,
        })
    }
}

impl FromStr for Direction {
    type Err = Day3Error;

    fn from_str(input: &str) -> StdResult<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Day3Error::DirectionParseError(input.to_string())),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.x.abs() + self.y.abs()).partial_cmp(&(other.x.abs() + other.y.abs()))
    }
}

fn parse_input_to_paths(input: &str) -> Result<Vec<Vec<Instruction>>, Day3Error> {
    let paths = input
        .lines()
        .map(|slice| {
            slice
                .split(",")
                .map(|instruction| Instruction::from_str(instruction))
                .collect::<Result<Vec<Instruction>, Day3Error>>()
        })
        .collect::<Result<Vec<Vec<Instruction>>, Day3Error>>()?;

    if paths.len() == 2 {
        Ok(paths)
    } else {
        Err(Day3Error::InvalidWirePathCount(paths.len()))
    }
}

fn find_wire_points_for_instructions(instructions: &Vec<Instruction>) -> HashSet<Point> {
    const POINT_SET_SIZE: usize = 160_000;
    let mut points = Vec::with_capacity(POINT_SET_SIZE);

    let mut latest_point = Point {
        x: 0,
        y: 0,
        steps_from_origin: 0,
    };

    instructions.iter().for_each(|instruction| {
        points.extend(get_all_points_covered_by_instruction(
            &latest_point,
            instruction,
        ));

        latest_point = get_new_latest_point(&latest_point, instruction);
    });

    points.into_iter().collect()
}

fn get_all_points_covered_by_instruction(point: &Point, instruction: &Instruction) -> Vec<Point> {
    match instruction.direction {
        Direction::Up => ((point.y + 1)..=(point.y + instruction.distance))
            .enumerate()
            .map(|(i, y)| Point {
                x: point.x,
                y,
                steps_from_origin: point.steps_from_origin + (i as u32) + 1,
            })
            .collect(),
        Direction::Down => ((point.y - instruction.distance)..=(point.y - 1))
            .rev()
            .enumerate()
            .map(|(i, y)| Point {
                x: point.x,
                y,
                steps_from_origin: point.steps_from_origin + (i as u32) + 1,
            })
            .collect(),
        Direction::Left => ((point.x - instruction.distance)..=(point.x - 1))
            .rev()
            .enumerate()
            .map(|(i, x)| Point {
                x,
                y: point.y,
                steps_from_origin: point.steps_from_origin + (i as u32) + 1,
            })
            .collect(),
        Direction::Right => ((point.x + 1)..=(point.x + instruction.distance))
            .enumerate()
            .map(|(i, x)| Point {
                x,
                y: point.y,
                steps_from_origin: point.steps_from_origin + (i as u32) + 1,
            })
            .collect(),
    }
}

fn get_new_latest_point(point: &Point, instruction: &Instruction) -> Point {
    match instruction.direction {
        Direction::Up => Point {
            x: point.x,
            y: (point.y + instruction.distance),
            steps_from_origin: point.steps_from_origin + instruction.distance.abs() as u32,
        },
        Direction::Down => Point {
            x: point.x,
            y: (point.y - instruction.distance),
            steps_from_origin: point.steps_from_origin + instruction.distance.abs() as u32,
        },
        Direction::Left => Point {
            x: (point.x - instruction.distance),
            y: point.y,
            steps_from_origin: point.steps_from_origin + instruction.distance.abs() as u32,
        },
        Direction::Right => Point {
            x: (point.x + instruction.distance),
            y: point.y,
            steps_from_origin: point.steps_from_origin + instruction.distance.abs() as u32,
        },
    }
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn part1_puzzle() -> Result<()> {
        assert_eq!(Day3::new()?.part1()?, "721");

        Ok(())
    }

    #[test]
    fn part2_puzzle() -> Result<()> {
        assert_eq!(Day3::new()?.part2()?, "7388");

        Ok(())
    }
}
