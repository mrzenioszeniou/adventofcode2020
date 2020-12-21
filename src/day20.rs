use crate::parse;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type Tile = Vec<Vec<u8>>;
type Tiles = HashMap<usize, Tile>;
type Ledger = HashMap<(Side, Vec<u8>), HashSet<(usize, Tile)>>;
type Image = Vec<Vec<(usize, Tile)>>;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Side {
  Top,
  Bottom,
  Left,
  Right,
}

pub fn solve() {
  let parsed_tiles = parse::parse_day20("input/20.txt");

  println!("PART 1: {}", part1(parsed_tiles));
  println!("PART 2: Bar");
}

///
/// Actual solution
///
/// 2221 2687 2657 3163 2843 3203 1097 3541 3617 3433 3881 1723
/// 2243 3169 3793 3469 2381 1579 1933 3217 2879 2591 1453 1697
/// 1223 3989 1459 1489 1973 2713 2003 3361 3467 1669 2273 1721
/// 1777 2549 2857 3643 1657 3253 1153 3343 2531 1759 3041 2459
/// 1297 3517 3917 1543 2897 3851 3823 3533 3391 2683 2029 3923
/// 1063 3331 2927 3221 3209 2207 3191 1229 1861 2357 3803 2039
/// 1249 1483 3769 2063 1279 3797 3779 1663 2749 2137 3089 2539
/// 1549 1259 2767 3929 2081 3119 1867 2711 1117 3889 2633 1187
/// 1811 1621 3413 3373 3463 1801 1583 2237 2203 1033 1171 1787
/// 3527 1301 2969 3329 3623 1123 2671 1741 2143 2729 1181 1747
/// 1907 1627 1319 3697 3323 3919 1951 1429 1523 2293 1013 2087
/// 1511 3581 2999 3943 3511 3271 2659 3299 1487 2833 2129 2287
///
/// -> 13224049461431
///
fn part1(tiles: Tiles) -> usize {
  let remaining_tiles: HashSet<usize> = tiles.keys().cloned().collect();
  let ledger = build_ledger(tiles);

  // for ((side, side_vals), tiles) in ledger.into_iter() {
  //   println!(
  //     "{:?} on the {:?}: {:?}",
  //     side_vals,
  //     side,
  //     tiles
  //       .iter()
  //       .map(|(tile_id, _)| *tile_id)
  //       .collect::<Vec<_>>()
  //   );
  // }

  let image = assemble(remaining_tiles, &ledger, vec![]).expect("No solution for PART 1");

  for row in image.iter() {
    for tile in row.iter() {
      print!("{} ", tile.0);
    }
    println!();
  }

  image[0][0].0 * image[0][11].0 * image[11][0].0 * image[11][11].0
}

fn assemble(remaining_tiles: HashSet<usize>, ledger: &Ledger, image: Image) -> Option<Image> {
  if remaining_tiles.is_empty() {
    return Some(image);
  }

  let mut fitting: HashSet<(usize, Tile)> = HashSet::from_iter(
    ledger
      .values()
      .map(|set| set.iter())
      .flatten()
      .filter(|(tile_id, _)| remaining_tiles.contains(tile_id))
      .cloned(),
  );
  let mut next_i = 0;

  if !image.is_empty() {
    // Figure out which the last tile was
    let last_i = image.len() - 1;
    let last_j = image[last_i].len() - 1;
    // What's the next row and column we want to add?
    let next_j = (last_j + 1) % 12;
    next_i = if next_j == 0 { last_i + 1 } else { last_i };

    // Filter based on what's left of the new tile
    if next_j > 0 {
      match ledger.get(&(Side::Left, right(&image[last_i][last_j].1))) {
        Some(valid) => fitting.retain(|t| valid.contains(t)),
        None => {}
      }
    }

    // Filter based on what's above the new tile
    if next_i > 0 {
      match ledger.get(&(Side::Top, bottom(&image[next_i - 1][next_j].1))) {
        Some(valid) => fitting.retain(|t| valid.contains(t)),
        None => {}
      }
    }
  };

  let n_fitting = fitting.len();
  let mut i = 0;

  for (next_id, next_tile) in fitting.into_iter() {
    if image.is_empty() {
      i += 1;
      println!("{}/{}", i, n_fitting);
    }

    let mut remaining_tiles = remaining_tiles.clone();
    remaining_tiles.remove(&next_id);

    let mut image = image.clone();
    if next_i >= image.len() {
      image.push(vec![]);
    }
    image[next_i].push((next_id, next_tile));

    match assemble(remaining_tiles, ledger, image) {
      Some(assembled_image) => return Some(assembled_image),
      None => {}
    }
  }

  None
}

fn build_ledger(tiles: Tiles) -> Ledger {
  let mut ret: Ledger = HashMap::new();

  for (tile_id, mut tile) in tiles.into_iter() {
    for _ in 0..4 {
      // Add top side to the ledger
      let key = (Side::Top, top(&tile));
      let val = (tile_id, tile.clone());
      attach(key, val, &mut ret);

      // Add bottom side to the ledger
      let key = (Side::Bottom, bottom(&tile));
      let val = (tile_id, tile.clone());
      attach(key, val, &mut ret);

      // Add left side to the ledger
      let key = (Side::Left, left(&tile));
      let val = (tile_id, tile.clone());
      attach(key, val, &mut ret);

      // Add right side to the ledger
      let key = (Side::Right, right(&tile));
      let val = (tile_id, tile.clone());
      attach(key, val, &mut ret);

      // Flip tile
      let flipped = flip(&tile);

      // Add top side to the ledger
      let key = (Side::Top, top(&flipped));
      let val = (tile_id, flipped.clone());
      attach(key, val, &mut ret);

      // Add bottom side to the ledger
      let key = (Side::Bottom, bottom(&flipped));
      let val = (tile_id, flipped.clone());
      attach(key, val, &mut ret);

      // Add left side to the ledger
      let key = (Side::Left, left(&flipped));
      let val = (tile_id, flipped.clone());
      attach(key, val, &mut ret);

      // Add right side to the ledger
      let key = (Side::Right, right(&flipped));
      let val = (tile_id, flipped.clone());
      attach(key, val, &mut ret);

      tile = rotate(&tile);
    }
  }

  ret
}

fn top(tile: &Tile) -> Vec<u8> {
  tile[0].clone()
}

fn bottom(tile: &Tile) -> Vec<u8> {
  tile[tile.len() - 1].clone()
}

fn left(tile: &Tile) -> Vec<u8> {
  tile.iter().map(|line| line[0]).collect()
}

fn right(tile: &Tile) -> Vec<u8> {
  tile.iter().map(|line| line[line.len() - 1]).collect()
}

fn rotate(tile: &Tile) -> Tile {
  let mut ret = vec![];

  let n = tile.len();

  for j in 0..n {
    let mut line = vec![];
    for i in 1..=n {
      line.push(tile[n - i][j]);
    }
    ret.push(line);
  }

  ret
}

fn flip(tile: &Tile) -> Tile {
  tile.iter().rev().cloned().collect()
}

fn attach(
  key: (Side, Vec<u8>),
  val: (usize, Tile),
  ledger: &mut HashMap<(Side, Vec<u8>), HashSet<(usize, Tile)>>,
) {
  match ledger.get_mut(&key) {
    Some(tiles) => {
      tiles.insert(val);
    }
    None => {
      let mut tiles = HashSet::new();
      tiles.insert(val);
      ledger.insert(key, tiles);
    }
  }
}

#[test]
fn test_rotate() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(rotate(&tile), vec![vec![2, 0], vec![3, 1]]);
}

#[test]
fn test_flip() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(flip(&tile), vec![vec![2, 3], vec![0, 1]]);
}

#[test]
fn test_top() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(top(&tile), vec![0, 1]);
}

#[test]
fn test_bottom() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(bottom(&tile), vec![2, 3]);
}

#[test]
fn test_left() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(left(&tile), vec![0, 2]);
}

#[test]
fn test_right() {
  let tile = vec![vec![0, 1], vec![2, 3]];
  assert_eq!(right(&tile), vec![1, 3]);
}

#[test]
fn test_build_ledger() {
  let mut tiles = HashMap::new();
  tiles.insert(42, vec![vec![0, 1], vec![2, 3]]);

  let ledger = build_ledger(tiles);

  assert_eq!(ledger.len(), 32);
}
