mod day1;
mod day2;
mod day3;
mod day4;

use reqwest::header::COOKIE;
use std::env;
use std::fmt;
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
        let client = reqwest::Client::new();
        match client
            .post(&format!("https://adventofcode.com/2020/day/{}/input", day))
            .header(
                COOKIE,
                format!(
                    "session={}",
                    fs::read_to_string("secret.cookie")
                        .unwrap()
                        .lines()
                        .next()
                        .unwrap()
                ),
            )
            .send()
        {
            Ok(mut response) => Some(Puzzle {
                day: day.parse().unwrap(),
                part: Part::from(part),
                input: response.text().unwrap(),
            }),
            _ => match fs::read_to_string(filename) {
                Ok(content) => Some(Puzzle {
                    day: day.parse().unwrap(),
                    part: Part::from(part),
                    input: content,
                }),
                Err(_) => None,
            },
        }
    }
}

pub enum RunResult {
    I32(i32),
    U32(u32),
    VECI32(Vec<i32>),
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunResult::I32(x) => write!(f, "{}", x),
            RunResult::U32(x) => write!(f, "{}", x),
            RunResult::VECI32(xs) => {
                let tmp: Vec<String> = xs.iter().map(|x| x.to_string()).collect();
                write!(f, "[{}]", tmp.join(", "))
            }
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
            Puzzle { day: 4, .. } => day4::run(&p),
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
