use crate::Part;
use crate::Puzzle;
use crate::RunError;
use crate::RunResult;

pub fn run(puzzle: &Puzzle) -> Result<RunResult, RunError> {
    match puzzle.part {
        Part::One => Ok(RunResult::U32(part1(&puzzle.input))),
        Part::Two => Ok(RunResult::U32(part2(&puzzle.input))),
    }
}

fn part1(input: &str) -> u32 {
    let mut allowed_passports = 0;
    for passport in input.split("\n\n") {
        if passport.contains("byr")
            && passport.contains("iyr")
            && passport.contains("eyr")
            && passport.contains("hgt")
            && passport.contains("hcl")
            && passport.contains("ecl")
            && passport.contains("pid")
        {
            allowed_passports += 1;
        }
    }
    allowed_passports
}

fn part2(input: &str) -> u32 {
    let mut allowed_passports = 0;
    for passport in input.split("\n\n") {
        let mut valid_byr_bool = false;
        let mut valid_iyr_bool = false;
        let mut valid_eyr_bool = false;
        let mut valid_hgt_bool = false;
        let mut valid_hcl_bool = false;
        let mut valid_ecl_bool = false;
        let mut valid_pid_bool = false;

        let fields = passport.split(|c| c == '\n' || c == ' ');
        for field in fields {
            let key_value: Vec<&str> = field.split(':').collect();
            if key_value.len() != 2 {
                break;
            }
            let key = key_value[0];
            let value = key_value[1];
            match key {
                "byr" => valid_byr_bool = in_range(value, 1920, 2002),
                "iyr" => valid_iyr_bool = in_range(value, 2010, 2020),
                "eyr" => valid_eyr_bool = in_range(value, 2020, 2030),
                "hgt" => valid_hgt_bool = valid_hgt(value),
                "hcl" => valid_hcl_bool = valid_hcl(value),
                "ecl" => valid_ecl_bool = valid_ecl(value),
                "pid" => valid_pid_bool = valid_pid(value),
                _ => (),
            };
        }
        if valid_byr_bool
            && valid_iyr_bool
            && valid_eyr_bool
            && valid_hgt_bool
            && valid_hcl_bool
            && valid_ecl_bool
            && valid_pid_bool
        {
            allowed_passports += 1;
        }
    }
    allowed_passports
}

fn in_range(value: &str, lower: u32, upper: u32) -> bool {
    match value.parse::<u32>() {
        Ok(year) => lower <= year && year <= upper,
        Err(_) => false,
    }
}

/*
 * hgt (Height) - a number followed by either cm or in:
 * If cm, the number must be at least 150 and at most 193.
 * If in, the number must be at least 59 and at most 76.
 */
fn valid_hgt(value: &str) -> bool {
    let unit = if value.ends_with("cm") {
        "cm"
    } else if value.ends_with("in") {
        "in"
    } else {
        ""
    };
    let value: &str = value.trim_end_matches(unit);
    if unit == "cm" {
        in_range(value, 150, 193)
    } else if unit == "in" {
        in_range(value, 59, 76)
    } else {
        false
    }
}

/*
 * hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
 */
fn valid_hcl(value: &str) -> bool {
    if !value.starts_with('#') {
        return false;
    }
    let hex_value = value.trim_start_matches('#');
    let mut num_of_chars = 0;
    for c in hex_value.chars() {
        if c.is_ascii_hexdigit() {
            num_of_chars += 1;
        } else {
            return false;
        }
    }
    num_of_chars == 6
}

/*
 * ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
 */
fn valid_ecl(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

/*
 * pid (Passport ID) - a nine-digit number, including leading zeroes.
 */
fn valid_pid(value: &str) -> bool {
    let mut num_of_chars = 0;
    for c in value.chars() {
        if !c.is_ascii_digit() {
            return false;
        } else {
            num_of_chars += 1;
        }
    }
    num_of_chars == 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn part2_invalid() {
        let input = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\n iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_byr() {
        assert_eq!(in_range("2002", 1920, 2002), true);
        assert_eq!(in_range("2003", 1920, 2002), false);

        assert_eq!(in_range("1989", 1920, 2002), true);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(valid_hgt("60in"), true);
        assert_eq!(valid_hgt("190cm"), true);
        assert_eq!(valid_hgt("190in"), false);
        assert_eq!(valid_hgt("190"), false);

        assert_eq!(valid_hgt("165cm"), true);
    }

    #[test]
    fn test_hcl() {
        assert_eq!(valid_hcl("#123abc"), true);
        assert_eq!(valid_hcl("123abz"), false);
        assert_eq!(valid_hcl("123abc"), false);

        assert_eq!(valid_hcl("#a97842"), true);
    }

    #[test]
    fn test_iyr() {
        assert_eq!(in_range("2014", 2010, 2020), true);
    }

    #[test]
    fn test_eyr() {
        assert_eq!(in_range("2029", 2020, 2030), true);
    }

    #[test]
    fn test_ecl() {
        assert_eq!(valid_ecl("brn"), true);
        assert_eq!(valid_ecl("wat"), false);

        assert_eq!(valid_ecl("blu"), true);
    }

    #[test]
    fn test_pid() {
        assert_eq!(valid_pid("000000001"), true);
        assert_eq!(valid_pid("0123456789"), false);

        assert_eq!(valid_pid("896056539"), true);
    }
}
