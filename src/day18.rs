use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U64(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U64(part2(&puzzle.input))),
    }
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.replace(" ", ""))
        .map(|line| eval2(0, &|x, y| x + y, &line))
        .sum()
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.replace(" ", ""))
        .map(|line| eval(0, &|x, y| x + y, &line))
        .sum()
}

fn eval2(value: u64, operator: &dyn Fn(u64, u64) -> u64, string: &str) -> u64 {
    match string.chars().next() {
        Some('(') => {
            let par_index = find_matching_paranthesis(string);
            let par_result = eval2(0, &|x, y| x + y, &string[1..par_index]);
            eval2(
                operator(value, par_result),
                operator,
                &string[par_index + 1..],
            )
        }
        Some('*') => value * eval2(0, &|x, y| x + y, &string[1..]),
        Some('+') => eval2(value, &|x, y| x + y, &string[1..]),
        Some(_) => {
            let op_index = string
                .find(|c: char| !c.is_digit(10))
                .unwrap_or(string.len());
            eval2(
                operator(value, string[0..op_index].parse().unwrap()),
                operator,
                &string[op_index..],
            )
        }
        None => value,
    }
}

fn eval(value: u64, operator: &dyn Fn(u64, u64) -> u64, string: &str) -> u64 {
    match string.chars().next() {
        Some('(') => {
            let par_index = find_matching_paranthesis(string);
            let par_result = eval(0, &|x, y| x + y, &string[1..par_index]);
            eval(
                operator(value, par_result),
                operator,
                &string[par_index + 1..],
            )
        }
        Some('*') => eval(value, &|x, y| x * y, &string[1..]),
        Some('+') => eval(value, &|x, y| x + y, &string[1..]),
        Some(_) => {
            let op_index = string
                .find(|c: char| !c.is_digit(10))
                .unwrap_or(string.len());
            eval(
                operator(value, string[0..op_index].parse().unwrap()),
                operator,
                &string[op_index..],
            )
        }
        None => value,
    }
}

fn find_matching_paranthesis(string: &str) -> usize {
    let mut count = 1;
    for (j, c) in string.chars().enumerate().skip(1) {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }
        if count == 0 {
            return j;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("2 * 3 + (4 * 5)\n5 + (8 * 3 + 9 + 3 * 4 * 3)\n5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 26 + 437 + 12240 + 13632);
    }
}
