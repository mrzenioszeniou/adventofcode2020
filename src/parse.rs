use std::collections::HashMap;
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

pub fn parse_day17(filepath: &str) -> Vec<Vec<Vec<usize>>> {
  let ext = 15;

  let mut init = vec![];

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
    init.push(init_line);
  }

  let n = init[0].len();

  for _ in 0..ext {
    init.insert(0, vec![0; n]);
    init.push(vec![0; n]);
  }

  let mut ret = vec![init];

  for _ in 0..ext {
    ret.insert(0, vec![vec![0; n]; n]);
    ret.push(vec![vec![0; n]; n]);
  }

  ret
}
