use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Write;

pub fn solve() {
  println!("PART 1: {}", part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]));
  println!("PART 2: {}", part_2(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]));
}

/// Solves Part 1
fn part_1(mut cups: Vec<usize>) -> String {
  rounds(&mut cups, 9, 100);

  let start = cups.iter().position(|&e| e == 1).unwrap();

  let mut ans = String::new();

  for i in 1..9 {
    write!(ans, "{}", cups[(start + i) % cups.len()]).unwrap();
  }

  ans
}

/// Solves Part 2
fn part_2(mut cups: Vec<usize>) -> usize {
  rounds(&mut cups, 1_000_000, 10_000_000);

  let start = cups.iter().position(|&e| e == 1).unwrap();

  cups[(start + 1) % cups.len()] * cups[(start + 2) % cups.len()]
}

fn rounds(cups: &mut Vec<usize>, lim: usize, rounds: usize) {
  let m = cups.iter().max().unwrap().clone();

  cups.append(&mut (m + 1..=lim).collect());

  let mut current_idx = 0;
  let bar = ProgressBar::new(rounds as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("[{percent:>3}%] {bar:40.cyan/blue} {eta} remaining")
      .progress_chars("##-"),
  );
  for _ in 0..rounds {
    current_idx = round(cups, current_idx, lim);
    bar.inc(1);
  }

  bar.finish();
}

fn round(cups: &mut Vec<usize>, current_idx: usize, max: usize) -> usize {
  let current = cups[current_idx].clone();
  let mut pickups = vec![];
  for _ in 0..3 {
    if current_idx + 1 < cups.len() {
      pickups.push(cups.remove(current_idx + 1));
    } else {
      pickups.push(cups.remove(0));
    }
  }

  // print!("Cups: ");
  // for (i, cup) in cups.iter().enumerate() {
  //   if i == current_idx {
  //     print!("({}) ", cup);
  //   } else {
  //     print!(" {}  ", cup);
  //   }
  // }
  // println!("Pickups: {:?}", pickups);

  let mut destination = current - 1;
  loop {
    if destination == 0 {
      destination = max;
    } else if pickups.contains(&destination) {
      destination -= 1;
    } else {
      break;
    }
  }

  let destination_idx = cups.iter().position(|&e| e == destination).unwrap();
  cups.insert(destination_idx + 1, pickups.pop().unwrap());
  cups.insert(destination_idx + 1, pickups.pop().unwrap());
  cups.insert(destination_idx + 1, pickups.pop().unwrap());

  if destination_idx > current_idx {
    current_idx + 1
  } else {
    (cups.iter().position(|&e| e == current).unwrap() + 1) % cups.len()
  }
}

#[test]
fn test() {
  assert_eq!(part_1(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]), "67384529");
  assert_eq!(part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]), "95648732");
  // assert_eq!(part_2(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]), 149245887792);
}
