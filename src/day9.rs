use crate::parse;
use std::collections::HashSet;

pub fn solve() {
  let nums = parse::read_int_list("input/9.txt");

  let mut slice: Vec<u64> = nums.iter().take(25).cloned().collect();

  let mut invalid = 25;

  for num in nums.iter().skip(25) {
    if !get_all_sums(&slice).contains(num) {
      println!("PART 1: {}", num);
      break;
    }
    slice.remove(0);
    slice.push(*num);
    invalid += 1;
  }

  let mut range = None;

  'outer: for n in 2..=nums.len() {
    for i in 0..nums.len() - n + 1 {
      let sum = nums.iter().skip(i).take(n).sum::<u64>();
      if sum > nums[invalid] {
        continue 'outer;
      } else if sum == nums[invalid] {
        range = Some((i, n));
        break 'outer;
      }
    }
  }

  match range {
    Some((i, n)) => println!(
      "PART 2: {} ({},{})",
      nums.iter().skip(i).take(n).max().unwrap() + nums.iter().skip(i).take(n).min().unwrap(),
      i,
      n
    ),
    None => panic!("NO SOLUTION FOUND FOR PART 2"),
  }
}

fn get_all_sums(nums: &Vec<u64>) -> HashSet<u64> {
  let mut ret = HashSet::new();

  for i in 0..nums.len() {
    for j in i + 1..nums.len() {
      ret.insert(nums[i] + nums[j]);
    }
  }

  ret
}
