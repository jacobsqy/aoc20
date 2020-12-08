use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => part1(&puzzle.input.lines().collect()),
        Part::Two => part2(&puzzle.input.lines().collect()),
    }
}

fn part1(lines: &Vec<&str>) -> Result<RunResult, RunError> {
    let mut visited_values: HashSet<i32> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut i: i32 = 0;
    let last_instruction = lines.len() as i32;
    while i != last_instruction {
        if !visited_values.insert(i) {
            // Return the accumulator if the value was already in the HashSet
            return Err(RunError::NoResult);
        }

        match lines[i as usize].split_once(' ') {
            Some(("acc", x)) => {
                accumulator += x.parse::<i32>().unwrap();
                i += 1;
            }
            Some(("jmp", x)) => i += x.parse::<i32>().unwrap(),
            Some(("nop", _)) => i += 1,
            _ => (),
        }
    }
    //Err(RunError::NoResult)
    Ok(RunResult::I32(accumulator))
}

fn part2(lines: &Vec<&str>) -> Result<RunResult, RunError> {
    lines
        .iter()
        .enumerate()
        .filter(|(_i, l)| l.starts_with("jmp") || l.starts_with("nop"))
        .map(|(i, l)| {
            if l.starts_with("jmp") {
                part1(&[&lines[0..i], &[&l.replace("jmp", "nop")], &lines[i + 1..]].concat())
            } else {
                part1(&[&lines[0..i], &[&l.replace("nop", "jmp")], &lines[i + 1..]].concat())
            }
        })
        .filter(|r| r.is_ok())
        .next()
        .unwrap_or(Err(RunError::NoResult))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("8", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input.lines().collect()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("8", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input.lines().collect()));
    }
}
