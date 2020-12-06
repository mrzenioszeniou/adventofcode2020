use crate::parse;

use std::collections::BTreeSet;

// F -> 0
// B -> 1
// R -> 1
// L -> 0

pub fn solve() {
  let mut seats: BTreeSet<usize> = BTreeSet::new();

  for line in parse::read_lines("input/5.txt") {
    let seat_str = line
      .replace("F", "0")
      .replace("B", "1")
      .replace("R", "1")
      .replace("L", "0");
    let seat =
      usize::from_str_radix(&seat_str, 2).expect(&format!("Couldn't parse {} as binary", seat_str));

    seats.insert(seat);
  }

  // assert_eq!(
  //   1,
  //   seats.len(),
  //   "One remaining seat expected but found {}",
  //   seats.len()
  // );

  println!("PART 1: {}", seats.iter().max().expect("No seats found"));

  print!(
    "PART 2: {}",
    seats
      .iter()
      .zip(seats.iter().skip(1))
      .filter_map(|(&seat_left, &seat_right)| {
        if seat_left + 2 == seat_right {
          Some(seat_left + 1)
        } else {
          None
        }
      })
      .next()
      .expect("No seat found")
  );
}
