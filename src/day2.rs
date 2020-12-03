use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

struct Rule {
    least: usize,
    most: usize,
    character: char,
}

impl Rule {
    fn new(input: &str) -> Result<Self, RunError> {
        let input: Vec<&str> = input.split(|c| c == ' ' || c == '-').collect();
        let least: usize = input[0].parse()?;
        let most: usize = input[1].parse()?;
        let character: char = input[2]
            .chars()
            .next()
            .ok_or_else(|| RunError::ParseError)?;
        Ok(Rule {
            least,
            most,
            character,
        })
    }
}

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    let valid_password = match puzzle.part {
        Part::One => valid_password1,
        Part::Two => valid_password2,
    };
    let mut allowed_pwds = 0;
    for line in puzzle.input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let rule = Rule::new(line[0])?;
        let pwd = line[1];

        if valid_password(rule, pwd) {
            allowed_pwds += 1;
        }
    }
    Ok(RunResult::I32(allowed_pwds))
}

fn valid_password1(rule: Rule, pwd: &str) -> bool {
    let mut occurences = 0;

    for pwd_char in pwd.chars() {
        if pwd_char == rule.character {
            occurences += 1;
        }
    }
    rule.least <= occurences && occurences <= rule.most
}

fn valid_password2(rule: Rule, pwd: &str) -> bool {
    let chars: Vec<char> = pwd.chars().collect();

    let index_1_bool = match chars.get(rule.least) {
        Some(x) => x == &rule.character,
        None => false,
    };
    let index_2_bool = match chars.get(rule.most) {
        Some(x) => x == &rule.character,
        None => false,
    };

    index_1_bool ^ index_2_bool
}
