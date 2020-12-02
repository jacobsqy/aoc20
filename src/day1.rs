use crate::Puzzle;
use crate::RunError;

pub fn run(puzzle: &crate::Puzzle) -> Result<i32, RunError> {
    match puzzle {
        Puzzle {
            part: crate::Part::One,
            ..
        } => part1(&parse_input(puzzle)),
        Puzzle {
            part: crate::Part::Two,
            ..
        } => part2(&parse_input(puzzle)),
    }
}

fn parse_input(puzzle: &crate::Puzzle) -> Vec<i32> {
    puzzle
        .input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> Result<i32, RunError> {
    let input_length = input.len();
    for i in 0..input_length {
        let num1 = input[i];
        for j in i..input_length {
            let num2 = input[j];
            if num1 + num2 == 2020 {
                return Ok(num1 * num2);
            }
        }
    }
    Err(RunError::NoResult)
}

fn part2(input: &Vec<i32>) -> Result<i32, RunError> {
    let input_length = input.len();
    for i in 0..input_length {
        let num1 = input[i];
        for j in i..input_length {
            let num2 = input[j];
            for k in j..input_length {
                let num3 = input[k];
                if num1 + num2 + num3 == 2020 {
                    return Ok(num1 * num2 * num3);
                }
            }
        }
    }
    Err(RunError::NoResult)
}
