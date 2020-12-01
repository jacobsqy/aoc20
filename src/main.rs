#![feature(test)]

extern crate test;

mod day1;

use std::env;
use std::fs;
use std::num::ParseIntError;

pub enum Part {
    One,
    Two,
}

impl From<&str> for Part {
    fn from(string: &str) -> Self {
        match string.parse() {
            Ok(1) => Part::One,
            Ok(2) => Part::Two,
            _ => panic!("Could not parse part!"),
        }
    }
}

pub enum RunError {
    ParseError(ParseIntError),
    NoResult,
}

impl From<ParseIntError> for RunError {
    fn from(error: ParseIntError) -> Self {
        RunError::ParseError(error)
    }
}

pub struct Puzzle<T> {
    day: i32,
    part: Part,
    input: T,
}

impl Puzzle<Vec<i32>> {
    fn new(day: &str, part: &str, filename: &str) -> Puzzle<Vec<i32>> {
        let day: i32 = day.parse().unwrap();
        let part: Part = Part::from(part);
        let input: Vec<i32> = fs::read_to_string(filename).unwrap().lines()
            .map(|x| x.parse::<i32>().unwrap()).collect();

        Puzzle {day, part, input}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let filename = &args[3];

    let puzzle: Puzzle<Vec<i32>> = Puzzle::new(day, part, filename);

    let e = match puzzle.day {
        1 => day1::run(&puzzle),
        _ => Err(RunError::NoResult)
    };
    match e {
        Ok(output) => println!("Answer: {}", output),
        Err(RunError::NoResult) => println!("No answer found!"),
        Err(RunError::ParseError(e)) =>
            println!("There was a problem parsing the input\n{:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_day1_part1(b: &mut Bencher) {
        let input: Vec<i32> = fs::read_to_string("input").unwrap().lines()
            .map(|x| x.parse::<i32>().unwrap()).collect();
        b.iter(|| day1::part1(&input));
    }

    #[bench]
    fn bench_day1_part2(b: &mut Bencher) {
        let input: Vec<i32> = fs::read_to_string("input").unwrap().lines()
            .map(|x| x.parse::<i32>().unwrap()).collect();
        b.iter(|| day1::part2(&input));
    }
}
