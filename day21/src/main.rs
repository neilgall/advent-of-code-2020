use std::collections::{HashMap, HashSet};
use parser::*;

// -- model

#[derive(Debug, PartialEq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

impl Food {
    fn new(ingredients: &[&str], allergens: &[&str]) -> Self {
        Food {
            ingredients: ingredients.iter().map(|i| i.to_string()).collect(),
            allergens: allergens.iter().map(|a| a.to_string()).collect()
        }
    }
}

#[derive(Debug)]
struct Assoc(HashMap<String, HashSet<String>>);

impl Assoc {
    fn new(foods: &[Food]) -> Self {
        let mut assoc: HashMap<String, HashSet<String>> = HashMap::new();

        for food in foods.iter() {
            for allergen in food.allergens.iter() {
                match assoc.get_mut(allergen) {
                    Some(existing) => {
                        *existing = existing.intersection(&food.ingredients).cloned().collect();
                    }
                    None => {
                        assoc.insert(allergen.to_string(), food.ingredients.clone());
                    }
                }
            }
        }

        Assoc(assoc)
    }

    fn is_fully_determined(&self) -> bool {
        self.0.values().all(|ingredients| ingredients.len() < 2)
    }

    fn eliminate_duplicate_matches(&mut self) {
        let determined: HashSet<String> = self.0.values()
            .filter_map(|ingredients|
                if ingredients.len() == 1 { ingredients.iter().next() } else { None }
            ).cloned().collect();

        for ingredients in self.0.values_mut().filter(|ings| ings.len() > 1) {
            *ingredients = ingredients.difference(&determined).cloned().collect();
        }
    }

    fn ingredients_with_allergen(&self) -> HashSet<String> {
        self.0.values().flat_map(|values| values.iter().cloned()).collect()
    }
}


// -- parser

fn parse_input(input: &str) -> ParseResult<Vec<Food>> {
    let ingredients = one_or_more(whitespace_wrap(identifier));
    let allergens = identifier
        .sep_by(match_literal(", "))
        .between(match_literal("(contains "), match_literal(")"));

    let food = pair(ingredients, allergens, |ingredients, allergens| Food {
        ingredients: ingredients.into_iter().collect(),
        allergens: allergens.into_iter().collect()
    });

    one_or_more(whitespace_wrap(food)).parse(input)
}

// -- problems 

fn part1(foods: &Vec<Food>) -> usize {
    let mut assoc = Assoc::new(&foods[..]);
    while !assoc.is_fully_determined() {
        assoc.eliminate_duplicate_matches();
    }
    let all_ingredients: HashSet<String> = foods.iter().flat_map(|food| food.ingredients.iter().cloned()).collect();
    let ingredients_with_allergen = assoc.ingredients_with_allergen();

    let ingredients_with_no_allergen: HashSet<String> = all_ingredients.difference(&ingredients_with_allergen).cloned().collect();

    foods.iter().map(|food|
        food.ingredients.intersection(&ingredients_with_no_allergen).count()
    ).sum()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let foods = parse_input(&input).unwrap().1;

    println!("part 1 {}", part1(&foods));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_foods() -> Vec<Food> {
        vec![
            Food::new(
                &["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                &["dairy", "fish"]
            ),
            Food::new(
                &["trh", "fvjkl", "sbzzf", "mxmxvkd"],
                &["dairy"]
            ),
            Food::new(
                &["sqjhc", "fvjkl"],
                &["soy"]
            ),
            Food::new(
                &["sqjhc", "mxmxvkd", "sbzzf"],
                &["fish"]
            )
        ]
    }

    #[test]
    fn test_parser() {
        let foods = parse_input("
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        ");
        assert_eq!(foods, Ok(("", test_foods())));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_foods()), 5);
    }
}
