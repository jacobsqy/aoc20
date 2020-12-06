use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        //Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError: NoResult),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
