extern crate indicatif;
extern crate regex;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
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
    13 => day13::solve(),
    14 => day14::solve(),
    15 => day15::solve(),
    16 => day16::solve(),
    17 => day17::solve(),
    18 => day18::solve(),
    19 => day19::solve(),
    20 => day20::solve(),
    21 => day21::solve(),
    22 => day22::solve(),
    _ => println!("No solution available for day {}", day),
  }
}
