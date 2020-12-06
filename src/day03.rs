use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    let map: Vec<Vec<char>> = parse(&puzzle.input);
    match puzzle.part {
        Part::One => Ok(part1(&map)),
        Part::Two => Ok(part2(&map)),
    }
    //Err(RunError::NoResult)
}

fn parse(string: &str) -> Vec<Vec<char>> {
    string.lines().map(|line| line.chars().collect()).collect()
}

fn part1(map: &Vec<Vec<char>>) -> RunResult {
    RunResult::U32(solve(map, 3, 1))
}

fn part2(map: &Vec<Vec<char>>) -> RunResult {
    RunResult::U32(
        solve(map, 3, 1)
            * solve(map, 1, 1)
            * solve(map, 5, 1)
            * solve(map, 7, 1)
            * solve(map, 1, 2),
    )
}

fn solve(map: &Vec<Vec<char>>, x_slope: usize, y_slope: usize) -> u32 {
    let mut trees: u32 = 0;
    let mut column: usize = 0;
    let width: usize = map[0].len();
    for row in 0..map.len() {
        if row % y_slope == 0 {
            if map[row][column] == '#' {
                trees += 1;
            }
            column = (column + x_slope) % width;
        }
    }
    trees
}
