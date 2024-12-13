use itertools::Itertools;
use regex::{Captures, Regex};
use crate::Day;
use crate::util::{load_from, Errors};

pub struct Day03 {}

impl Day for Day03 {
    fn part_1(&self) -> Result<String, Errors> {
        let file = load_from("day03a.txt")?;
        let couples = scan_string(file.as_str());
        let result = mul_sum(&couples);
        Ok(result.to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        let file = load_from("day03a.txt")?;
        let ins = scan_enable_string(file.as_str());
        let result = mul_sum_enable(&ins);
        Ok(result.to_string())
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day03 {})
    }
}

const MUL_REGEX: &str = r"mul\((?<first>\d+),(?<second>\d+)\)";
const ENABLE_MUL_REGEX: &str = r"(?<enable>do\(\))|(?<disable>don\'t\(\))|mul\((?<first>\d+),(?<second>\d+)\)";

#[derive(Debug, PartialEq)]
enum Instruction {
    Enable,
    Disable,
    Mul(isize, isize)
}

fn mul_sum(vals: &Vec<(isize, isize)>) -> isize {
    vals.iter().map(|(a, b)| a * b).sum()
}

fn mul_sum_enable(vals: &Vec<Instruction>) -> isize {
    let mut enabled = true;
    let mut result = 0isize;
    for inst in vals {
        match inst {
            Instruction::Enable => { enabled = true; },
            Instruction::Disable => { enabled = false; },
            Instruction::Mul(a, b) if enabled => { result = result + a*b },
            _ => {}
        }
    }
    result
}

fn scan_string(string: &str) -> Vec<(isize, isize)> {
    let regex = Regex::new(MUL_REGEX).unwrap();
    regex
        .captures_iter(string)
        .map(|x| (
            extract_capture(&x, "first"),
            extract_capture(&x, "second")
        ))
        .collect_vec()
}

fn scan_enable_string(string: &str) -> Vec<Instruction> {
    let regex = Regex::new(ENABLE_MUL_REGEX).unwrap();
    regex
        .captures_iter(string)
        .map(|x| {
            if x.name("enable").is_some() {
                Instruction::Enable
            } else if x.name("disable").is_some() {
                Instruction::Disable
            } else {
                Instruction::Mul(
                    extract_capture(&x, "first"),
                    extract_capture(&x, "second")
                )
            }
        })
        .collect_vec()
}

fn extract_capture(capture: &Captures, name: &str) -> isize {
    capture.name(name).map(|x| x.as_str().parse::<isize>()).unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day03::{mul_sum, mul_sum_enable, scan_enable_string, scan_string, Instruction};

    const SAMPLE_DATA: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_DATA_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_scan_string() {
        assert_eq!(scan_string(SAMPLE_DATA), vec![(2,4), (5,5), (11,8), (8,5)])
    }

    #[test]
    fn test_mul_sum() {
        assert_eq!(mul_sum(&vec![(2,4), (5,5), (11,8), (8,5)]), 161);
    }

    #[test]
    fn test_scan_enable_string() {
        assert_eq!(scan_enable_string(SAMPLE_DATA_TWO), vec![Instruction::Mul(2,4), Instruction::Disable, Instruction::Mul(5,5), Instruction::Mul(11,8), Instruction::Enable, Instruction::Mul(8,5)])
    }

    #[test]
    fn test_mul_sum_enable() {
        assert_eq!(mul_sum_enable(&vec![Instruction::Mul(2,4), Instruction::Disable, Instruction::Mul(5,5), Instruction::Mul(11,8), Instruction::Enable, Instruction::Mul(8,5)]), 48)
    }
}