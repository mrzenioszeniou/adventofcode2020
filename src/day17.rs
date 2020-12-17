use crate::parse;

pub fn solve() {
  let start = parse::parse_day17("input/17.txt");

  let mut world = start.clone();

  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);

  println!("PART 1: {}", count_ones(world));
}

fn process(world: Vec<Vec<Vec<usize>>>) -> Vec<Vec<Vec<usize>>> {
  let mut ret = vec![];
  let n = world.len();

  for i in 0..n {
    let mut matrix = vec![];
    for j in 0..n {
      let mut line = vec![];
      for k in 0..n {
        let neigbours = neigbours(i, j, k, n)
          .into_iter()
          .map(|(i, j, k)| world[i][j][k])
          .sum();
        match (world[i][j][k], neigbours) {
          (0, 3) | (1, 2) | (1, 3) => line.push(1),
          _ => line.push(0),
        }
      }
      matrix.push(line);
    }
    ret.push(matrix);
  }

  ret
}

fn neigbours(i: usize, j: usize, k: usize, n: usize) -> Vec<(usize, usize, usize)> {
  let i = i as isize;
  let j = j as isize;
  let k = k as isize;
  let n = n as isize;

  let mut ret = vec![];

  for di in -1..=1 {
    for dj in -1..=1 {
      for dk in -1..=1 {
        if di == 0 && dj == 0 && dk == 0 {
          continue;
        }
        let ni = i + di;
        let nj = j + dj;
        let nk = k + dk;

        if ni >= 0 && nj >= 0 && nk >= 0 && ni < n && nj < n && nk < n {
          ret.push((ni as usize, nj as usize, nk as usize));
        }
      }
    }
  }

  ret
}

fn count_ones(world: Vec<Vec<Vec<usize>>>) -> usize {
  let mut ret = 0;
  let n = world.len();

  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        ret += world[i][j][k];
      }
    }
  }

  ret
}

#[test]
fn test_neigbours() {
  assert_eq!(neigbours(0, 0, 0, 1).len(), 0);

  assert_eq!(neigbours(0, 0, 0, 2).len(), 7);
  assert_eq!(neigbours(0, 0, 1, 2).len(), 7);
  assert_eq!(neigbours(0, 1, 0, 2).len(), 7);
  assert_eq!(neigbours(0, 1, 1, 2).len(), 7);
  assert_eq!(neigbours(1, 0, 0, 2).len(), 7);
  assert_eq!(neigbours(1, 0, 1, 2).len(), 7);
  assert_eq!(neigbours(1, 1, 0, 2).len(), 7);
  assert_eq!(neigbours(1, 1, 1, 2).len(), 7);

  assert_eq!(neigbours(0, 0, 1, 3).len(), 11);
  assert_eq!(neigbours(0, 1, 0, 3).len(), 11);
  assert_eq!(neigbours(1, 0, 0, 3).len(), 11);
  assert_eq!(neigbours(1, 1, 0, 3).len(), 17);
  assert_eq!(neigbours(1, 0, 1, 3).len(), 17);
  assert_eq!(neigbours(0, 1, 1, 3).len(), 17);

  assert_eq!(neigbours(1, 1, 1, 3).len(), 26);
}

#[test]
fn test_process_and_count() {
  let mut world = parse::parse_day17("input/17_example.txt");

  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);

  assert_eq!(count_ones(world), 112);
}
