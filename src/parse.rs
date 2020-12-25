use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use regex::Regex;

pub fn read_int_list(filepath: &str) -> Vec<u64> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  let mut ret = vec![];

  for line in raw_data.split_whitespace() {
    ret.push(
      line
        .parse::<u64>()
        .expect(&format!("Couldn't parse {} as an integer", line)),
    );
  }

  ret
}

pub fn read_lines(filepath: &str) -> Vec<String> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  raw_data
    .split("\n")
    .map(|s| String::from(s.trim()))
    .collect()
}

pub fn read_char_matrix(filepath: &str) -> Vec<Vec<char>> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  raw_data
    .split("\n")
    .map(|s| s.trim().chars().collect())
    .collect()
}

pub fn parse_day2(filepath: &str) -> Vec<((char, usize, usize), String)> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  let mut ret = vec![];
  let regex = Regex::from_str("([0-9]+)-([0-9]+)\\s+([a-z]):\\s+([a-z]+)").unwrap();
  for line in raw_data.split("\n") {
    let line: String = String::from(line.trim());
    let captures = regex
      .captures(&line)
      .expect("Couldn't parse line with regex");
    let min = captures
      .get(1)
      .expect("Couldn't parse min with regex")
      .as_str()
      .parse::<usize>()
      .unwrap();
    let max = captures
      .get(2)
      .expect("Couldn't parse max with regex")
      .as_str()
      .parse::<usize>()
      .unwrap();
    let character = captures
      .get(3)
      .expect("Couldn't parse char with regex")
      .as_str()
      .chars()
      .next()
      .unwrap();
    let password = String::from(
      captures
        .get(4)
        .expect("Couldn't parse password with regex")
        .as_str(),
    );

    ret.push(((character, min, max), password));
  }

  ret
}

pub fn parse_day3(filepath: &str) -> Vec<Vec<bool>> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  let mut ret = vec![];
  for line in raw_data.split("\n") {
    let mut line_vec = vec![];
    for character in line.chars() {
      match character {
        '.' => line_vec.push(false),
        '#' => line_vec.push(true),
        c => panic!("Unexpected character {} in {}", c, filepath),
      }
    }
    ret.push(line_vec);
  }

  ret
}

pub fn parse_day4(filepath: &str) -> Vec<HashMap<String, String>> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  let mut ret = vec![];
  let mut curr = HashMap::new();
  for line in raw_data.split("\n") {
    let line = String::from(String::from(line).trim());

    if line.is_empty() {
      if !curr.is_empty() {
        ret.push(curr);
      }
      curr = HashMap::new();
    } else {
      for field in line.split_whitespace() {
        let name = field.split(":").next().unwrap();
        let value = field.split(":").skip(1).next().unwrap();

        curr.insert(String::from(name), String::from(value));
      }
    }
  }

  if !curr.is_empty() {
    ret.push(curr);
  }

  ret
}

pub fn parse_day12(filepath: &str) -> Vec<(char, isize)> {
  let path = PathBuf::from_str(filepath).unwrap();
  let mut raw_data = String::new();
  let _ = File::open(path)
    .unwrap()
    .read_to_string(&mut raw_data)
    .unwrap();

  let re = Regex::from_str("([NSEWLRF])([0-9]+)").expect("Couldn't parse regex");

  let mut ret = vec![];
  for line in raw_data.split("\n") {
    let captures = re
      .captures(&line)
      .expect(&format!("Couldn't capture regex in {}", line));
    let action = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let number = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
    ret.push((action, number));
  }

  ret
}

pub fn parse_day13_part1(filepath: &str) -> (usize, Vec<usize>) {
  let raw_data = read_lines(filepath);

  let start: usize = raw_data[0].parse().expect("Couldn't parse start time");
  let buses: Vec<usize> = raw_data[1]
    .split(",")
    .filter(|s| *s != "x")
    .map(|s| s.parse().expect("Couldn't parse bus"))
    .collect();

  (start, buses)
}

pub fn parse_day16(
  filepath: &str,
) -> (
  Vec<usize>,
  Vec<Vec<usize>>,
  HashMap<String, (usize, usize, usize, usize)>,
) {
  let field_re = Regex::from_str("(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)")
    .expect("Couldn't parse field regex");

  let mut my_ticket = vec![];
  let mut nearby_tickets = vec![];
  let mut fields = HashMap::new();
  let mut mine = false;

  for line in read_lines(filepath).iter() {
    if line.trim().is_empty() {
      continue;
    } else if line.trim() == "your ticket:" {
      mine = true;
    } else if line.trim() == "nearby tickets:" {
      mine = false;
    } else if let Some(captures) = field_re.captures(line) {
      let field = String::from(captures.get(1).unwrap().as_str());
      let min_0 = captures.get(2).unwrap().as_str().parse().unwrap();
      let max_0 = captures.get(3).unwrap().as_str().parse().unwrap();
      let min_1 = captures.get(4).unwrap().as_str().parse().unwrap();
      let max_1 = captures.get(5).unwrap().as_str().parse().unwrap();
      fields.insert(field, (min_0, max_0, min_1, max_1));
    } else {
      let mut ticket = vec![];
      for val_str in line.split(",") {
        ticket.push(
          val_str
            .parse()
            .expect(&format!("Couldn't parse {} as a usize", val_str)),
        );
      }

      if mine {
        my_ticket = ticket;
      } else {
        nearby_tickets.push(ticket);
      }
    }
  }

  (my_ticket, nearby_tickets, fields)
}

pub fn parse_day17(filepath: &str) -> Vec<Vec<Vec<Vec<usize>>>> {
  let ext = 15;

  let mut init_matrix = vec![];

  for line in read_char_matrix(filepath).into_iter() {
    let mut init_line = vec![0; ext];
    for character in line.into_iter() {
      init_line.push(match character {
        '.' => 0,
        '#' => 1,
        c => panic!("Unexpected character {}", c),
      });
    }
    init_line.append(&mut vec![0; ext]);
    init_matrix.push(init_line);
  }

  let n = init_matrix[0].len();

  for _ in 0..ext {
    init_matrix.insert(0, vec![0; n]);
    init_matrix.push(vec![0; n]);
  }

  let mut init_cube = vec![init_matrix];

  for _ in 0..ext {
    init_cube.insert(0, vec![vec![0; n]; n]);
    init_cube.push(vec![vec![0; n]; n]);
  }

  let mut ret = vec![init_cube];

  for _ in 0..ext {
    ret.insert(0, vec![vec![vec![0; n]; n]; n]);
    ret.push(vec![vec![vec![0; n]; n]; n]);
  }

  ret
}

pub fn parse_day19_rules(filepath: &str) -> HashMap<usize, HashSet<String>> {
  let term_re = Regex::from_str("^([0-9]+): \"([a-z]+)\"$").expect("Couldn't parse regex");
  let rule_re =
    Regex::from_str("([0-9]+):\\s*([0-9]+)\\s*([0-9]+)?\\s*\\|?\\s*([0-9]+)?\\s*([0-9]+)?")
      .expect("Could parse regex");
  let mut ret: HashMap<usize, HashSet<String>> = HashMap::new();
  let mut lines = read_lines(filepath);

  'outer: while !lines.is_empty() {
    // if lines.len() == 1 {
    //   println!("Only '{}' left", lines[0]);
    // } else {
    //   println!("{} lines left", lines.len());
    // }

    for line_idx in 0..lines.len() {
      // Check for terminal definition
      if let Some(captures) = term_re.captures(&lines[line_idx]) {
        let rule: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let val = String::from(captures.get(2).unwrap().as_str());

        match ret.get_mut(&rule) {
          Some(vals) => {
            vals.insert(val);
          }
          None => {
            let mut vals = HashSet::new();
            vals.insert(val);
            ret.insert(rule, vals);
          }
        }

        lines.remove(line_idx);
        continue 'outer;
      }
      // Check for rules definition
      else if let Some(captures) = rule_re.captures(&lines[line_idx]) {
        let rule: Vec<Option<usize>> = captures
          .iter()
          .skip(1)
          .map(|c| c.map(|r| r.as_str().parse().unwrap()))
          .collect();

        if rule
          .iter()
          .skip(1)
          .all(|r| r.map(|r| ret.contains_key(&r)).unwrap_or(true))
        {
          let mut definition: HashSet<String> = HashSet::new();

          let rule_id: usize = rule[0].unwrap();

          let subrule_0: HashSet<String> = ret.get(&rule[1].unwrap()).unwrap().clone();

          if let Some(subrule_1) = rule[2] {
            let subrule_1 = ret.get(&subrule_1).unwrap().clone();
            for subrules_0 in subrule_0.iter() {
              for subrules_1 in subrule_1.iter() {
                definition.insert(format!("{}{}", subrules_0, subrules_1));
              }
            }
          } else {
            definition.extend(subrule_0);
          }

          if let Some(subrule_2) = rule[3] {
            let subrule_2 = ret.get(&subrule_2).unwrap().clone();

            if let Some(subrule_3) = rule[4] {
              let subrule_3 = ret.get(&subrule_3).unwrap().clone();

              for subrules_2 in subrule_2.iter() {
                for subrules_3 in subrule_3.iter() {
                  definition.insert(format!("{}{}", subrules_2, subrules_3));
                }
              }
            } else {
              definition.extend(subrule_2);
            }
          }

          ret.insert(rule_id, definition);

          lines.remove(line_idx);
          continue 'outer;
        }
      } else {
        panic!("Can't match line {}", line_idx);
      }
    }
    panic!("This shouldn't have happened");
  }

  ret
}

pub fn parse_day20(filepath: &str) -> HashMap<usize, Vec<Vec<u8>>> {
  let tile_re = Regex::from_str("Tile ([0-9]+):").expect("Couldn't parse tile regex");

  let mut ret = HashMap::new();

  let mut tile_id = 0;
  let mut tile = vec![];

  for line in read_lines(filepath).into_iter() {
    if line.is_empty() {
      ret.insert(tile_id, tile);
      tile = vec![];
    } else if let Some(captures) = tile_re.captures(&line) {
      tile_id = captures.get(1).unwrap().as_str().parse().unwrap();
    } else {
      tile.push(
        line
          .chars()
          .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("Dafuq is this: {}", c),
          })
          .collect(),
      );
    }
  }

  if !tile.is_empty() {
    ret.insert(tile_id, tile);
  }

  ret
}

pub fn parse_day21(filepath: &str) -> HashMap<Vec<String>, Vec<String>> {
  let mut ret = HashMap::new();

  let re = Regex::from_str("([a-z ]+) \\(contains ([a-z ,]+)\\)").expect("Couldn't parse regex");

  for line in read_lines(filepath).iter() {
    let recipe = re
      .captures(line)
      .unwrap()
      .get(1)
      .unwrap()
      .as_str()
      .split(" ")
      .map(str::trim)
      .map(String::from)
      .collect();
    let allergens = re
      .captures(line)
      .unwrap()
      .get(2)
      .unwrap()
      .as_str()
      .split(",")
      .map(str::trim)
      .map(String::from)
      .collect();

    ret.insert(recipe, allergens);
  }

  ret
}

pub fn parse_day22(filepath: &str) -> (Vec<usize>, Vec<usize>) {
  let mut player_1 = vec![];
  let mut player_2 = vec![];

  let mut player: usize = 0;

  for line in read_lines(filepath).into_iter() {
    if line == "Player 1:" {
      player = 1;
    } else if line == "Player 2:" {
      player = 2;
    } else if !line.is_empty() {
      match player {
        1 => {
          player_1.push(line.parse().expect("Couldn't parse card"));
        }
        2 => {
          player_2.push(line.parse().expect("Couldn't parse card"));
        }
        _ => panic!("Found a card, but the player is set to {}", player),
      }
    }
  }

  (player_1, player_2)
}
