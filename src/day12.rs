use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::I32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::I32(part2(&puzzle.input))),
    }
}

fn forward_ship(ship: (i32, i32, i32), x: i32) -> (i32, i32, i32) {
    let (mut ns, mut ew, dir) = ship;
    match dir {
        // North
        0 => ns += x,
        // East
        1 => ew += x,
        // South
        2 => ns -= x,
        // West
        3 => ew -= x,
        x => panic!("Invalid direction! {}", x),
    }
    (ns, ew, dir)
}

fn left_ship(dir: i32, x: i32) -> i32 {
    (((dir - (x / 90)) % 4) + 4) % 4
}

fn right_ship(dir: i32, x: i32) -> i32 {
    (((dir + (x / 90)) % 4) + 4) % 4
}

fn part1(input: &str) -> i32 {
    let mut ship = (0, 0, 1);

    for line in input.lines() {
        match line.split_at(1) {
            ("N", x) => ship.0 += x.parse::<i32>().unwrap(),
            ("S", x) => ship.0 -= x.parse::<i32>().unwrap(),
            ("E", x) => ship.1 += x.parse::<i32>().unwrap(),
            ("W", x) => ship.1 -= x.parse::<i32>().unwrap(),
            ("L", x) => ship.2 = left_ship(ship.2, x.parse().unwrap()),
            ("R", x) => ship.2 = right_ship(ship.2, x.parse().unwrap()),
            ("F", x) => ship = forward_ship(ship, x.parse().unwrap()),
            _ => panic!("Could not parse line! {}", line),
        }
    }

    ship.0.abs() + ship.1.abs()
}

fn forward_waypoint(waypoint: &(i32, i32), ship: &(i32, i32), x: i32) -> (i32, i32) {
    (ship.0 + (waypoint.0 * x), ship.1 + (waypoint.1 * x))
}

fn right_waypoint(waypoint: &(i32, i32), x: i32) -> (i32, i32) {
    let (mut ns, mut ew) = *waypoint;
    for _ in 0..x / 90 {
        let new_ew_pos = ns;
        let new_ns_pos = -ew;
        ns = new_ns_pos;
        ew = new_ew_pos;
    }
    (ns, ew)
}

fn left_waypoint(waypoint: &(i32, i32), x: i32) -> (i32, i32) {
    let (mut ns, mut ew) = *waypoint;
    for _ in 0..x / 90 {
        let new_ew_pos = -ns;
        let new_ns_pos = ew;
        ns = new_ns_pos;
        ew = new_ew_pos;
    }
    (ns, ew)
}

fn part2(input: &str) -> i32 {
    // (ns, ew)
    let mut ship = (0, 0);
    // (ns, ew)
    let mut waypoint = (1, 10);

    for line in input.lines() {
        match line.split_at(1) {
            ("N", x) => waypoint.0 += x.parse::<i32>().unwrap(),
            ("S", x) => waypoint.0 -= x.parse::<i32>().unwrap(),
            ("E", x) => waypoint.1 += x.parse::<i32>().unwrap(),
            ("W", x) => waypoint.1 -= x.parse::<i32>().unwrap(),
            ("L", x) => waypoint = left_waypoint(&waypoint, x.parse().unwrap()),
            ("R", x) => waypoint = right_waypoint(&waypoint, x.parse().unwrap()),
            ("F", x) => ship = forward_waypoint(&waypoint, &ship, x.parse().unwrap()),
            _ => panic!("Could not parse line! {}", line),
        }
    }

    ship.0.abs() + ship.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("12", "1", None).unwrap();
        b.iter(|| part1(&puzzle.input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let puzzle: Puzzle = Puzzle::new("12", "2", None).unwrap();
        b.iter(|| part2(&puzzle.input));
    }
}
