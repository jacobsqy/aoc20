use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        //Part::One => Ok(RunResult::I32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::I32(part2(&puzzle.input))),
        _ => Err(RunError::NoResult),
    }
}

struct Ship {
    ew_pos: i32,
    ns_pos: i32,
    waypoint: Waypoint,
}

impl Ship {
    fn forward(&mut self, x: i32) {
        self.ew_pos += self.waypoint.ew_pos * x;
        self.ns_pos += self.waypoint.ns_pos * x;
    }
}

struct Waypoint {
    ew_pos: i32,
    ns_pos: i32,
}

impl Waypoint {
    fn right(&mut self, x: i32) {
        for _ in 0..x / 90 {
            self.right_90();
        }
    }

    fn left(&mut self, x: i32) {
        for _ in 0..x / 90 {
            self.left_90();
        }
    }

    fn left_90(&mut self) {
        let new_ew_pos;
        let new_ns_pos;
        if self.ew_pos > 0 && self.ns_pos > 0 {
            // First quadrant
            new_ew_pos = -self.ns_pos;
            new_ns_pos = self.ew_pos;
        } else if self.ew_pos > 0 {
            // Second quadrant
            new_ew_pos = -self.ns_pos;
            new_ns_pos = self.ew_pos;
        } else if self.ns_pos < 0 {
            // Third quadrant
            new_ew_pos = -self.ns_pos;
            new_ns_pos = self.ew_pos;
        } else {
            // Fourth quadrant
            new_ew_pos = -self.ns_pos;
            new_ns_pos = self.ew_pos;
        }
        self.ns_pos = new_ns_pos;
        self.ew_pos = new_ew_pos;
    }

    fn right_90(&mut self) {
        let new_ew_pos;
        let new_ns_pos;
        if self.ew_pos > 0 && self.ns_pos > 0 {
            // First quadrant
            new_ew_pos = self.ns_pos;
            new_ns_pos = -self.ew_pos;
        } else if self.ew_pos > 0 {
            // Second quadrant
            new_ew_pos = self.ns_pos;
            new_ns_pos = -self.ew_pos;
        } else if self.ns_pos < 0 {
            // Third quadrant
            new_ew_pos = self.ns_pos;
            new_ns_pos = -self.ew_pos;
        } else {
            // Fourth quadrant
            new_ew_pos = self.ns_pos;
            new_ns_pos = -self.ew_pos;
        }
        self.ns_pos = new_ns_pos;
        self.ew_pos = new_ew_pos;
    }
}

fn part2(input: &str) -> i32 {
    let mut ship = Ship {
        ew_pos: 0,
        ns_pos: 0,
        waypoint: Waypoint {
            ew_pos: 10,
            ns_pos: 1,
        },
    };

    for line in input.lines() {
        match line.split_at(1) {
            ("N", x) => ship.waypoint.ns_pos += x.parse::<i32>().unwrap(),
            ("S", x) => ship.waypoint.ns_pos -= x.parse::<i32>().unwrap(),
            ("E", x) => ship.waypoint.ew_pos += x.parse::<i32>().unwrap(),
            ("W", x) => ship.waypoint.ew_pos -= x.parse::<i32>().unwrap(),
            ("L", x) => ship.waypoint.left(x.parse().unwrap()),
            ("R", x) => ship.waypoint.right(x.parse().unwrap()),
            ("F", x) => ship.forward(x.parse().unwrap()),
            _ => panic!("Could not parse line! {}", line),
        }
        println!("\n{}", line);
        println!("Ship coordinates: NS: {}, EW: {}", ship.ns_pos, ship.ew_pos);
        println!(
            "Waypoint coordinates: NS: {}, EW: {}",
            ship.waypoint.ns_pos, ship.waypoint.ew_pos
        );
    }

    ship.ns_pos.abs() + ship.ew_pos.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
