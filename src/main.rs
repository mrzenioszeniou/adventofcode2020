extern crate regex;

mod parse;

use std::collections::HashMap;
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

fn day1() {
  let values = parse::read_int_list("input/1.txt");

  'outer: for (i, val_i) in values.iter().enumerate() {
    for (j, val_j) in values.iter().enumerate() {
      if i == j {
        continue;
      } else if val_i + val_j == 2020 {
        println!("PART 1: i={}, j={}, i*j={}", val_i, val_j, val_i * val_j);
        break 'outer;
      }
    }
  }

  for (i, val_i) in values.iter().enumerate() {
    for (j, val_j) in values.iter().enumerate() {
      for (k, val_k) in values.iter().enumerate() {
        if i == j || i == k || j == k {
          continue;
        } else if val_i + val_j + val_k == 2020 {
          println!(
            "PART 2: i={}, j={}, K={} i*j*k={}",
            val_i,
            val_j,
            val_k,
            val_i * val_j * val_k
          );
          return;
        }
      }
    }
  }
}

fn day2() {
  let input = parse::parse_day2("input/2.txt");

  let mut valid_cnt = 0;
  'outer: for ((character, min, max), password) in input.clone() {
    let mut characters: HashMap<char, usize> = HashMap::new();
    for character in password.chars() {
      match characters.get_mut(&character) {
        Some(cnt) => {
          *cnt += 1;
        }
        None => {
          characters.insert(character, 1);
        }
      }
    }

    match characters.get(&character) {
      Some(cnt) if *cnt < min || *cnt > max => continue 'outer,
      None if min > 0 => continue 'outer,
      _ => valid_cnt += 1,
    }
  }
  println!("PART 1: {}", valid_cnt);

  let mut valid_cnt = 0;
  for ((character, pos_a, pos_b), password) in input {
    let pos_a = password.chars().skip(pos_a - 1).next().unwrap() == character;
    let pos_b = password.chars().skip(pos_b - 1).next().unwrap() == character;
    if pos_a != pos_b {
      valid_cnt += 1;
    }
  }
  println!("PART 2: {}", valid_cnt);
}

fn day3() {
  let input = parse::parse_day3("input/3.txt");

  let steps = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
  let mut trees = 1;

  for (step_i, step_j) in steps.into_iter() {
    let mut i = 0;
    let mut j = 0;
    let mut cnt = 0;
    while i < input.len() {
      if input[i][j] {
        cnt += 1;
      }
      i += step_i;
      j = (j + step_j) % input[0].len();
    }
    if step_i == 1 && step_j == 3 {
      println!("PART 1: {}", cnt);
    }
    trees *= cnt;
  }
  println!("PART 2: {}", trees);
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
    1 => day1(),
    2 => day2(),
    3 => day3(),
    _ => println!("No solution for day {}", day),
  }
}
