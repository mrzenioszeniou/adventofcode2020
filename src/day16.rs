use crate::parse;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn solve() {
  let (my_ticket, nearby_tickets, fields) = parse::parse_day16("input/16.txt");

  // println!("Fields");
  // for (field, lims) in fields.iter() {
  //   println!(" {}: {:?}", field, lims);
  // }
  // println!("\nMy ticket: {:?}", my_ticket);
  // println!("\nNearby tickets:");
  // for ticket in nearby_tickets.iter() {
  //   println!("  {:?}", ticket);
  // }

  println!("PART 1: {}", part1(&nearby_tickets, &fields));
  println!("PART 2: {}", part2(&my_ticket, &nearby_tickets, &fields));
}

fn part1(
  nearby_tickets: &Vec<Vec<usize>>,
  fields: &HashMap<String, (usize, usize, usize, usize)>,
) -> usize {
  let mut ret = 0;

  for ticket in nearby_tickets.iter() {
    for value in ticket.iter() {
      if !fields.values().any(|(min_0, max_0, min_1, max_1)| {
        value >= min_0 && value <= max_0 || value >= min_1 && value <= max_1
      }) {
        ret += value;
      }
    }
  }

  ret
}

fn part2(
  my_ticket: &Vec<usize>,
  nearby_tickets: &Vec<Vec<usize>>,
  fields: &HashMap<String, (usize, usize, usize, usize)>,
) -> usize {
  let mut clean_tickets = vec![];

  'ticket: for ticket in nearby_tickets.iter() {
    for value in ticket.iter() {
      if !fields.values().any(|(min_0, max_0, min_1, max_1)| {
        value >= min_0 && value <= max_0 || value >= min_1 && value <= max_1
      }) {
        continue 'ticket;
      }
    }
    clean_tickets.push(ticket);
  }

  let mut valid_permutations: HashSet<Vec<String>> =
    get_valid_permutations(my_ticket.clone(), fields.clone());

  for (i, ticket) in nearby_tickets.iter().enumerate() {
    let valid_subset = get_valid_permutations(ticket.clone(), fields.clone());
    valid_permutations.retain(|permutation| valid_subset.contains(permutation));
    // println!("{}", i);
  }

  println!("{:?}", valid_permutations);

  0
}

fn get_valid_permutations(
  mut ticket: Vec<usize>,
  fields: HashMap<String, (usize, usize, usize, usize)>,
) -> HashSet<Vec<String>> {
  let mut valid_permutations = HashSet::new();

  // println!("{:?} {:?}", ticket, fields);

  if ticket.is_empty() {
    valid_permutations.insert(vec![]);
    // println!("Returning {:?}", valid_permutations);
    return valid_permutations;
  }

  let value = ticket.pop().unwrap();

  for (field, _) in fields.iter().filter(|(_, (min_0, max_0, min_1, max_1))| {
    (value >= *min_0 && value <= *max_0) || (value >= *min_1 && value <= *max_1)
  }) {
    let mut remaining_fields = fields.clone();
    remaining_fields.retain(|f, _| f != field);

    let valid_sub_permutations = get_valid_permutations(ticket.clone(), remaining_fields);

    if valid_permutations.is_empty() {}

    for mut sub_permutation in valid_sub_permutations.into_iter() {
      sub_permutation.push(String::from(field));
      valid_permutations.insert(sub_permutation);
    }
  }

  valid_permutations
}

#[test]
fn test_get_valid_permutations() {
  let mut ticket = vec![11];
  let mut fields = HashMap::new();
  fields.insert(String::from("class"), (0, 1, 4, 19));
  assert_eq!(
    get_valid_permutations(ticket.clone(), fields.clone()).len(),
    1
  );

  ticket.push(12);
  fields.insert(String::from("row"), (0, 5, 8, 19));
  assert_eq!(
    get_valid_permutations(ticket.clone(), fields.clone()).len(),
    2
  );

  ticket.push(13);
  fields.insert(String::from("seat"), (0, 13, 16, 19));
  assert_eq!(
    get_valid_permutations(ticket.clone(), fields.clone()).len(),
    6
  );
}
