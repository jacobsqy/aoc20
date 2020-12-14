use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U64(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn part1(input: &str) -> u64 {
    // 36-bits
    let mut bitmask: Vec<(usize, char)> = Vec::new();
    // Address -> Value
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input.lines() {
        let split: Vec<&str> = line
            .split(|c| c == ' ' || c == '[' || c == ']' || c == '=')
            .filter(|s| s.len() > 0)
            .map(|s| s.trim())
            .collect();
        if split[0] == "mem" {
            let adress: u64 = split[1].parse().unwrap();
            let mut value: Vec<char> = format!("{:036b}", split[2].parse::<u64>().unwrap())
                .chars()
                .collect();
            for (i, bit) in &bitmask {
                value[*i] = *bit;
            }
            let value: String = value.into_iter().collect();
            memory.insert(adress, u64::from_str_radix(&value, 2).unwrap());
        } else if split[0] == "mask" {
            bitmask = split[1]
                .chars()
                .enumerate()
                .filter(|(_i, c)| c.is_ascii_digit())
                .collect();
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
