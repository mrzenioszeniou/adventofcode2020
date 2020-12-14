use crate::parse;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve() {
  let commands = parse::read_lines("input/14.txt");

  part1(&commands);
  part2(&commands);
}

fn part1(commands: &Vec<String>) {
  let mut memory: HashMap<usize, usize> = HashMap::new();

  let maskupdate_re = Regex::from_str("mask = ([01X]+)").expect("Couldn't parse mask regex");
  let memaccess_re =
    Regex::from_str("mem\\[([0-9]+)\\] = ([0-9]+)").expect("Couldn't parse access regex");

  let mut zero_mask = 0;
  let mut one_mask = 0;
  for command in commands.iter() {
    match maskupdate_re.captures(command) {
      Some(captures) => {
        let masks = parse_mask(captures.get(1).unwrap().as_str());
        zero_mask = masks.0;
        one_mask = masks.1;
      }
      None => {}
    }
    match memaccess_re.captures(command) {
      Some(captures) => {
        let address: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let mut value: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        value &= zero_mask;
        value |= one_mask;
        match memory.get_mut(&address) {
          Some(v) => {
            *v = value;
          }
          None => {
            memory.insert(address, value);
          }
        }
      }
      None => {}
    }
  }

  let res: usize = memory.values().sum();

  println!("PART 1: {}", res);
}

fn part2(commands: &Vec<String>) {
  let mut memory: HashMap<usize, usize> = HashMap::new();

  let maskupdate_re = Regex::from_str("mask = ([01X]+)").expect("Couldn't parse mask regex");
  let memaccess_re =
    Regex::from_str("mem\\[([0-9]+)\\] = ([0-9]+)").expect("Couldn't parse access regex");

  let mut mask = "";

  for command in commands.iter() {
    match maskupdate_re.captures(command) {
      Some(captures) => mask = captures.get(1).unwrap().as_str(),
      None => {}
    }

    match memaccess_re.captures(command) {
      Some(captures) => {
        let init_address: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let value: usize = captures.get(2).unwrap().as_str().parse().unwrap();

        for address in mask_adress(mask, init_address) {
          match memory.get_mut(&address) {
            Some(v) => {
              *v = value;
            }
            None => {
              memory.insert(address, value);
            }
          }
        }
      }
      None => {}
    }
  }

  let res: usize = memory.values().sum();

  println!("PART 2: {}", res);
}

fn parse_mask(mask: &str) -> (usize, usize) {
  let zero_mask = usize::from_str_radix(&mask.replace("X", "1"), 2)
    .expect(&format!("Couldn't parse zero mask '{}'", mask));
  let one_mask = usize::from_str_radix(&mask.replace("X", "0"), 2)
    .expect(&format!("Couldn't parse one mask '{}'", mask));

  (zero_mask, one_mask)
}

fn mask_adress(mask: &str, address: usize) -> Vec<usize> {
  let mut addresses = vec![address];
  let mut bit_num = 1_usize;

  for bit_char in mask.chars().rev() {
    match bit_char {
      '1' => {
        for address in addresses.iter_mut() {
          *address |= bit_num;
        }
      }
      '0' => {}
      'X' => {
        let mut new_addresses = vec![];
        for address in addresses {
          new_addresses.push(address | bit_num);
          new_addresses.push(address & !bit_num);
        }
        addresses = new_addresses;
      }
      _ => panic!("Unexpected mask bit {}", bit_char),
    }
    bit_num *= 2;
  }

  // println!("{:064b}:", address);
  // for address in addresses.iter() {
  //   println!("  {:064b}", address);
  // }

  addresses
}
