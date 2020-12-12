extern crate regex;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod parse;

use std::env;
use std::process;

fn print_usage() {
  println!(
    "
  USAGE: 
  
    adventofcode2020 <DAY>
    
      <DAY>: A positive integer representing the problem you want to solve
  "
  );
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    print_usage();
    process::exit(1);
  }

  let day = args[1].parse::<u32>().expect(&format!(
    "Couldn't parse provided {} as an integer",
    args[1]
  ));

  match day {
    1 => day1::solve(),
    2 => day2::solve(),
    3 => day3::solve(),
    4 => day4::solve(),
    5 => day5::solve(),
    6 => day6::solve(),
    7 => day7::solve(),
    8 => day8::solve(),
    9 => day9::solve(),
    10 => day10::solve(),
    11 => day11::solve(),
    12 => day12::solve(),
    _ => println!("No solution for day {}", day),
  }
}
