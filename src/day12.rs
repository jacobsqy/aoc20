use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        //Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
