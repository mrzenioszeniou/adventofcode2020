use crate::parse;

use regex::Regex;
use std::str::FromStr;

pub fn solve() {
  let passports = parse::parse_day4("input/4.txt");
  let hgt_cm_regex = Regex::from_str("([0-9]+)cm").unwrap();
  let hgt_in_regex = Regex::from_str("([0-9]+)in").unwrap();
  let hcl_regex = Regex::from_str("#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]").unwrap();
  let pid_regex = Regex::from_str("^[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]$").unwrap();

  assert!(hgt_in_regex.find("60in").is_some());
  assert!(hgt_cm_regex.find("190cm").is_some());
  assert!(hgt_in_regex.find("190in").is_some());
  assert!(hgt_cm_regex.find("190").is_none());
  assert!(hgt_in_regex.find("190").is_none());

  assert!(hcl_regex.find("#123abc").is_some());
  assert!(hcl_regex.find("#123abz").is_none());
  assert!(hcl_regex.find("123abc").is_none());

  assert!(pid_regex.find("000000001").is_some());
  assert!(pid_regex.find("0123456789").is_none());

  let mut valids_1 = 0_usize;
  let mut valids_2 = 0_usize;

  for passport in passports.into_iter() {
    let mut valid_1 = true;
    let mut valid_2 = true;

    match passport.get("byr") {
      Some(byr) => {
        let byr = byr.parse::<u32>().unwrap();
        if byr < 1920 || byr > 2002 {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    match passport.get("iyr") {
      Some(iyr) => {
        let iyr = iyr.parse::<u32>().unwrap();
        if iyr < 2010 || iyr > 2020 {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    match passport.get("eyr") {
      Some(eyr) => {
        let eyr = eyr.parse::<u32>().unwrap();
        if eyr < 2020 || eyr > 2030 {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    match passport.get("hgt") {
      Some(hgt) => {
        if let Some(captures) = hgt_cm_regex.captures(hgt) {
          let hgt = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
          if hgt < 150 || hgt > 193 {
            valid_2 = false;
          }
        } else if let Some(captures) = hgt_in_regex.captures(hgt) {
          let hgt = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
          if hgt < 59 || hgt > 76 {
            valid_2 = false;
          }
        } else {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    match passport.get("hcl") {
      Some(hcl) => {
        if hcl_regex.find(hcl).is_none() {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    match passport.get("ecl").map(|s| s.trim()) {
      Some("amb") | Some("blu") | Some("brn") | Some("gry") | Some("grn") | Some("hzl")
      | Some("oth") => {}
      None => {
        valid_1 = false;
        valid_2 = false;
      }
      _ => valid_2 = false,
    }

    match passport.get("pid") {
      Some(pid) => {
        if pid_regex.find(pid).is_none() {
          valid_2 = false;
        }
      }
      None => {
        valid_1 = false;
        valid_2 = false;
      }
    }

    if valid_1 {
      valids_1 += 1;
    }

    if valid_2 {
      valids_2 += 1;
    }
  }

  println!("PART 1: {}", valids_1);
  println!("PART 2: {}", valids_2);
}
