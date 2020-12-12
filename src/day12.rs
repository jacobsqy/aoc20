use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::I32(part1(&puzzle.input))),
        //Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

struct Ship {
    ew_pos: i32,
    ns_pos: i32,
    direction: i32,
}

impl Ship {
    fn forward(&mut self, x: i32) {
        match self.direction {
            // North
            0 => self.ns_pos += x,
            // East
            1 => self.ew_pos += x,
            // South
            2 => self.ns_pos -= x,
            // West
            3 => self.ew_pos -= x,
            x => panic!("Invalid direction! {}", x),
        }
    }

    fn left(&mut self, x: i32) {
        println!("{}", x);
        self.direction = (((self.direction - (x / 90)) % 4) + 4) % 4;
    }

    fn right(&mut self, x: i32) {
        println!("{}", x);
        self.direction = (((self.direction + (x / 90)) % 4) + 4) % 4;
    }
}

fn part1(input: &str) -> i32 {
    let mut ship = Ship {
        ew_pos: 0,
        ns_pos: 0,
        direction: 1,
    };

    for line in input.lines() {
        match line.split_at(1) {
            ("N", x) => ship.ns_pos += x.parse::<i32>().unwrap(),
            ("S", x) => ship.ns_pos -= x.parse::<i32>().unwrap(),
            ("E", x) => ship.ew_pos += x.parse::<i32>().unwrap(),
            ("W", x) => ship.ew_pos -= x.parse::<i32>().unwrap(),
            ("L", x) => ship.left(x.parse().unwrap()),
            ("R", x) => ship.right(x.parse().unwrap()),
            ("F", x) => ship.forward(x.parse().unwrap()),
            _ => panic!("Could not parse line! {}", line),
        }
        println!("NS: {}, EW: {}", ship.ns_pos, ship.ew_pos);
    }

    ship.ns_pos.abs() + ship.ew_pos.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
