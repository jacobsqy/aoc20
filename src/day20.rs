use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U64(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

fn part1(input: &str) -> u64 {
    println!("{:?}", parse(input));
    0
}

fn parse(input: &str) -> HashMap<u64, Vec<Vec<bool>>> {
    let mut tiles: HashMap<u64, Vec<Vec<bool>>> = HashMap::new();
    for tile in input.split("\n\n").filter(|t| t.len() > 0) {
        let mut tile_lines = tile.lines();
        let tile_title = tile_lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap();
        let tile_title = tile_title[..tile_title.len() - 1].parse().unwrap();
        tiles.insert(
            tile_title,
            tile_lines
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        );
    }
    tiles
}

fn horizontal_match(left: &Vec<Vec<bool>>, right: &Vec<Vec<bool>>) -> bool {
    left.last() == right.first()
}

fn vertical_match(top: &Vec<Vec<bool>>, bottom: &Vec<Vec<bool>>) -> bool {
    top.iter()
        .map(|r| r.last())
        .eq(bottom.iter().map(|r| r.first()))
}

fn horizontal_flip(tile: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    tile.iter().rev().map(|x| x.clone()).collect()
}

fn vertical_flip(tile: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    tile.iter()
        .map(|x| x.iter().rev().copied().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_horizontal_match() {
        let p = parse(
            &Puzzle::new("20", "1", Some(&String::from("test20")))
                .unwrap()
                .input,
        );
        let left = p.get(&1951).unwrap();
        let left_flipped = &horizontal_flip(p.get(&1951).unwrap().to_vec());

        let right = p.get(&2311).unwrap();
        let right_flipped = &horizontal_flip(p.get(&2311).unwrap().to_vec());

        let right_right = p.get(&3079).unwrap();

        println!("{:?}\n\n{:?}", left, right);

        assert!(vertical_match(left, right));
        assert!(vertical_match(left, right));
        //assert!(horizontal_match(left, right));
        //assert!(!horizontal_match(left, right_right));
        //assert!(horizontal_match(right, right_right));
    }
}
