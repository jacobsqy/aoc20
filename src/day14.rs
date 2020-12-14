use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U64(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U64(part2(&puzzle.input))),
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

fn part2(input: &str) -> u64 {
    // 36-bits
    let mut bitmask: Vec<char> = Vec::new();
    // Address -> Value
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input.lines() {
        let split: Vec<&str> = line
            .split(|c| c == ' ' || c == '[' || c == ']' || c == '=')
            .filter(|s| s.len() > 0)
            .collect();
        if split[0] == "mem" {
            let adress: Vec<char> = format!("{:036b}", split[1].parse::<u64>().unwrap())
                .chars()
                .collect();
            let value: u64 = split[2].parse::<u64>().unwrap();

            for a in find_all_adresses(
                adress
                    .iter()
                    .zip(&bitmask)
                    .map(|(x, y)| {
                        y.to_digit(2)
                            .map_or(*y, |opty| digit_to_char(opty | x.to_digit(2).unwrap()))
                    })
                    .collect(),
            )
            .iter()
            .map(|a| u64::from_str_radix(&a.into_iter().collect::<String>(), 2).unwrap())
            {
                memory.insert(a, value);
            }
        } else if split[0] == "mask" {
            bitmask = split[1].chars().collect();
        }
    }

    memory.values().sum()
}

fn find_all_adresses(input: Vec<char>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let x_indeces: Vec<usize> = input
        .iter()
        .enumerate()
        .filter(|(_i, x)| **x == 'X')
        .map(|(i, _)| i)
        .collect();

    for x in 0..2_u32.pow(x_indeces.len() as u32) {
        let mut adress: Vec<char> = input.clone();
        for (i, x_bit) in format!("{:036b}", x)
            .chars()
            .skip(36 - x_indeces.len())
            .enumerate()
        {
            adress[x_indeces[i]] = x_bit;
        }
        result.push(adress);
    }
    result
}

fn digit_to_char(x: u32) -> char {
    match x {
        0 => '0',
        1 => '1',
        x => panic!("Can't parse digit to binary char! {}", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("14", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("14", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
