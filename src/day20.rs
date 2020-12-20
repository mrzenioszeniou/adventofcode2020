use crate::parse;

pub fn solve() {
  let parsed_tiles = parse::parse_day20("input/20.txt");

  println!("PART 1: {} tiles parsed", parsed_tiles.len());
  println!("PART 2: Bar");
}
