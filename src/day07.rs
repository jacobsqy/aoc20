use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&parse1(&puzzle.input)))),
        Part::Two => Ok(RunResult::U32(part2(&parse2(&puzzle.input)))),
    }
}

fn parse1(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" contain ");
        let parent = split.next().unwrap().rsplit_once(' ').unwrap().0;
        let children = split.next().unwrap().split(", ");
        for child in children {
            let child = child.split_once(' ').unwrap();
            result
                .entry(child.1.rsplit_once(' ').unwrap().0)
                .and_modify(|v| v.push(parent))
                .or_insert(vec![parent]);
        }
    }
    result
}

fn parse2(input: &str) -> HashMap<&str, Vec<(&str, &str)>> {
    let mut result: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    for line in input.lines() {
        let mut num_bags = Vec::new();

        let mut split = line.split(" contain ");
        let bag = split.next().unwrap().rsplit_once(' ').unwrap().0;
        let content = split.next().unwrap().split(", ");
        for num_bag in content {
            let num_bag = num_bag.split_once(' ').unwrap();
            let num_bag = (num_bag.0, num_bag.1.rsplit_once(' ').unwrap().0);
            num_bags.push(num_bag);
        }
        result.insert(bag, num_bags);
    }
    result
}

fn part1(rules: &HashMap<&str, Vec<&str>>) -> u32 {
    part1_helper(rules, "shiny gold").len() as u32 - 1
}

fn part1_helper(rules: &HashMap<&str, Vec<&str>>, parent: &str) -> HashSet<String> {
    let mut ancestors = HashSet::new();
    ancestors.insert(String::from(parent));
    match rules.get(parent) {
        Some(parents) => {
            for parent in parents {
                for ancestor in part1_helper(rules, parent) {
                    ancestors.insert(ancestor);
                }
            }
        }
        None => (),
    }
    ancestors
}

fn part2(rules: &HashMap<&str, Vec<(&str, &str)>>) -> u32 {
    part2_helper(rules, "shiny gold").unwrap_or(1) - 1
}

fn part2_helper(rules: &HashMap<&str, Vec<(&str, &str)>>, parent: &str) -> Option<u32> {
    let mut result: u32 = 1;
    for child in rules.get(parent)? {
        match child.0.parse::<u32>() {
            Ok(n) => result += n * part2_helper(rules, child.1).unwrap_or(1),
            _ => (),
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_parse1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("7", "1", None).unwrap();
        b.iter(|| parse1(&puzzle.input));
    }

    #[bench]
    fn bench_parse2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("7", "2", None).unwrap();
        b.iter(|| parse2(&puzzle.input));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("7", "1", None).unwrap();
        let parsed_input = parse1(&puzzle.input);
        b.iter(|| part1(&parsed_input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("7", "2", None).unwrap();
        let parsed_input = parse2(&puzzle.input);
        b.iter(|| part2(&parsed_input));
    }
}
