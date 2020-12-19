use crate::parse;

pub fn solve() {
  let rules = parse::parse_day19_rules("input/19_rules.txt");

  let mut valid_1 = 0;
  let mut valid_2 = 0;
  for mut line in parse::read_lines("input/19_messages.txt") {
    if rules.get(&0).unwrap().contains(&line) {
      valid_1 += 1;
    }

    let mut cnt_42 = 0;
    let mut cnt_31 = 0;

    while line.len() >= 8 && rules.get(&42).unwrap().contains(&line[0..8]) {
      line = String::from(&line[8..]);
      cnt_42 += 1;
    }

    while line.len() >= 8 && rules.get(&31).unwrap().contains(&line[0..8]) {
      line = String::from(&line[8..]);
      cnt_31 += 1;
    }

    if line.is_empty() && cnt_31 > 0 && cnt_42 > cnt_31 {
      valid_2 += 1;
    }
  }

  println!("PART 1: {}", valid_1);
  println!("PART 2: {}", valid_2);
}
