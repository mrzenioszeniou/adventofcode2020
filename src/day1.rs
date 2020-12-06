use crate::parse;

pub fn solve() {
  let values = parse::read_int_list("input/1.txt");

  'outer: for (i, val_i) in values.iter().enumerate() {
    for (j, val_j) in values.iter().enumerate() {
      if i == j {
        continue;
      } else if val_i + val_j == 2020 {
        println!("PART 1: i={}, j={}, i*j={}", val_i, val_j, val_i * val_j);
        break 'outer;
      }
    }
  }

  for (i, val_i) in values.iter().enumerate() {
    for (j, val_j) in values.iter().enumerate() {
      for (k, val_k) in values.iter().enumerate() {
        if i == j || i == k || j == k {
          continue;
        } else if val_i + val_j + val_k == 2020 {
          println!(
            "PART 2: i={}, j={}, K={} i*j*k={}",
            val_i,
            val_j,
            val_k,
            val_i * val_j * val_k
          );
          return;
        }
      }
    }
  }
}
