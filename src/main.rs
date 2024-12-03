use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::process::ExitCode;
use crate::util::Errors;

mod util;
mod day01;
mod day02;
mod day03;

trait Day {
    fn part_1(&self) -> Result<String, Errors>;
    fn part_2(&self) -> Result<String, Errors>;
    fn create_day() -> Box<dyn Day> where Self: Sized;
}

#[allow(unstable_name_collisions)]
fn main() -> ExitCode {
    let available_days: HashMap<u8, Box<dyn Day>> = HashMap::from([
        (1u8, Day01::create_day()),
        (2u8, Day02::create_day()),
        (3u8, Day03::create_day()),
    ]);

    let args = Args::parse();

    let days_to_run: Vec<u8> = if args.all {
        available_days.keys().map(u8::clone).sorted().collect_vec()
    } else {
        args.days
    };

    if days_to_run.is_empty() {
        println!("No days provided.");
        ExitCode::from(1)
    } else if days_to_run.iter().any(|x| !available_days.contains_key(x)) {
        println!("Days available: {}", available_days.iter().map(|(x, _)| format!("{}", *x)).intersperse(String::from(" ")).collect::<String>());
        ExitCode::from(2)
    } else {
        for day in days_to_run {
            println!("Running day: {}", day);
            match available_days.get(&day).unwrap().part_1() {
                Ok(answer) => println!("--> Part 1: {}", answer),
                Err(err) => println!("--> Part 1 error: {:?}", err),
            }

            match available_days.get(&day).unwrap().part_2() {
                Ok(answer) => println!("--> Part 2: {}", answer),
                Err(err) => println!("--> Part 2 error: {:?}", err),
            }
        }

        ExitCode::SUCCESS
    }
}

#[derive(Debug, Parser)]
struct Args {
    days: Vec<u8>,

    #[arg(short, long)]
    all: bool
}
