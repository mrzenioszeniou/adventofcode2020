use crate::parse;
use std::collections::HashMap;

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
    clean_tickets.push(ticket.clone());
  }

  let mut my_ticket = my_ticket.clone();
  let mut fields = fields.clone();
  let mut ret = 1;

  while !fields.is_empty() {
    // Find a column that matches only one field
    let mut valid_field = None;
    for i in 0..clean_tickets[0].len() {
      let mut valid_fields = fields
        .iter()
        .filter(|(_, (min_0, max_0, min_1, max_1))| {
          clean_tickets
            .iter()
            .map(|ticket| ticket[i])
            .chain(my_ticket[i]..=my_ticket[i])
            .all(|val| val >= *min_0 && val <= *max_0 || val >= *min_1 && val <= *max_1)
        })
        .map(|field| field.0)
        .collect::<Vec<_>>();

      if valid_fields.len() == 1 {
        valid_field = Some((i, valid_fields.pop().unwrap().clone()));
        break;
      }
    }

    // Remove that field and column
    match valid_field {
      Some(valid_field) => {
        if valid_field.1.starts_with("departure") {
          ret *= my_ticket[valid_field.0];
        }

        my_ticket.remove(valid_field.0);
        fields.remove(&valid_field.1);

        for ticket in clean_tickets.iter_mut() {
          (*ticket).remove(valid_field.0);
        }
      }
      None => panic!("No field matching the same column of values could be found"),
    }
  }

  ret
}
