use std::collections::{HashSet, HashMap};

use anyhow::Result;
use aoclib::strtools;
use itertools::Itertools;

pub fn part1(input: String) -> Result<usize> {
    let recipes: Vec<Recipe<'_>> = input.lines()
        .map(Recipe::from_line)
        .collect();

    let all_allergens: HashSet<&str> = recipes.iter()
        .flat_map(|r| r.allergens().iter().cloned())
        .collect();

    let mut ingredients: HashMap<&str, HashSet<&str>> = recipes.iter()
        .flat_map(|r| r.ingredients().iter().cloned())
        .map(|i| (i, all_allergens.clone()))
        .collect();

    for r in recipes.iter() {
        for a in r.allergens().iter().cloned() {
            for (i, possibles) in ingredients.iter_mut() {
                if !r.ingredients().contains(i) {
                    possibles.remove(a);
                }
            }
        }
    }

    Ok(recipes.iter()
        .flat_map(|r| r.ingredients().iter().cloned())
        .filter(|&i| {
            ingredients.get(i).unwrap().is_empty()
        })
        .count()
    )
}

pub fn part2(input: String) -> Result<String> {
    let recipes: Vec<Recipe<'_>> = input.lines()
        .map(Recipe::from_line)
        .collect();

    let all_allergens: HashSet<&str> = recipes.iter()
        .flat_map(|r| r.allergens().iter().cloned())
        .collect();

    let mut ingredients: HashMap<&str, HashSet<&str>> = recipes.iter()
        .flat_map(|r| r.ingredients().iter().cloned())
        .map(|i| (i, all_allergens.clone()))
        .collect();

    for r in recipes.iter() {
        for a in r.allergens().iter().cloned() {
            for (i, possibles) in ingredients.iter_mut() {
                if !r.ingredients().contains(i) {
                    possibles.remove(a);
                }
            }
        }
    }

    for r in recipes.iter() {
        for a in r.allergens().iter().cloned() {
            let possible_ingredients = r.ingredients().iter().cloned()
                .filter(|&i| ingredients.get(i).unwrap().contains(a));
            if let Ok(i) = possible_ingredients.exactly_one() {
                ingredients.get_mut(i).unwrap().clear();
                ingredients.get_mut(i).unwrap().insert(a);
            }
        }
    }

    let mut decoded: Vec<(&str, &str)> = ingredients.into_iter()
        .filter_map(|(i, a)| {
            assert!(a.is_empty() || a.len() == 1);
            a.into_iter().next().map(|a| (i, a))
        })
        .collect();
    decoded.sort_by_key(|v| v.1);
    Ok(decoded.into_iter().map(|(i, _a)| i).join(","))
}

struct Recipe<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl<'a> Recipe<'a> {
    fn from_line(line: &'a str) -> Self {
        let (ingredients, allergens) = strtools::split_once(line, " (contains ");
        let allergens = allergens.trim_end_matches(')');

        Self {
            ingredients: ingredients.trim().split(' ').collect(),
            allergens: allergens.trim().split(", ").collect(),
        }
    }

    fn ingredients(&self) -> &[&'a str] {
        &self.ingredients
    }

    fn allergens(&self) -> &[&'a str] {
        &self.allergens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            5,
            part1(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
                trh fvjkl sbzzf mxmxvkd (contains dairy)
                sqjhc fvjkl (contains soy)
                sqjhc mxmxvkd sbzzf (contains fish)"
                .to_string()
            ).unwrap(),
        );
    }
}
