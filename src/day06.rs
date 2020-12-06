use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
    }
}

fn part1(string: &str) -> u32 {
    let mut result = 0;
    let mut questions: HashSet<char> = HashSet::new();
    for group in string.split("\n\n") {
        for c in group.chars() {
            if c != '\n' {
                questions.insert(c);
            }
        }
        result += questions.len();
        questions.drain();
    }
    result as u32
}

fn part2(string: &str) -> u32 {
    let mut result = 0;
    let mut qna: HashMap<char, usize> = HashMap::new();
    for group in string.split("\n\n") {
        let mut group_members = 0;
        for member in group.lines() {
            group_members += 1;
            for c in member.chars() {
                qna.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        for answers in qna.values() {
            if *answers == group_members {
                result += 1;
            }
        }
        qna.drain();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let puzzle: Puzzle = Puzzle::new("6", "1", None).unwrap();
        assert_eq!(part1(&puzzle.input), 6382);
    }

    #[test]
    fn test_part2() {
        let puzzle: Puzzle = Puzzle::new("6", "2", None).unwrap();
        assert_eq!(part2(&puzzle.input), 3197);
    }

    #[bench]
    fn bench_day6_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("6", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_day6_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("6", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
