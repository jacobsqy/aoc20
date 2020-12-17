use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U64(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U64(part2(&puzzle.input))),
    }
}

// returns a triplet of (rules, ticket, nearby tickets)
//fn parse(
//input: &str,
//) -> (
//HashMap<&str, ((u64, u64), (u64, u64))>,
//Vec<u64>,
//Vec<Vec<u64>>,
//) {
//let mut input = input.split("\n\n");
//let ranges: HashMap<&str, ((u64, u64), (u64, u64))> = HashMap::new();
//}

fn part1(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let mut ranges: HashMap<&str, ((u64, u64), (u64, u64))> = HashMap::new();
    for line in input.next().unwrap().lines() {
        let (name, rs) = line.split_once(": ").unwrap();
        let (r1, r2) = rs.split_once(" or ").unwrap();
        let r1: (&str, &str) = r1.split_once('-').unwrap();
        let r1: (u64, u64) = (r1.0.parse::<u64>().unwrap(), r1.1.parse::<u64>().unwrap());
        let r2: (&str, &str) = r2.split_once('-').unwrap();
        let r2: (u64, u64) = (r2.0.parse::<u64>().unwrap(), r2.1.parse::<u64>().unwrap());
        ranges.insert(name, (r1, r2));
    }

    let mut result = 0;
    for line in input.skip(1).next().unwrap().lines().skip(1) {
        for number in line.split(',') {
            let digit = number.parse::<u64>().unwrap();
            if !valid_field(digit, &ranges) {
                result += digit;
            }
        }
    }
    result
}

fn part2(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let mut ranges: HashMap<&str, ((u64, u64), (u64, u64))> = HashMap::new();
    for line in input.next().unwrap().lines() {
        let (name, rs) = line.split_once(": ").unwrap();
        let (r1, r2) = rs.split_once(" or ").unwrap();
        let r1: (&str, &str) = r1.split_once('-').unwrap();
        let r1: (u64, u64) = (r1.0.parse::<u64>().unwrap(), r1.1.parse::<u64>().unwrap());
        let r2: (&str, &str) = r2.split_once('-').unwrap();
        let r2: (u64, u64) = (r2.0.parse::<u64>().unwrap(), r2.1.parse::<u64>().unwrap());
        ranges.insert(name, (r1, r2));
    }

    // parse your ticket
    let your_ticket: Vec<u64> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let valid_tickets: Vec<Vec<u64>> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter(|l| {
            l.split(',')
                .map(|n| valid_field(n.parse::<u64>().unwrap(), &ranges))
                .all(|x| x)
        })
        .map(|l| l.split(',').map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();

    // Field-index -> Range-name
    let mut valid_fields: HashMap<usize, HashSet<&str>> = HashMap::new();
    for field_index in 0..valid_tickets[0].len() {
        for field in valid_tickets.iter().map(|x| x[field_index]) {
            let new_fields = find_valid_fields(field, &ranges);
            valid_fields
                .entry(field_index)
                .and_modify(|x| *x = x.intersection(&new_fields).copied().collect())
                .or_insert(new_fields);
        }
    }

    let mut result: HashMap<usize, &str> = HashMap::new();
    let mut used_fields: HashSet<&str> = HashSet::new();
    while used_fields.len() != valid_fields.len() {
        for (i, fields) in &valid_fields {
            let diff: Vec<&str> = fields.difference(&used_fields).copied().collect();
            if diff.len() == 1 {
                result.insert(*i, diff[0]);
                used_fields.insert(diff[0]);
            }
        }
    }
    result
        .iter()
        .filter(|(_k, v)| v.starts_with("departure"))
        .map(|(k, _v)| your_ticket[*k])
        .product()
}

fn find_valid_fields<'a>(
    field: u64,
    ranges: &'a HashMap<&str, ((u64, u64), (u64, u64))>,
) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    for (k, (r1, r2)) in ranges {
        if in_range(r1, field) || in_range(r2, field) {
            result.insert(k);
        }
    }
    result
}

fn valid_field(field: u64, ranges: &HashMap<&str, ((u64, u64), (u64, u64))>) -> bool {
    for (r1, r2) in ranges.values() {
        if in_range(r1, field) || in_range(r2, field) {
            return true;
        }
    }
    false
}

fn in_range((lower, higher): &(u64, u64), x: u64) -> bool {
    *lower <= x && x <= *higher
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
