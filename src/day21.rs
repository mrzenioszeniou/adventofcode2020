use crate::parse;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn solve() {
  let foods = parse::parse_day21("input/21.txt");

  // Key: Allergen, Value: Ingredients
  let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
  // Key: Ingredient, Value: Times it appears
  let mut ingredients: HashMap<String, usize> = HashMap::new();

  for (food_ingredients, food_allergens) in foods.iter() {
    for allergen in food_allergens.iter() {
      match allergens.get_mut(allergen) {
        Some(ingredients) => {
          ingredients.retain(|i| food_ingredients.contains(i));
        }
        None => {
          allergens.insert(
            allergen.clone(),
            HashSet::from_iter(food_ingredients.iter().cloned()),
          );
        }
      }
    }

    for ingredient in food_ingredients.iter() {
      match ingredients.get_mut(ingredient) {
        Some(ingredient_cnt) => {
          *ingredient_cnt += 1;
        }
        None => {
          ingredients.insert(ingredient.clone(), 1);
        }
      }
    }
  }

  let mut ans_1 = 0;
  for (ingredient, cnt) in ingredients.iter() {
    if !allergens.values().any(|i| i.contains(ingredient)) {
      ans_1 += cnt;
    }
  }
  println!("PART 1: {}", ans_1);

  let mut ans_2 = vec![];
  while !allergens.is_empty() {
    let next = allergens
      .iter()
      .filter(|(_, ingredients)| ingredients.len() == 1)
      .next()
      .expect("OOPS");
    let allergen = next.0.clone();
    let ingredient = next.1.iter().next().unwrap().clone();

    for (_, ingredients) in allergens.iter_mut() {
      ingredients.remove(&ingredient);
    }

    allergens.remove(&allergen);
    ans_2.push((allergen, ingredient));
  }

  ans_2.sort_by_key(|e| e.0.clone());

  print!("PART 2: ");
  for (_, ingredient) in ans_2.iter() {
    print!("{},", ingredient);
  }
}
