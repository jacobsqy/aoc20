use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
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
                if *seat == 'L' && occupied_adjacent_seats(i as i32, j as i32, &seats) == 0 {
                    new_seats[i][j] = '#';
                    changed = true;
                } else if *seat == '#' && occupied_adjacent_seats(i as i32, j as i32, &seats) > 3 {
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

fn part2(input: &str) -> u32 {
    let mut seats: Vec<Vec<char>> = parse(input);
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_seats: Vec<Vec<char>> = seats.clone();

        for i in 0..seats.len() {
            for (j, seat) in seats[i].iter().enumerate() {
                if *seat == 'L' && occupied_visible_seats(i as i32, j as i32, &seats) == 0 {
                    new_seats[i][j] = '#';
                    changed = true;
                } else if *seat == '#' && occupied_visible_seats(i as i32, j as i32, &seats) > 4 {
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

fn occupied_visible_seats(x: i32, y: i32, seats: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    let plus = &|a, i| a + i;
    let minus = &|a, i| a - i;
    let same = &|a, _i| a;
    // North-west
    if seat_is_visible(x, y, minus, minus, seats) {
        result += 1;
    }
    // North
    if seat_is_visible(x, y, same, minus, seats) {
        result += 1;
    }
    // North-east
    if seat_is_visible(x, y, plus, minus, seats) {
        result += 1;
    }
    // East
    if seat_is_visible(x, y, minus, same, seats) {
        result += 1;
    }
    // West
    if seat_is_visible(x, y, plus, same, seats) {
        result += 1;
    }
    // South-west
    if seat_is_visible(x, y, minus, plus, seats) {
        result += 1;
    }
    // South
    if seat_is_visible(x, y, same, plus, seats) {
        result += 1;
    }
    // South-east
    if seat_is_visible(x, y, plus, plus, seats) {
        result += 1;
    }
    result
}

fn seat_is_visible(
    x: i32,
    y: i32,
    change_x: &dyn Fn(i32, i32) -> i32,
    change_y: &dyn Fn(i32, i32) -> i32,
    seats: &Vec<Vec<char>>,
) -> bool {
    let mut i = 1;
    loop {
        match seat_is_occupied(change_x(x, i), change_y(y, i), seats) {
            // The seat is occupied
            Some('#') => return true,
            // The seat is floor
            Some('.') => i += 1,
            // The border has been reached or the seat is empty
            _ => return false,
        }
    }
}

fn occupied_adjacent_seats(x: i32, y: i32, seats: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    if seat_is_occupied(x - 1, y - 1, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x, y - 1, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x + 1, y - 1, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x - 1, y, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x + 1, y, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x - 1, y + 1, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x, y + 1, &seats) == Some('#') {
        result += 1;
    }
    if seat_is_occupied(x + 1, y + 1, &seats) == Some('#') {
        result += 1;
    }
    result
}

fn seat_is_occupied(x: i32, y: i32, seats: &Vec<Vec<char>>) -> Option<char> {
    if x >= 0 && y >= 0 {
        match *seats.get(x as usize)?.get(y as usize)? {
            '#' => return Some('#'),
            'L' => return Some('L'),
            _ => return Some('.'),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
