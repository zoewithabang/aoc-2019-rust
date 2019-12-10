use anyhow::Result;
use aoc_2019::day_runner::DayRunner;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
struct Args {
    #[structopt(short, long)]
    day: Option<u32>,
    #[structopt(short, long, requires = "day")]
    part: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let day_runner = DayRunner::new()?;

    if let Some(day) = args.day {
        day_runner.run_day(day, args.part);

        return Ok(());
    }

    loop {
        println!();
        println!("Which day should I run? (1-25 or q to quit)");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "q" {
            println!("Later!");

            return Ok(());
        }

        let day = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("That's... not a valid day");

                continue;
            }
        };

        day_runner.run_day(day, None);
    }
}
