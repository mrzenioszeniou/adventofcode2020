use crate::parse;

use std::collections::HashMap;

pub fn solve() {
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
