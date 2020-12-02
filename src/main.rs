mod day1;
mod day2;
mod day3;

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
    ParseError,
    ArgError(String),
    NoResult,
}

impl From<ParseIntError> for RunError {
    fn from(_: ParseIntError) -> Self {
        RunError::ParseError
    }
}

pub struct Puzzle {
    day: i32,
    part: Part,
    input: String,
}

impl Puzzle {
    fn new(day: &str, part: &str, filename: &str) -> Option<Puzzle> {
        match fs::read_to_string(filename) {
            Ok(content) => Some(Puzzle {
                day: day.parse().unwrap(),
                part: Part::from(part),
                input: content,
            }),
            Err(_) => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let filename = &args[3];

    let puzzle: Option<Puzzle> = Puzzle::new(day, part, filename);

    let result = match puzzle {
        Some(p) => match p {
            Puzzle { day: 1, .. } => day1::run(&p),
            Puzzle { day: 2, .. } => day2::run(&p),
            Puzzle { day: 3, .. } => day3::run(&p),
            _ => Err(RunError::NoResult),
        },
        _ => Err(RunError::ArgError(String::from("Could not parse day"))),
    };

    match result {
        Ok(output) => println!("Answer: {}", output),
        Err(RunError::NoResult) => println!("No answer found!"),
        Err(RunError::ArgError(e)) => println!("Invalid argument\n{:?}", e),
        Err(RunError::ParseError) => println!("There was a problem parsing the input!"),
    }
}
