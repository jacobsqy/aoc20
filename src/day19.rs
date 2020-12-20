use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use regex::Regex;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn part1(input: &str) -> u32 {
    let (rules_str, text) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, &str> = HashMap::new();
    for (n, rule) in rules_str.lines().map(|l| l.split_once(": ").unwrap()) {
        rules.insert(n.parse().unwrap(), rule);
    }
    let mut regex_vec = parse_regex(rules.get(&0).unwrap(), &rules);
    regex_vec.insert(0, '^');
    regex_vec.push('$');
    let regex_str: String = regex_vec.iter().collect();

    println!("{}", regex_str);
    let regex = Regex::new(&regex_str).unwrap();
    println!(
        "Matches: {:?}",
        text.lines()
            .filter(|l| regex.is_match(l))
            .collect::<Vec<&str>>()
    );
    text.lines().filter(|l| regex.is_match(l)).count() as u32
}

fn parse_regex(string: &str, lines: &HashMap<usize, &str>) -> Vec<char> {
    if string.contains('"') {
        let i = string.find('"').unwrap();
        vec![string.chars().nth(i + 1).unwrap()]
    } else if string.contains('|') {
        let i = string.find('|').unwrap();
        [
            vec!['('],
            parse_regex(&string[..i], lines),
            vec!['|'],
            parse_regex(&string[i + 1..], lines),
            vec![')'],
        ]
        .concat()
    } else {
        for x in string.split_ascii_whitespace() {
            println!("Parsing: {}", x);
        }
        string
            .split_ascii_whitespace()
            .map(|rule| parse_regex(lines.get(&rule.parse::<usize>().unwrap()).unwrap(), lines))
            .collect::<Vec<Vec<char>>>()
            .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
