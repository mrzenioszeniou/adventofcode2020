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

  let tile_ids: HashSet<usize> = parsed_tiles.keys().cloned().collect();

  let ledger = build_ledger(parsed_tiles);

  let image = assemble(tile_ids, &ledger, vec![]).expect("No solution for PART 1");
  let ans1 = image[0][0].0 * image[0][11].0 * image[11][0].0 * image[11][11].0;

  println!("PART 1: {}", ans1);

  let mut cleaned_image = vec![vec![0_u8; 96]; 96];

  for (tile_i, tile_line) in image.iter().enumerate() {
    for (tile_j, (_, tile)) in tile_line.iter().enumerate() {
      let tile = clean_tile(tile);

      for i in 0..8 {
        for j in 0..8 {
          let global_i = tile_i * 8 + i;
          let global_j = tile_j * 8 + j;

          cleaned_image[global_i][global_j] = tile[i][j];
        }
      }
    }
  }

  let cleaned_image = flip(&cleaned_image);

  println!("PART 2: {}", count_rough(&cleaned_image));
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

  // Hack to speed up runs after part 1 is done. This if was not here when I was calculating it
  if image.is_empty() {
    fitting.retain(|(tile_id, _)| *tile_id == 2221);
  }

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

  for (next_id, next_tile) in fitting.into_iter() {
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

fn is_monster(image: &Tile, image_i: usize, image_j: usize) -> bool {
  let pattern: Vec<Vec<usize>> = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
  ];

  if image_i + pattern.len() > image.len() || image_j + pattern[0].len() > image[0].len() {
    return false;
  }

  for i in 0..pattern.len() {
    for j in 0..pattern[i].len() {
      if pattern[i][j] == 1 && image[image_i + i][image_j + j] == 0 {
        return false;
      }
    }
  }

  true
}

fn clean_tile(tile: &Tile) -> Tile {
  let mut ret = tile.clone();

  ret.remove(tile.len() - 1);
  ret.remove(0);

  for line in ret.iter_mut() {
    line.remove(line.len() - 1);
    line.remove(0);
  }

  ret
}

fn count_rough(image: &Vec<Vec<u8>>) -> usize {
  let mut rough = 0;
  let mut monsters = 0;

  for i in 0..image.len() {
    for j in 0..image[i].len() {
      rough += image[i][j] as usize;
      if is_monster(&image, i, j) {
        monsters += 1;
      }
    }
  }

  rough - monsters * 15
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

#[test]
fn test_clean_tile() {
  let tile = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

  assert_eq!(clean_tile(&tile), vec![vec![4]]);
}

#[test]
fn test_is_monster() {
  let pattern = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
  ];
  assert_eq!(is_monster(&pattern, 0, 0), false);

  let pattern = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
  ];
  assert_eq!(is_monster(&pattern, 0, 0), true);

  let pattern = vec![
    vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0],
    vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0],
  ];

  assert_eq!(is_monster(&pattern, 0, 0), true);
}

#[test]
fn test_count_rough() {
  let image = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
  ];
  assert_eq!(count_rough(&image), 14);

  let image = vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    vec![1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
  ];
  assert_eq!(count_rough(&image), 0);

  let image = vec![
    vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0],
    vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1],
    vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0],
  ];
  assert_eq!(count_rough(&image), 13);

  let image = vec![
    vec![
      0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0,
    ],
    vec![
      1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0,
    ],
    vec![
      0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0,
    ],
    vec![
      1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1,
    ],
    vec![
      0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1,
    ],
    vec![
      0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1,
    ],
    vec![
      1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0,
    ],
    vec![
      0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,
    ],
    vec![
      1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0,
    ],
    vec![
      1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1,
    ],
    vec![
      0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1,
    ],
    vec![
      0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0,
    ],
    vec![
      0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0,
    ],
    vec![
      1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0,
    ],
    vec![
      0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1,
    ],
    vec![
      1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0,
    ],
    vec![
      1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0,
    ],
    vec![
      0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0,
    ],
    vec![
      0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1,
    ],
    vec![
      1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0,
    ],
    vec![
      1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0,
    ],
    vec![
      1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1,
    ],
    vec![
      1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1,
    ],
    vec![
      1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1,
    ],
  ];

  assert_eq!(count_rough(&image), 273);
}
