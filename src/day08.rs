use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => part1(&puzzle.input.lines().collect()),
        Part::Two => part2(&puzzle.input.lines().collect()),
        _ => Err(RunError::NoResult),
    }
}

fn part1(lines: &Vec<&str>) -> Result<RunResult, RunError> {
    let mut visited_values: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut i = 0;
    while i < lines.len() {
        if !visited_values.insert(i) {
            // Return the accumulator if the value was already in the HashSet
            return Err(RunError::NoResult);
        }

        match lines[i].split_once(' ') {
            Some(("acc", x)) => {
                accumulator += x.parse::<i32>().unwrap();
                i += 1;
            }
            Some(("jmp", x)) => {
                i = ((i as i32) + x.parse::<i32>().unwrap()) as usize;
            }
            Some(("nop", _)) => i += 1,
            _ => (),
        }
    }
    //Err(RunError::NoResult)
    Ok(RunResult::I32(accumulator))
}

fn part2(lines: &Vec<&str>) -> Result<RunResult, RunError> {
    for i in 0..lines.len() {
        if lines[i].starts_with("jmp") {
            let new_line = lines[i].replace("jmp", "nop");
            match part1(&[&lines[0..i], &[&new_line], &lines[i + 1..]].concat()) {
                Err(_) => (),
                r => return r,
            }
        } else if lines[i].starts_with("nop") {
            let new_line = lines[i].replace("nop", "jmp");
            match part1(&[&lines[0..i], &[&new_line], &lines[i + 1..]].concat()) {
                Err(_) => (),
                r => return r,
            }
        }
    }
    Err(RunError::NoResult)
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
