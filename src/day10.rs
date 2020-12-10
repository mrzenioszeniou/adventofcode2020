use crate::parse;

use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

pub fn solve() {
  let mut parsed_ints: BTreeSet<u64> =
    BTreeSet::from_iter(parse::read_int_list("input/10.txt").into_iter());

  let my_device = parsed_ints.iter().max().expect("No adapters found") + 3;

  parsed_ints.insert(my_device);

  let mut adapters = parsed_ints.clone();

  let mut prev = 0;
  let mut cnt_1 = 0;
  let mut cnt_3 = 0;
  while !adapters.is_empty() {
    let min = adapters.iter().min().unwrap().clone();
    match min - prev {
      3 => cnt_3 += 1,
      2 => {}
      1 => cnt_1 += 1,
      0 => {}
      _ => panic!("Can't find an adapter that fits {}", prev),
    }
    adapters.remove(&min);
    prev = min;
  }

  println!("PART 1: {}", cnt_1 * cnt_3);

  let mut solutions = BTreeMap::new();
  solutions.insert(my_device, 1);
  traverse(&parsed_ints, &mut solutions, 0);
  println!("PART 2: {}", solutions.get(&0).unwrap());
}

fn traverse(adapters: &BTreeSet<u64>, solutions: &mut BTreeMap<u64, usize>, curr: u64) {
  if solutions.contains_key(&curr) {
    return;
  }

  let mut cnt = 0;
  for each in adapters.range(curr + 1..curr + 4) {
    traverse(adapters, solutions, *each);
    cnt += solutions.get(&each).unwrap();
  }

  solutions.insert(curr, cnt);
}
