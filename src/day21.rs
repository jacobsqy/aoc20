use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
    }
}

fn part1(input: &str) -> u32 {
    let mut maybe_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: Vec<&str> = Vec::new();
    for (ingredients, allergens) in input
        .lines()
        .map(|l| l[..l.len() - 1].split_once(" (contains ").unwrap())
        .map(|(ingredients, allergens)| {
            (
                ingredients
                    .split_ascii_whitespace()
                    .collect::<HashSet<&str>>(),
                allergens.split(", "),
            )
        })
    {
        for allergen in allergens {
            maybe_allergen
                .entry(allergen)
                .and_modify(|x| {
                    *x = x
                        .intersection(&ingredients)
                        .copied()
                        .collect::<HashSet<&str>>()
                })
                .or_insert(ingredients.clone());
        }
        all_ingredients.append(&mut ingredients.iter().copied().collect::<Vec<&str>>().clone());
    }

    let first: HashSet<&str> = maybe_allergen.values().next().unwrap().clone();
    let allergen_ingredients = maybe_allergen
        .values()
        .fold(first, |x, y| x.union(y).copied().collect::<HashSet<&str>>());
    all_ingredients.retain(|i| !allergen_ingredients.contains(i));
    all_ingredients.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut maybe_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: Vec<&str> = Vec::new();
    for (ingredients, allergens) in input
        .lines()
        .map(|l| l[..l.len() - 1].split_once(" (contains ").unwrap())
        .map(|(ingredients, allergens)| {
            (
                ingredients
                    .split_ascii_whitespace()
                    .collect::<HashSet<&str>>(),
                allergens.split(", "),
            )
        })
    {
        for allergen in allergens {
            maybe_allergen
                .entry(allergen)
                .and_modify(|x| {
                    *x = x
                        .intersection(&ingredients)
                        .copied()
                        .collect::<HashSet<&str>>()
                })
                .or_insert(ingredients.clone());
        }
        all_ingredients.append(&mut ingredients.iter().copied().collect::<Vec<&str>>().clone());
    }
    let mut result = foo(maybe_allergen);
    result.sort_by_key(|x| x.0);
    println!(
        "{:?}",
        result.iter().map(|x| x.1).collect::<Vec<&str>>().join(",")
    );
    0
}

fn foo<'a>(input: HashMap<&'a str, HashSet<&'a str>>) -> Vec<(&'a str, &'a str)> {
    let ingredient_to_remove = input
        .iter()
        .filter(|(_k, v)| v.len() == 1)
        .map(|(k, v)| (*k, *v.iter().next().unwrap()))
        .next()
        .unwrap();

    let mut result = Vec::new();

    result.push(ingredient_to_remove);

    if input.len() > 1 {
        result.append(&mut foo(input
            .into_iter()
            .map(|(k, mut v)| {
                v.remove(ingredient_to_remove.1);
                (k, v)
            })
            .filter(|(_k, v)| v.len() > 0)
            .collect()));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
