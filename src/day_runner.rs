use crate::days::{day1::Day1, day2::Day2, day3::Day3, Day};
use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;

const PUZZLE_DAYS: usize = 25;

pub struct DayRunner {
    days: HashMap<u32, Box<dyn Day>>,
}

impl DayRunner {
    pub fn new() -> Result<DayRunner> {
        let mut days = HashMap::<u32, Box<dyn Day>>::with_capacity(PUZZLE_DAYS);
        days.insert(1, Box::new(Day1::new()?));
        days.insert(2, Box::new(Day2::new()?));
        days.insert(3, Box::new(Day3::new()?));

        Ok(DayRunner { days })
    }

    pub fn run_day(&self, day: u32, part: Option<u32>) {
        if day < 1 || day > 25 {
            println!("That's... not a valid day");

            return;
        }

        match self.days.get(&day) {
            Some(day_instance) => {
                if let Some(part) = part {
                    run_day_part_internal(day, part, &day_instance);
                } else {
                    run_day_internal(day, &day_instance)
                }
            }
            None => println!("I haven't got to that day yet!"),
        };
    }
}

fn run_day_internal(day_number: u32, day: &Box<dyn Day>) {
    println!();
    println!("========== DAY {} ==========", day_number);
    println!("===== Part 1 =====");
    print_day_result(day.part1());
    println!("===== Part 2 =====");
    print_day_result(day.part2());
}

fn run_day_part_internal(day_number: u32, day_part: u32, day: &Box<dyn Day>) {
    let part: Box<dyn Fn() -> Result<String>> = match day_part {
        1 => Box::new(|| day.part1()),
        2 => Box::new(|| day.part2()),
        _ => Box::new(|| Err(anyhow!("There is no part {}!", day_part))),
    };

    println!();
    println!("========== DAY {} ==========", day_number);
    println!("===== Part {} =====", day_part);
    print_day_result(part());
}

fn print_day_result(result: std::result::Result<String, Error>) {
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}
