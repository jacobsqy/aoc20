use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U64(part2(&puzzle.input))),
    }
}

fn part1(string: &str) -> u32 {
    let mut lines: Vec<u32> = string.lines().map(|x| x.parse().unwrap()).collect();
    lines.sort();
    let mut ones: u32 = 0;
    let mut threes: u32 = 0;
    let mut prev_line = 0;
    for line in lines {
        if line - prev_line == 3 {
            threes += 1;
        } else if line - prev_line == 1 {
            ones += 1;
        }

        /*
         *occurences
         *    .entry(line - prev_line)
         *    .and_modify(|x| *x = *x + 1)
         *    .or_insert(1);
         */
        prev_line = line;
    }
    ones * threes
}

fn part2(string: &str) -> u64 {
    let mut lines: Vec<u64> = string.lines().map(|x| x.parse().unwrap()).collect();
    lines.sort();
    lines.insert(0, 0);
    let mut alternatives: Vec<u64> = vec![0; lines.len()];
    alternatives[0] = 1;

    for i in 0..lines.len() {
        if lines.get(i + 3).map_or(false, |x| x - lines[i] == 3) {
            // Three alternative paths
            inc_alts(&mut alternatives, i, 3);
        } else if lines.get(i + 2).map_or(false, |x| x - lines[i] <= 3) {
            // Two alternative paths
            inc_alts(&mut alternatives, i, 2);
        } else {
            // One alternative paths
            inc_alts(&mut alternatives, i, 1);
        }
    }

    *alternatives.last().unwrap()
}

fn inc_alts(alts: &mut Vec<u64>, i: usize, local_alts: usize) {
    let alt = alts[i];
    for j in 1..=local_alts {
        match alts.get_mut(i + j) {
            Some(x) => *x += alt,
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("10", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("10", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
