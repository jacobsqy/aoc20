use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    let parsed_input = parse_ids(&puzzle.input);
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&parsed_input))),
        Part::Two => Ok(RunResult::U32(part2(&parsed_input))),
        //_ => Err(RunError::NoResult),
    }
}

fn parse_ids(input: &str) -> Vec<u32> {
    // 2^7 * 8 + 2^3 = 1032
    let mut result: Vec<u32> = Vec::new();
    //let mut result = [0; 1032];
    for line in input.lines() {
        let (row, col) = line.split_at(7);
        let id = parse_binary(row) * 8 + parse_binary(col);
        result.push(id);
    }
    result.sort();
    result
}

fn parse_binary(string: &str) -> u32 {
    let mut result: String = String::from("");
    //string.replace('F', "0").replace('B', "1").replace('R', "1")
    for c in string.chars() {
        match c {
            'F' => result.push('0'),
            'B' => result.push('1'),
            'L' => result.push('0'),
            'R' => result.push('1'),
            _ => panic!("Can't parse string"),
        }
    }
    u32::from_str_radix(&result, 2).unwrap()
}

fn part1(ids: &Vec<u32>) -> u32 {
    ids[0]
}

fn part2(ids: &Vec<u32>) -> u32 {
    for i in 0..ids.len() {
        let next_id = ids[i] + 1;
        if next_id != ids[i + 1] {
            return next_id;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_ids("BFFFBBFRRR")), 567);
    }

    #[test]
    fn test_part2() {
        let puzzle: Puzzle = Puzzle::new("5", "2", None).unwrap();
        assert_eq!(part2(&parse_ids(&puzzle.input)), 592);
    }

    #[bench]
    fn bench_day5_parse(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("5", "1", None).unwrap();
        b.iter(|| parse_ids(&puzzle.input));
    }

    #[bench]
    fn bench_day5_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("5", "1", None).unwrap();
        let parsed_input = parse_ids(&puzzle.input);
        b.iter(|| part1(&parsed_input));
    }

    #[bench]
    fn bench_day5_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("5", "2", None).unwrap();
        let parsed_input = parse_ids(&puzzle.input);
        b.iter(|| part2(&parsed_input));
    }
}
