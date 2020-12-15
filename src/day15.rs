use std::collections::HashMap;

pub fn solve() {
  assert_eq!(get_step(vec![0, 3, 6], 10), 0);
  println!("PART 1: {}", get_step(vec![0, 5, 4, 1, 10, 14, 7], 2020));
  println!(
    "PART 2: {}",
    get_step(vec![0, 5, 4, 1, 10, 14, 7], 30_000_000)
  );
}

fn get_step(numbers: Vec<usize>, turn: usize) -> usize {
  let mut history: Vec<usize> = numbers.clone();
  let mut ledger: HashMap<usize, Vec<usize>> = HashMap::new();

  for (turn, number) in history.iter().enumerate() {
    ledger.insert(*number, vec![turn + 1]);
  }

  while history.len() < turn {
    let last = history.last().unwrap();
    let next = ledger
      .get(last)
      .filter(|turns| turns.len() > 1)
      .map(|turns| turns.iter().rev().skip(1).next().map(|n| history.len() - n))
      .flatten()
      .unwrap_or(0);
    history.push(next);
    match ledger.get_mut(&next) {
      Some(turns) => turns.push(history.len()),
      None => {
        ledger.insert(next, vec![history.len()]);
      }
    }
  }

  history[turn - 1]
}
