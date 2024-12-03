use std::collections::HashSet;
use itertools::Itertools;
use crate::Day;
use crate::day02::Direction::{Decreasing, Increasing, NotStarted};
use crate::util::{load_from, Errors};

pub struct Day02 {}

impl Day for Day02 {
    fn part_1(&self) -> Result<String, Errors> {
        let file = load_from("day02a.txt");
        let lines = parse_lines(file.as_str());
        Ok(lines.iter().map(|x| is_safe(x, 1)).filter(|x| *x).count().to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        let file = load_from("day02a.txt");
        let lines = parse_lines(file.as_str());
        Ok(lines.iter().map(|x| is_safe(x, 2)).filter(|x| *x).count().to_string())
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day02 {})
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    NotStarted,
    Increasing,
    Decreasing
}

impl Direction {
    fn is_safe(&self, new: &Direction) -> bool {
        if *self == NotStarted || *new == NotStarted {
            true
        } else {
            *self == *new
        }
    }
}

fn parse_lines(p0: &str) -> Vec<Vec<isize>> {
    let mut lines: Vec<Vec<isize>> = Vec::new();
    for line in p0.lines() {
        lines.push(line.split_whitespace().map(|x| x.parse().unwrap()).collect());
    }
    lines
}

fn is_safe(line: &[isize], tolerance: usize) -> bool {
    let mut current_direction = NotStarted;
    let mut failures: HashSet<usize> = HashSet::new();
    for i in 0..(line.len() - 1) {
        let next = check(line, i, i+1, &current_direction);
        match next {
            Some(n) => {
                current_direction = n;
            },
            None => {
                failures.insert(i);
                failures.insert(i.saturating_sub(1));
                if i+1 < line.len() {
                    failures.insert(i + 1);
                }
            },
        }
    }

    if !failures.is_empty() && tolerance > 1 {
        for failure in failures {
            // a little brute force... but it'll do.
            let mut repaired = line.iter().map(|x| *x).collect_vec();
            repaired.remove(failure);
            if is_safe(&repaired, tolerance-1) {
                return true;
            }
        }
        false
    } else {
        failures.is_empty()
    }
}

fn check(line: &[isize], idx1: usize, idx2: usize, current_direction: &Direction) -> Option<Direction> {
    let first = line[idx1];
    let second = line[idx2];
    let r = first.abs_diff(second);
    let next_direction = if first < second { Increasing } else { Decreasing };
    if r == 0 || r > 3 || !current_direction.is_safe(&next_direction) {
        None
    } else {
        Some(next_direction)
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day02::{is_safe, parse_lines};

    const EXAMPLE: &str = "7 6 4 2 1\n\
                           1 2 7 8 9\n\
                           9 7 6 2 1\n\
                           1 3 2 4 5\n\
                           8 6 4 4 1\n\
                           1 3 6 7 9";

    lazy_static! {
        static ref EXAMPLE_PARSED: Vec<Vec<isize>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_lines(EXAMPLE), *EXAMPLE_PARSED);
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true, true)]
    #[case(vec![1, 2, 7, 8, 9], false, false)]
    #[case(vec![9, 7, 6, 2, 1], false, false)]
    #[case(vec![1, 3, 2, 4, 5], false, true)]
    #[case(vec![8, 6, 4, 4, 1], false, true)]
    #[case(vec![1, 3, 6, 7, 9], true, true)]
    #[case(vec![93, 92, 92, 90, 92], false, false)] // taken from list
    #[case(vec![65, 69, 71, 74, 76, 78], false, true)] // taken from list
    #[case(vec![69, 71, 74, 76, 78], true, true)] // taken from list
    #[case(vec![76, 74, 77, 79, 82, 83, 85, 88], false, true)]
    fn test_is_safe(#[case] input: Vec<isize>, #[case] expected_not_tol: bool, #[case] expected_tol: bool) {
        assert_eq!(is_safe(&input, 1), expected_not_tol, "Zero tolerance, {:?}", input);
        assert_eq!(is_safe(&input, 2), expected_tol, "One tolerance, {:?}", input);
    }

}