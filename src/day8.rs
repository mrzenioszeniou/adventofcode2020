use crate::parse;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve() {
  let lines = parse::read_lines("input/8.txt");

  let (term1, acc1) = execute(&lines);
  assert!(!term1);
  println!("PART 1: {}", acc1);

  for i in 0..lines.len() {
    let mut lines = lines.clone();

    if lines[i].contains("nop") {
      lines[i] = lines[i].replace("nop", "jmp");
    } else if lines[i].contains("jmp") {
      lines[i] = lines[i].replace("jmp", "nop");
    } else {
      continue;
    }

    let (term2, acc2) = execute(&lines);
    if term2 {
      println!("PART 2: {}", acc2);
      return;
    }
  }

  panic!("Couldn't fix the program");
}

/// Returns `true` if the program was executed until the end, and `false` if an infinite loop was
/// detected.
fn execute(lines: &Vec<String>) -> (bool, i32) {
  let re = Regex::from_str("(acc|jmp|nop) (\\+|\\-)([0-9]+)").expect("Couldn't parse regex");

  let mut history: HashSet<usize> = HashSet::new();
  let mut curr = 0;
  let mut acc = 0;

  loop {
    if history.contains(&curr) {
      return (false, acc);
    }

    if curr >= lines.len() {
      return (true, acc);
    }

    history.insert(curr);
    let captures = re.captures(&lines[curr]).unwrap();

    let cmd = captures.get(1).unwrap().as_str();
    let sign: i32 = match captures.get(2).unwrap().as_str() {
      "+" => 1,
      "-" => -1,
      c => panic!("Unexpected sign character {}", c),
    };
    let num = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();

    match cmd {
      "acc" => {
        acc += sign * num;
        curr += 1;
      }
      "jmp" => {
        let new = curr as i32 + (sign * num);
        if new < 0 {
          panic!("Negative address encountered");
        } else {
          curr = new as usize;
        }
      }
      "nop" => {
        curr += 1;
      }
      c => panic!("Unexpected command {}", c),
    }
  }
}
