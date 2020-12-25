use crate::parse;
use std::collections::HashSet;

pub fn solve() {
  let starting_decks = parse::parse_day22("input/22.txt");

  let mut player_1 = starting_decks.0.clone();
  let mut player_2 = starting_decks.1.clone();
  while !player_1.is_empty() && !player_2.is_empty() {
    combat(&mut player_1, &mut player_2);
  }

  let ans_1: usize = if player_2.is_empty() {
    player_1
      .into_iter()
      .rev()
      .enumerate()
      .map(|(i, c)| (i + 1) * c)
      .sum()
  } else {
    player_2
      .into_iter()
      .rev()
      .enumerate()
      .map(|(i, c)| (i + 1) * c)
      .sum()
  };

  println!("PART 1: {}", ans_1);

  let mut player_1 = starting_decks.0.clone();
  let mut player_2 = starting_decks.1.clone();
  let ans_2: usize = if recursive_combat(&mut player_1, &mut player_2) {
    player_1
      .into_iter()
      .rev()
      .enumerate()
      .map(|(i, c)| (i + 1) * c)
      .sum()
  } else {
    player_2
      .into_iter()
      .rev()
      .enumerate()
      .map(|(i, c)| (i + 1) * c)
      .sum()
  };

  println!("PART 2: {}", ans_2);
}

fn combat(player_1: &mut Vec<usize>, player_2: &mut Vec<usize>) {
  let p1_card = player_1.remove(0);
  let p2_card = player_2.remove(0);

  if p1_card > p2_card {
    player_1.push(p1_card);
    player_1.push(p2_card);
  } else if p2_card > p1_card {
    player_2.push(p2_card);
    player_2.push(p1_card);
  } else {
    panic!(
      "Tie detected: P1:{:?}, P2:{:?}, {}=={}",
      player_1, player_2, p1_card, p2_card
    );
  }
}

/// Returns `true` if player 1 won, `false` otherwise.
fn recursive_combat(player_1: &mut Vec<usize>, player_2: &mut Vec<usize>) -> bool {
  let mut history: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

  while !player_1.is_empty() && !player_2.is_empty() {
    let turn = (player_1.clone(), player_2.clone());

    if history.contains(&turn) {
      return true;
    } else {
      history.insert(turn);
    }

    let p1_card = player_1.remove(0);
    let p2_card = player_2.remove(0);

    if player_1.len() >= p1_card && player_2.len() >= p2_card {
      let mut sub_player_1: Vec<usize> = player_1.iter().take(p1_card).cloned().collect();
      let mut sub_player_2: Vec<usize> = player_2.iter().take(p2_card).cloned().collect();

      if recursive_combat(&mut sub_player_1, &mut sub_player_2) {
        player_1.push(p1_card);
        player_1.push(p2_card);
      } else {
        player_2.push(p2_card);
        player_2.push(p1_card);
      }
    } else {
      if p1_card > p2_card {
        player_1.push(p1_card);
        player_1.push(p2_card);
      } else if p2_card > p1_card {
        player_2.push(p2_card);
        player_2.push(p1_card);
      } else {
        panic!(
          "Tie detected: P1:{:?}, P2:{:?}, {}=={}",
          player_1, player_2, p1_card, p2_card
        );
      }
    }
  }

  player_2.is_empty()
}
