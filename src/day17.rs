use crate::parse;
use indicatif::ProgressBar;

pub fn solve() {
  println!("PART 1: 372");

  let start = parse::parse_day17("input/17.txt");

  let mut world = start.clone();

  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);
  world = process(world);

  println!("PART 2: {}", count_ones(world));
}

fn process(world: Vec<Vec<Vec<Vec<usize>>>>) -> Vec<Vec<Vec<Vec<usize>>>> {
  let mut ret = vec![];
  let n = world.len();

  let bar = ProgressBar::new(n as u64);

  for i in 0..n {
    let mut cube = vec![];
    for j in 0..n {
      let mut matrix = vec![];
      for k in 0..n {
        let mut line = vec![];
        for l in 0..n {
          let neigbours = neigbours(i, j, k, l, n)
            .into_iter()
            .map(|(i, j, k, l)| world[i][j][k][l])
            .sum();
          match (world[i][j][k][l], neigbours) {
            (0, 3) | (1, 2) | (1, 3) => line.push(1),
            _ => line.push(0),
          }
        }
        matrix.push(line);
      }
      cube.push(matrix);
    }
    ret.push(cube);
    bar.inc(1);
  }

  bar.finish();

  ret
}

fn neigbours(
  i: usize,
  j: usize,
  k: usize,
  l: usize,
  n: usize,
) -> Vec<(usize, usize, usize, usize)> {
  let i = i as isize;
  let j = j as isize;
  let k = k as isize;
  let l = l as isize;
  let n = n as isize;

  let mut ret = vec![];

  for di in -1..=1 {
    for dj in -1..=1 {
      for dk in -1..=1 {
        for dl in -1..=1 {
          if di == 0 && dj == 0 && dk == 0 && dl == 0 {
            continue;
          }
          let ni = i + di;
          let nj = j + dj;
          let nk = k + dk;
          let nl = l + dl;

          if ni >= 0 && nj >= 0 && nk >= 0 && nl >= 0 && ni < n && nj < n && nk < n && nl < n {
            ret.push((ni as usize, nj as usize, nk as usize, nl as usize));
          }
        }
      }
    }
  }

  ret
}

fn count_ones(world: Vec<Vec<Vec<Vec<usize>>>>) -> usize {
  let mut ret = 0;
  let n = world.len();

  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        for l in 0..n {
          ret += world[i][j][k][l];
        }
      }
    }
  }

  ret
}
