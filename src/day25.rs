pub fn solve() {
  println!("PART 1: {}", part_1(335121, 363891));
  println!("PART 2: Merry Xmas & Happy New Year!");
}

fn part_1(card_pk: usize, door_pk: usize) -> usize {
  let mut card_loop_cnt = 0;
  let mut value = transform(7, 1, None);
  while value != card_pk {
    value = transform(7, 1, Some(value));
    card_loop_cnt += 1;
  }

  transform(door_pk, card_loop_cnt, None)
}

fn transform(subject: usize, loop_size: usize, value: Option<usize>) -> usize {
  let mut value = value.unwrap_or(1);

  for _ in 0..loop_size {
    value *= subject;
    value %= 20201227;
  }

  value
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(transform(7, 8, None), 5764801);
    assert_eq!(transform(7, 11, None), 17807724);
    assert_eq!(part_1(5764801, 17807724, N))
  }
}
