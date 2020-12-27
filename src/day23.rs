use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Write;

pub fn solve() {
  println!("PART 1: {}", part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]));
  println!("PART 2: {}", part_2(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]));
}

/// Solves Part 1
fn part_1(cups: Vec<usize>) -> String {
  let next = rounds(cups, 9, 100);

  let mut ans = String::new();

  let mut curr = 1;
  for _ in 0..8 {
    write!(ans, "{}", next[curr]).unwrap();
    curr = next[curr];
  }

  ans
}

/// Solves Part 2
fn part_2(cups: Vec<usize>) -> usize {
  let next = rounds(cups, 1_000_000, 10_000_000);

  next[1] * next[next[1]]
}

/// Performs a number of game rounds on the provided arrangement of cups
fn rounds(mut cups: Vec<usize>, lim: usize, rounds: usize) -> Vec<usize> {
  let m = cups.iter().max().unwrap().clone();

  cups.append(&mut (m + 1..=lim).collect());
  assert_eq!(lim, cups.len());

  let mut next = vec![0; cups.len() + 1];
  for i in 0..cups.len() {
    next[cups[i]] = cups[(i + 1) % cups.len()];
  }

  let mut current = cups[0];
  let bar = ProgressBar::new(rounds as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("[{percent:>3}%] {bar:40.cyan/blue} {elapsed:>3} elapsed | {eta:>3} remaining")
      .progress_chars("##-"),
  );
  for _ in 0..rounds {
    current = round(&mut next, current);
    bar.inc(1);
  }

  bar.finish();

  next
}

/// Performs one game round on the provided arrangement of cups
fn round(next: &mut Vec<usize>, current: usize) -> usize {
  let max = next.len() - 1;

  let pickups = vec![
    next[current],
    next[next[current]],
    next[next[next[current]]],
  ];

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

  next[current] = next[pickups[2]];
  next[pickups[2]] = next[destination];
  next[destination] = pickups[0];

  next[current]
}

#[test]
fn test() {
  assert_eq!(part_1(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]), "67384529");
  assert_eq!(part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]), "95648732");
  // assert_eq!(part_2(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]), 149245887792);
}
