use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
    }
}

fn part1(input: &str) -> u32 {
    solve(input, 2020)
}

fn part2(input: &str) -> u32 {
    solve(input, 30000000)
}

fn solve(input: &str, n: u32) -> u32 {
    let mut values: HashMap<u32, u32> = HashMap::new();
    let mut i = 1;
    let mut prev_value = None;
    for line in input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse::<u32>().unwrap())
    {
        prev_value = values.insert(line, i).map(|old_i| i - old_i);
        i += 1;
    }

    for j in i..n {
        match prev_value {
            Some(x) => {
                prev_value = values.insert(x, j).map(|old_j| j - old_j);
            }
            None => {
                prev_value = values.insert(0, j).map(|old_j| j - old_j);
            }
        }
    }
    prev_value.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("15", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("15", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
