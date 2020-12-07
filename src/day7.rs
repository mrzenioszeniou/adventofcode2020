use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

use crate::parse;

pub fn solve() {
  // dim silver bags contain 3 posh fuchsia bags.
  // wavy olive bags contain 1 striped olive bag, 1 dull cyan bag.
  // dull coral bags contain 1 dim olive bag, 5 muted violet bags, 2 dark gray bags.
  // plaid cyan bags contain no other bags.

  let container_re = Regex::from_str("([a-z]+ [a-z]+) bags contain").expect("Couldn't parse regex");
  let content_re = Regex::from_str("([0-9]+) ([a-z]+ [a-z]+) bags?").expect("Couldn't patse regex");
  let leaf_re =
    Regex::from_str("^([a-z]+ [a-z]+) bags contain no other bags.$").expect("Couldn't parse regex");

  let mut rules: HashMap<String, Vec<(String, usize)>> = HashMap::new();

  for line in parse::read_lines("input/7.txt").iter() {
    if leaf_re.find(line).is_some() {
      let captures = leaf_re.captures(line).unwrap();
      let colour = String::from(captures.get(1).unwrap().as_str());
      // println!("Empty: {}", colour);
      rules.insert(colour, vec![]);
    } else if container_re.find(line).is_some() {
      let captures = container_re.captures(line).unwrap();
      let colour = String::from(captures.get(1).unwrap().as_str());
      let mut contents: Vec<(String, usize)> = vec![];
      for content in content_re.captures_iter(line) {
        contents.push((
          String::from(content.get(2).unwrap().as_str()),
          content.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        ));
      }
      // println!("{} \t\t|\t {:?}", colour, contents);
      rules.insert(colour, contents);
    } else {
      panic!("Couldn't match line to any regex: '{}'", line);
    }
  }

  let mut cnt = 0;
  for colour in rules.keys() {
    if colour != "shiny gold" && count_occurences(&rules, colour, "shiny gold") > 0 {
      cnt += 1;
    // println!("✅ {} can carry a bright shiny bag", colour);
    } else {
      // println!("🚫 {} cannot carry a bright shiny bag", colour);
    }
  }
  println!("PART 1: {}", cnt);

  println!("PART 2: {}", count_descendents(&rules, "shiny gold") - 1)
}

fn count_occurences(
  rules: &HashMap<String, Vec<(String, usize)>>,
  curr: &str,
  target: &str,
) -> usize {
  if curr == target {
    1
  } else {
    match rules.get(curr) {
      Some(ref contents) => contents
        .iter()
        .map(|(content, n)| n * count_occurences(rules, content, target))
        .sum(),
      None => 0,
    }
  }
}

fn count_descendents(rules: &HashMap<String, Vec<(String, usize)>>, curr: &str) -> usize {
  let contents: &Vec<(String, usize)> = rules
    .get(curr)
    .expect(&format!("No rule found for {}", curr));

  contents
    .iter()
    .map(|(content, n)| *n * count_descendents(rules, content))
    .sum::<usize>()
    + 1_usize
}
