use crate::parse;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::max;
use std::collections::HashMap;

pub fn solve() {
  let coordinates = parse::parse_day24("input/24.txt");
  let mut tiles: HashMap<(isize, isize, isize), bool> = HashMap::new();
  for coordinate in coordinates.iter() {
    match tiles.get_mut(coordinate) {
      Some(is_black) => {
        *is_black = !*is_black;
      }
      None => {
        tiles.insert(coordinate.clone(), true);
      }
    }
  }

  println!("PART 1: {}", tiles.iter().filter(|(_, t)| **t).count());

  let days = 100;
  let mut edge = tiles
    .iter()
    .filter(|(_, b)| **b)
    .map(|(t, _)| max(max(t.0.abs(), t.1.abs()), t.2.abs()))
    .max()
    .unwrap()
    + 1;

  let bar = ProgressBar::new(days as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("[{percent:>3}%] {bar:40.cyan/blue} {elapsed:>3} elapsed | {eta:>3} remaining")
      .progress_chars("##-"),
  );
  for _ in 0..days {
    let mut new_tiles = HashMap::new();

    for i in -edge..=edge {
      for j in -edge..=edge {
        for k in -edge..=edge {
          let coordinates = (i, j, k);
          let black_neighbours = count_neighbours(&coordinates, &tiles);
          let is_black = tiles.get(&coordinates).cloned().unwrap_or(false);

          if is_black && (black_neighbours == 0 || black_neighbours > 2) {
            new_tiles.remove(&coordinates);
          } else if (!is_black && black_neighbours == 2) || is_black {
            let max = max(
              coordinates.0.abs(),
              max(coordinates.1.abs(), coordinates.2.abs()),
            );
            if edge < max + 1 {
              edge = max + 1;
            }
            new_tiles.insert(coordinates, true);
          }
        }
      }
    }

    bar.inc(1);
    tiles = new_tiles;
  }

  bar.finish();

  println!("PART 2: {}", tiles.iter().filter(|(_, t)| **t).count());
}

fn count_neighbours(
  tile: &(isize, isize, isize),
  tiles: &HashMap<(isize, isize, isize), bool>,
) -> usize {
  let neighbours = vec![
    (1, -1, 0),
    (0, -1, 1),
    (-1, 0, 1),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
  ];
  let mut cnt = 0;
  for neighbour in neighbours.into_iter() {
    let mut coordinates = tile.clone();
    coordinates.0 += neighbour.0;
    coordinates.1 += neighbour.1;
    coordinates.2 += neighbour.2;
    if let Some(true) = tiles.get(&coordinates) {
      cnt += 1;
    }
  }

  cnt
}
