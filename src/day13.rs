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

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let time: u32 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>())
        .filter(|opt| opt.is_ok())
        .map(|x| x.unwrap())
        .collect();

    (time..)
        .map(|t| (t, buses.iter().filter(|b| t % **b == 0).next()))
        .filter(|(_t, b)| b.is_some())
        .map(|(t, b)| b.unwrap() * (t - time))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("13", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }
}
