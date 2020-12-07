use crate::parse;

use std::collections::HashMap;

pub fn solve() {
  let mut cnt_part_1 = 0;
  let mut cnt_part_2 = 0;

  let mut curr = HashMap::new();
  let mut curr_size = 0;
  let mut group = 1;

  for line in parse::read_lines("input/6.txt") {
    if line.is_empty() {
      cnt_part_1 += curr.len();
      cnt_part_2 += curr
        .iter()
        .filter(|(_, answers)| **answers == curr_size)
        .count();

      println!("Group {} | Size {} | {:?}", group, curr_size, curr);
      group += 1;
      curr = HashMap::new();
      curr_size = 0;
    } else {
      for question in line.chars() {
        match curr.get_mut(&question) {
          Some(answers) => {
            *answers += 1;
          }
          None => {
            curr.insert(question, 1_usize);
          }
        }
      }
      curr_size += 1;
    }
  }

  cnt_part_1 += curr.len();
  cnt_part_2 += curr
    .iter()
    .filter(|(_, answers)| **answers == curr_size)
    .count();

  println!("PART 1: {}", cnt_part_1);
  println!("PART 2: {}", cnt_part_2);
}
