use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn part1(input: &str) -> u32 {
    let mut active_cubes: Vec<(i32, i32, i32)> = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.push((x as i32, y as i32, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut new_active_cubes: Vec<(i32, i32, i32)> = Vec::new();
        let mut neighbours: HashMap<(i32, i32, i32), u32> = HashMap::new();
        for cube in &active_cubes {
            for neighbour in find_neighbours(cube) {
                neighbours
                    .entry(neighbour)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }
        for (cube, n) in neighbours {
            if active_cubes.contains(&cube) && (n == 2 || n == 3) {
                new_active_cubes.push(cube);
            } else if n == 3 {
                new_active_cubes.push(cube);
            }
        }
        active_cubes = new_active_cubes;
    }

    active_cubes.len() as u32
}

fn find_neighbours((x, y, z): &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    for x_alt in x - 1..=x + 1 {
        for y_alt in y - 1..=y + 1 {
            for z_alt in z - 1..=z + 1 {
                if x_alt != *x || y_alt != *y || z_alt != *z {
                    result.push((x_alt, y_alt, z_alt));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
