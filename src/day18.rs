use crate::parse;
use regex::{Captures, Regex};
use std::str::FromStr;

pub fn solve() {
  let expressions = parse::read_lines("input/18.txt");

  let mut sum = 0;
  for expression in expressions.iter() {
    sum += eval_1(&mut String::from(expression));
  }

  println!("PART 1: {}", sum);

  let mut sum = 0;
  for expression in expressions.iter() {
    sum += eval_2(&mut String::from(expression));
  }

  println!("PART 2: {}", sum);
}

fn eval_1(exp: &mut String) -> usize {
  let mut num_str = String::new();

  while let Some(c) = exp.pop() {
    match c {
      ' ' => {}
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => num_str.insert(0, c),
      '+' => return num_str.parse::<usize>().unwrap() + eval_1(exp),
      '*' => return num_str.parse::<usize>().unwrap() * eval_1(exp),
      ')' => {
        let par = eval_1(exp);
        exp.push_str(&format!("{}", par));
        // println!("Appending {} | Expression is now: {}", par, exp);
      }
      '(' => return num_str.parse().unwrap(),
      c => panic!("Can't deal with this shit: {}", c),
    }
  }

  num_str.parse().unwrap()
}

fn eval_2(exp: &mut String) -> usize {
  let add_re = Regex::from_str("([0-9]+) \\+ ([0-9]+)").unwrap();
  let mul_re = Regex::from_str("([0-9]+) \\* ([0-9]+)").unwrap();
  let par_re = Regex::from_str("\\(([^\\(\\)]+)\\)").unwrap();

  while par_re.find(exp).is_some() {
    *exp = String::from(par_re.replace(exp, |capt: &Captures| {
      format!(
        "{}",
        eval_2(&mut String::from(capt.get(1).unwrap().as_str()))
      )
    }));
  }

  while add_re.find(exp).is_some() {
    *exp = String::from(add_re.replace(exp, |capt: &Captures| {
      format!(
        "{}",
        capt.get(1).unwrap().as_str().parse::<usize>().unwrap()
          + capt.get(2).unwrap().as_str().parse::<usize>().unwrap()
      )
    }));
  }

  while mul_re.find(exp).is_some() {
    *exp = String::from(mul_re.replace(exp, |capt: &Captures| {
      format!(
        "{}",
        capt.get(1).unwrap().as_str().parse::<usize>().unwrap()
          * capt.get(2).unwrap().as_str().parse::<usize>().unwrap()
      )
    }));
  }

  exp.parse().unwrap()
}

#[test]
fn test_eval() {
  assert_eq!(eval_1(&mut String::from("1 + 2 * 3")), 9);
  assert_eq!(eval_1(&mut String::from("1 + (2 * 3)")), 7);
  assert_eq!(eval_1(&mut String::from("1 + ((2 * 3))")), 7);
  assert_eq!(eval_1(&mut String::from("(1 + 2) + 3 * ((2 * 3))")), 36);
  assert_eq!(eval_1(&mut String::from("1 + 2 * 3 + 4 * 5 + 6")), 71);

  assert_eq!(eval_2(&mut String::from("1 * 2 + 3")), 5);
  assert_eq!(eval_2(&mut String::from("1 * (2 + 3)")), 5);
  assert_eq!(eval_2(&mut String::from("1 * ((2 + 3))")), 5);
  assert_eq!(eval_2(&mut String::from("(1 + 2) * 3 + ((2 * 3))")), 27);
  assert_eq!(eval_2(&mut String::from("1 + 2 * 3 + 4 * 5 + 6")), 231);
}
