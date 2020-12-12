use crate::parse;

pub fn solve() {
  let instructions = parse::parse_day12("input/12.txt");
  part1(&instructions);
  part2(&instructions);
}

fn part1(instructions: &Vec<(char, isize)>) {
  let mut x: isize = 0;
  let mut y: isize = 0;
  let mut angle: (isize, isize) = (1, 0);
  for instruction in instructions.iter() {
    match instruction {
      ('N', n) => y += n,
      ('S', n) => y -= n,
      ('E', n) => x += n,
      ('W', n) => x -= n,
      ('L', n) => rotate_left(&mut angle.0, &mut angle.1, *n),
      ('R', n) => rotate_right(&mut angle.0, &mut angle.1, *n),
      ('F', n) => {
        x += n * angle.0;
        y += n * angle.1;
      }
      (action, _) => panic!("Invalid action {}", action),
    }
  }
  println!("Part 1: {}", x.abs() + y.abs());
}

fn part2(instructions: &Vec<(char, isize)>) {
  let mut x = 0_isize;
  let mut y = 0_isize;
  let mut wx = 10_isize;
  let mut wy = 1_isize;

  for instruction in instructions.iter() {
    match instruction {
      ('N', n) => wy += *n,
      ('S', n) => wy -= *n,
      ('E', n) => wx += *n,
      ('W', n) => wx -= *n,
      ('L', n) => rotate_left(&mut wx, &mut wy, *n),
      ('R', n) => rotate_right(&mut wx, &mut wy, *n),
      ('F', n) => {
        x += wx * *n;
        y += wy * *n;
      }
      (action, _) => panic!("Invalid action {}", action),
    }
  }

  println!("Part 2: {}", x.abs() + y.abs());
}

fn rotate_left(x: &mut isize, y: &mut isize, n: isize) {
  for _ in 0..n / 90 {
    let tmp = *x;
    *x = -*y;
    *y = tmp;
  }
}

fn rotate_right(x: &mut isize, y: &mut isize, n: isize) {
  for _ in 0..n / 90 {
    let tmp = -*x;
    *x = *y;
    *y = tmp;
  }
}
