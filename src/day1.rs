use crate::Puzzle;
use crate::RunError;

pub fn run(puzzle: &crate::Puzzle<Vec<i32>>) -> Result<i32, RunError> {
    match puzzle {
        Puzzle{part: crate::Part::One, input, ..} => part1(input),
        Puzzle{part: crate::Part::Two, input, ..} => part2(input),
    }
}

pub fn part1(input: &Vec<i32>) -> Result<i32, RunError> {
    let input_length = input.len();
    for i in 0..input_length {
        let num1 = input[i];
        for j in i..input_length {
            let num2 = input[j];
            if num1 + num2 == 2020 {
                return Ok(num1 * num2)
            }
        }
    }
    Err(RunError::NoResult)
}

pub fn part2(input: &Vec<i32>) -> Result<i32, RunError> {
    let input_length = input.len();
    for i in 0..input_length {
        let num1 = input[i];
        for j in i..input_length {
            let num2 = input[j];
            for k in j..input_length {
                let num3 = input[k];
                if num1 + num2 + num3 == 2020 {
                    return Ok(num1 * num2 * num3)
                }
            }
        }
    }
    Err(RunError::NoResult)
}
