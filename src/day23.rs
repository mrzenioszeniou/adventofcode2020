use std::fmt::Write;

pub fn solve() {
  println!("PART 1: {}", part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]));
}

fn part_1(mut cups: Vec<usize>) -> String {
  let mut current = cups[0];

  for _ in 0..100 {
    let current_idx = cups.iter().position(|&e| e == current).unwrap();

    let mut pickups = vec![];
    for _ in 0..3 {
      if current_idx + 1 < cups.len() {
        pickups.push(cups.remove(current_idx + 1));
      } else {
        pickups.push(cups.remove(0));
      }
    }
    let mut destination = current - 1;
    while !cups.contains(&destination) {
      if destination == 0 {
        destination = cups.iter().max().unwrap().clone();
      } else {
        destination -= 1;
      }
    }
    let destination_idx = cups.iter().position(|&e| e == destination).unwrap();
    cups.insert(destination_idx + 1, pickups.pop().unwrap());
    cups.insert(destination_idx + 1, pickups.pop().unwrap());
    cups.insert(destination_idx + 1, pickups.pop().unwrap());
    current = cups[(cups.iter().position(|&e| e == current).unwrap() + 1) % cups.len()];
  }

  let start = cups.iter().position(|&e| e == 1).unwrap();

  let mut ans = String::new();

  for i in 1..9 {
    write!(ans, "{}", cups[(start + i) % cups.len()]).unwrap();
  }

  ans
}

#[test]
fn test_part_1() {
  assert_eq!(part_1(vec![3, 6, 8, 1, 9, 5, 7, 4, 2]), "95648732");
}
