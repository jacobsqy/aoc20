use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input) as u32)),
        Part::Two => Ok(RunResult::U32(
            part2(&puzzle.input, part1(&puzzle.input) as u64) as u32,
        )),
    }
}

fn part1(input: &str) -> u64 {
    let lines: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    for i in 25..lines.len() {
        match part1_helper(&lines, i) {
            Some(x) => return x,
            None => (),
        }
    }
    0
}

fn part1_helper(lines: &Vec<u64>, i: usize) -> Option<u64> {
    for j in i - 25..i {
        for k in j..i {
            if lines[j] + lines[k] == lines[i] {
                return None;
            }
        }
    }
    Some(lines[i])
}

fn part2(input: &str, target: u64) -> u64 {
    let lines: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut sequence: Vec<u64> = Vec::new();
    for i in 0..lines.len() {
        let mut sum = 0;
        for j in i..lines.len() {
            sum += lines[j];
            sequence.push(lines[j]);
            if sum > target {
                sequence.clear();
                break;
            } else if sum == target && sequence.len() > 1 {
                return sequence.iter().max().unwrap() + sequence.iter().min().unwrap();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("9", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("9", "2", None).unwrap();
        let target = part1(&puzzle.input) as u64;
        b.iter(|| part2(&puzzle.input, target));
    }
}
