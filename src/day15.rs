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
    let input = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse::<u32>().unwrap());

    for line in input {
        values.insert(line, i);
        i += 1;
    }

    (i..n)
        .fold(None, |value, j| {
            values.insert(value.unwrap_or(0), j).map(|old_j| j - old_j)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part2() {
        let puzzle: Puzzle = Puzzle::new("15", "2", None).unwrap();
        assert_eq!(part2(&puzzle.input), 37385);
    }

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
