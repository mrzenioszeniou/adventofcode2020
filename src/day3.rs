use crate::parse;

pub fn solve() {
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
