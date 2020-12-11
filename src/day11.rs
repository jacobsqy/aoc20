use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        result.push(line.chars().collect())
    }
    result
}

fn part1(input: &str) -> u32 {
    let mut seats: Vec<Vec<char>> = parse(input);
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_seats: Vec<Vec<char>> = seats.clone();

        for i in 0..seats.len() {
            for (j, seat) in seats[i].iter().enumerate() {
                if *seat == 'L' && occupied_adjacent_seats(i, j, &seats) == 0 {
                    new_seats[i][j] = '#';
                    changed = true;
                } else if *seat == '#' && occupied_adjacent_seats(i, j, &seats) > 3 {
                    new_seats[i][j] = 'L';
                    changed = true;
                }
            }
        }
        seats = new_seats;
    }

    let mut occupied_seats = 0;
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            if seats[i][j] == '#' {
                occupied_seats += 1;
            }
        }
    }
    occupied_seats
}

fn occupied_adjacent_seats(x: usize, y: usize, seats: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    if x > 0 && y > 0 && seats.get(x - 1).map(|x| x.get(y - 1)) == Some(Some(&'#')) {
        result += 1;
    }
    if y > 0 && seats.get(x).map(|x| x.get(y - 1)) == Some(Some(&'#')) {
        result += 1;
    }
    if y > 0 && seats.get(x + 1).map(|x| x.get(y - 1)) == Some(Some(&'#')) {
        result += 1;
    }
    if x > 0 && seats.get(x - 1).map(|x| x.get(y)) == Some(Some(&'#')) {
        result += 1;
    }
    if seats.get(x + 1).map(|x| x.get(y)) == Some(Some(&'#')) {
        result += 1;
    }
    if x > 0 && seats.get(x - 1).map(|x| x.get(y + 1)) == Some(Some(&'#')) {
        result += 1;
    }
    if seats.get(x).map(|x| x.get(y + 1)) == Some(Some(&'#')) {
        result += 1;
    }
    if seats.get(x + 1).map(|x| x.get(y + 1)) == Some(Some(&'#')) {
        result += 1;
    }
    result
}

fn seat_is_occupied(x: usize, y: usize, seats: &Vec<Vec<char>>) -> Option<bool> {
    if x > 0 && y > 0 && *seats.get(x)?.get(y)? == '#' {
        return Some(true);
    }
    Some(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
