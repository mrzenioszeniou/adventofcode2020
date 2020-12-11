use crate::parse;

pub fn solve() {
  let initial_seats = parse::read_char_matrix("input/11.txt");

  let mut prev = initial_seats.clone();
  let mut curr = round(&prev, 4, false);
  while curr != prev {
    prev = curr;
    curr = round(&prev, 4, false);
  }
  println!("Part 1: {}", cnt_all_occupied(&curr));

  let mut prev = initial_seats.clone();
  let mut curr = round(&prev, 5, true);
  while curr != prev {
    prev = curr;
    curr = round(&prev, 5, true);
  }

  println!("Part 2: {}", cnt_all_occupied(&curr));
}

fn round(seats: &Vec<Vec<char>>, soc_dis: usize, raycast: bool) -> Vec<Vec<char>> {
  let mut ret = vec![];

  for i in 0..seats.len() {
    let mut row = vec![];
    for j in 0..seats[i].len() {
      let occupied_seats = cnt_occupied_around(seats, i, j, raycast);
      match seats[i][j] {
        '#' if occupied_seats >= soc_dis => row.push('L'),
        'L' if occupied_seats == 0 => row.push('#'),
        c => row.push(c),
      }
    }
    ret.push(row);
  }

  ret
}

fn cnt_occupied_around(
  seats: &Vec<Vec<char>>,
  init_i: usize,
  init_j: usize,
  raycast: bool,
) -> usize {
  let mut ret = 0;

  let m = seats.len() as isize;
  let n = seats[0].len() as isize;

  let steps = vec![
    (0, 1),
    (0, -1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
  ];

  for (step_i, step_j) in steps {
    let mut i = init_i as isize + step_i;
    let mut j = init_j as isize + step_j;
    let mut lim = 1;
    let mut occupied = false;

    while i >= 0 && j >= 0 && i < m && j < n && lim > 0 {
      match seats[i as usize][j as usize] {
        '#' => {
          occupied = true;
          break;
        }
        'L' => break,
        _ => {}
      }
      i += step_i;
      j += step_j;
      if !raycast {
        lim -= 1;
      }
    }

    if occupied {
      ret += 1;
    }
  }

  ret
}

fn cnt_all_occupied(seats: &Vec<Vec<char>>) -> usize {
  let mut occupied = 0;
  for i in 0..seats.len() {
    for j in 0..seats[i].len() {
      if seats[i][j] == '#' {
        occupied += 1;
      }
    }
  }
  occupied
}
