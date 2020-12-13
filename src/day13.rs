use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U64(part2(&puzzle.input.lines().nth(1).unwrap()))),
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

fn part2(input: &str) -> u64 {
    let buses = input
        .split(',')
        .map(|x| x.parse::<i128>())
        .enumerate()
        .filter(|(_i, opt)| opt.is_ok())
        .map(|(i, x)| (i as i128, x.unwrap()));
    let (time, modulo) = buses
        .fold_first(|(i0, b0), (i1, b1)| find_mod(i0, i1, b0, b1))
        .unwrap();
    (modulo - time) as u64
}

fn find_mod(a0: i128, a1: i128, n0: i128, n1: i128) -> (i128, i128) {
    let (m1, m0) = find_beizout(n0, n1);
    let modulo = n0 * n1;
    let t = (((a0 * m1 * n1 + a1 * m0 * n0) % modulo) + modulo) % modulo;
    (t, modulo)
}

fn find_beizout(a: i128, b: i128) -> (i128, i128) {
    return find_beizout_helper(a, b, 0, 1, 1, 0);

    fn find_beizout_helper(
        r1: i128,
        r2: i128,
        s1: i128,
        s2: i128,
        t1: i128,
        t2: i128,
    ) -> (i128, i128) {
        if r1 == 0 {
            return (s2, t2);
        } else {
            let new_q = r2 / r1;
            let new_r = r2 % r1;
            let s0 = s2 - (new_q * s1);
            let t0 = t2 - (new_q * t1);
            return find_beizout_helper(new_r, r1, s0, s1, t0, t1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part2() {
        assert_eq!(part2("17,x,13,19"), 3417);
        assert_eq!(part2("67,7,59,61"), 754018);
        assert_eq!(part2("67,x,7,59,61"), 779210);
        assert_eq!(part2("67,7,x,59,61"), 1261476);
        assert_eq!(part2("1789,37,47,1889"), 1202161486);
        assert_eq!(part2("7,13,x,x,59,x,31,19"), 1068781);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("13", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("13", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input.lines().nth(1).unwrap()));
    }
}
