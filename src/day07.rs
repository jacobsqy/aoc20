use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(parse(&puzzle.input)))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn parse(input: &str) -> HashMap<&str, Vec<(u32, &str)>> {
    let mut result: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" contain ");
        let bag = split.next().unwrap().rsplit_once(' ').unwrap().0;
        let content = split.next().unwrap().split(", ");
        for num_bag in content {
            if num_bag != "no other bags." {
                let num_bag = num_bag.split_once(' ').unwrap();
                let num_bag = (
                    num_bag.0.parse::<u32>().unwrap(),
                    num_bag.1.rsplit_once(' ').unwrap().0,
                );
                result
                    .entry(bag)
                    .and_modify(|v| v.push(num_bag))
                    .or_insert(vec![num_bag]);
            }
        }
    }
    result
}

fn part1(rules: HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    let x = find_all_parent_bags(&rules, "shiny gold");
    x.len() as u32
}

fn find_all_parent_bags(
    rules: &HashMap<&str, Vec<(u32, &str)>>,
    bag_to_find: &str,
) -> HashSet<String> {
    let mut parents = find_parent_bags(rules, bag_to_find);
    if parents.len() > 0 {
        for parent in parents.clone() {
            let mut grand_parent = find_all_parent_bags(rules, &parent);
            parents = parents
                .union(&mut grand_parent)
                .map(|c| c.clone())
                .collect();
        }
    }
    parents
}

fn find_parent_bags(rules: &HashMap<&str, Vec<(u32, &str)>>, bag_to_find: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    for (parent_bag, content) in rules {
        for (_, bag) in content {
            if *bag == bag_to_find {
                result.insert(String::from(*parent_bag));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
