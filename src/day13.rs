use crate::parse;

pub fn solve() {
  let (start, buses) = parse::parse_day13_part1("input/13.txt");

  let mut best = (buses[0], calc_earliest(start, buses[0]));
  for &bus in buses.iter().skip(1) {
    let earliest = calc_earliest(start, bus);
    if earliest < best.1 {
      best = (bus, earliest);
    }
  }
  println!("PART 1: {}", (best.1 - start) * best.0);

  let mut buses: Vec<(usize, usize)> = vec![];

  for (i, bus) in parse::read_lines("input/13.txt")[1].split(",").enumerate() {
    match bus {
      "x" => {}
      bus => {
        buses.push((i, bus.parse().unwrap()));
      }
    }
  }

  let mut num = buses[0].1;
  let mut step = buses[0].1;
  for (i, bus) in buses.iter().skip(1) {
    while (num + i) % bus != 0 {
      num += step;
    }
    step *= bus;
  }

  println!("PART 2: {}", num);
}

pub fn calc_earliest(start: usize, bus: usize) -> usize {
  if start % bus == 0 {
    0
  } else {
    (start / bus + 1) * bus
  }
}
