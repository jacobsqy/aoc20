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
    input.lines().map(|x| x.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let mut seats: Vec<Vec<char>> = parse(input);
    let mut new_seats: Vec<(usize, usize, char)> = vec![(0, 0, '0')];
    while new_seats.len() > 0 {
        new_seats.clear();

        for i in 0..seats.len() {
            for (j, seat) in seats[i].iter().enumerate() {
                if *seat == 'L' && occupied_adjacent_seats(i as i32, j as i32, &seats) == 0 {
                    new_seats.push((i, j, '#'));
                } else if *seat == '#' && occupied_adjacent_seats(i as i32, j as i32, &seats) > 3 {
                    new_seats.push((i, j, 'L'));
                }
            }
        }

        for (x, y, v) in &new_seats {
            seats[*x][*y] = *v;
        }
    }

    seats.concat().iter().filter(|c| **c == '#').count() as u32
}

fn part2(input: &str) -> u32 {
    let mut seats: Vec<Vec<char>> = parse(input);
    let mut new_seats: Vec<(usize, usize, char)> = vec![(0, 0, '0')];
    while new_seats.len() > 0 {
        new_seats.clear();

        for i in 0..seats.len() {
            for (j, seat) in seats[i].iter().enumerate() {
                if *seat == 'L' && occupied_visible_seats(i as i32, j as i32, &seats) == 0 {
                    new_seats.push((i, j, '#'));
                } else if *seat == '#' && occupied_visible_seats(i as i32, j as i32, &seats) > 4 {
                    new_seats.push((i, j, 'L'));
                }
            }
        }

        for (x, y, v) in &new_seats {
            seats[*x][*y] = *v;
        }
    }

    seats.concat().iter().filter(|c| **c == '#').count() as u32
}

fn occupied_visible_seats(x: i32, y: i32, seats: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for i in -1..=1 {
        let change_x = &|a, index| a + i * index;
        for j in -1..=1 {
            let change_y = &|a, index| a + j * index;
            if !(i == 0 && j == 0) {
                if seat_is_visible(x, y, change_x, change_y, seats) {
                    result += 1;
                }
            }
        }
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
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                if seat_is_occupied(x + i, y + j, &seats) == Some('#') {
                    result += 1;
                }
            }
        }
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

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("11", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("11", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
