use crate::util::{load_from, Errors};
use crate::Day;
use core::str::Lines;
use itertools::Itertools;
use std::collections::HashMap;
use tailcall::tailcall;

pub struct Day07 {}

impl Day for Day07 {
    fn part_1(&self) -> Result<String, Errors> {
        run(false)
    }

    fn part_2(&self) -> Result<String, Errors> {
        run(true)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day07 {})
    }
}

fn run(part2: bool) -> Result<String, Errors> {
    let string = load_from("day07a.txt")?;
    let value_map = parse_lines(string.lines());
    let result: usize = value_map.iter()
        .map(|(target, values)| operate(vec![], values, *target, part2))
        .sum();
    Ok(format!("{}", result))
}

#[tailcall]
fn operate(current_state: Vec<usize>, values: &[usize], expected: usize, with_concatenation: bool) -> usize {
    if values.is_empty() {
        if current_state.iter().any(|node| *node == expected) {
            // we need to sum on this node
            expected
        } else {
            // not valid, don't add to sum
            0
        }
    } else {
        let next = values[0];
        let next_state: Vec<usize> = if current_state.is_empty() {
            vec![next]
        } else {
            current_state.iter().flat_map(|val| {
                [
                    evaluate_valid(val + next, expected),
                    evaluate_valid(val * next, expected),
                    if with_concatenation { Some(concat_digits(*val, next)) } else { None }
                ]
                    .iter()
                    .filter_map(|x| *x)
                    .collect_vec()
            }).collect_vec()
        };
        operate(next_state, values.split_first().map(|x| x.1).unwrap_or(&[]), expected, with_concatenation)
    }
}

fn concat_digits(lhs: usize, rhs: usize) -> usize {
    let left_shift = get_digits(rhs);
    lhs * 10_usize.pow(left_shift) + rhs
}

fn get_digits(value: usize) -> u32 {
    value.checked_ilog10().unwrap_or(0) + 1
}

fn evaluate_valid(result: usize, expected: usize) -> Option<usize> {
    if result > expected {
        None
    } else {
        Some(result)
    }
}

fn parse_lines(lines: Lines) -> HashMap<usize, Vec<usize>> {
    lines.map(parse_line).collect()
}

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    line.split_once(": ")
        .map(|(target, numbers)| {
            (
                target.parse::<usize>().unwrap(),
                numbers.trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect()
            )
        }).unwrap()
}



#[cfg(test)]
mod tests {
    use crate::day07::{operate, parse_line, parse_lines, concat_digits};
    use lazy_static::lazy_static;
    use rstest::rstest;
    use std::collections::HashMap;

    const TEST_INPUT: &str = "190: 10 19\n\
                              3267: 81 40 27\n\
                              83: 17 5\n\
                              156: 15 6\n\
                              7290: 6 8 6 15\n\
                              161011: 16 10 13\n\
                              192: 17 8 14\n\
                              21037: 9 7 18 13\n\
                              292: 11 6 16 20";

    lazy_static! {
        static ref TEST_RESULT: HashMap<usize, Vec<usize>> = HashMap::from(
            [
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ]
        );
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines(TEST_INPUT.lines()), *TEST_RESULT);
    }

    #[rstest]
    #[case("190: 10 19", (190, vec![10, 19]))]
    #[case("3267: 81 40 27", (3267, vec![81, 40, 27]))]
    #[case("83: 17 5", (83, vec![17, 5]))]
    #[case("156: 15 6", (156, vec![15, 6]))]
    #[case("7290: 6 8 6 15", (7290, vec![6, 8, 6, 15]))]
    #[case("161011: 16 10 13", (161011, vec![16, 10, 13]))]
    #[case("192: 17 8 14", (192, vec![17, 8, 14]))]
    #[case("21037: 9 7 18 13", (21037, vec![9, 7, 18, 13]))]
    #[case("292: 11 6 16 20", (292, vec![11, 6, 16, 20]))]
    fn test_parse_line(#[case] input: &str, #[case] expected: (usize, Vec<usize>)) {
        assert_eq!(parse_line(input), expected);
    }

    #[rstest]
    #[case(vec![10, 19], 190, true)]
    #[case(vec![81, 40, 27], 3267, true)]
    #[case(vec![17, 5], 83, false)]
    #[case(vec![15, 6], 156, false)]
    #[case(vec![6, 8, 6, 15], 7290, false)]
    #[case(vec![16, 10, 13], 161011, false)]
    #[case(vec![17, 8, 14], 192, false)]
    #[case(vec![9, 7, 18, 13], 21037, false)]
    #[case(vec![11, 6, 16, 20], 292, true)]
    fn test_operate_no_concat(#[case] values: Vec<usize>, #[case] target: usize, #[case] valid: bool) {
        assert_eq!(operate(vec![], &values, target, false), if valid { target } else { 0 });
    }

    #[rstest]
    #[case(vec![10, 19], 190, true)]
    #[case(vec![81, 40, 27], 3267, true)]
    #[case(vec![17, 5], 83, false)]
    #[case(vec![15, 6], 156, true)]
    #[case(vec![6, 8, 6, 15], 7290, true)]
    #[case(vec![16, 10, 13], 161011, false)]
    #[case(vec![17, 8, 14], 192, true)]
    #[case(vec![9, 7, 18, 13], 21037, false)]
    #[case(vec![11, 6, 16, 20], 292, true)]
    fn test_operate_concat(#[case] values: Vec<usize>, #[case] target: usize, #[case] valid: bool) {
        assert_eq!(operate(vec![], &values, target, true), if valid { target } else { 0 });
    }

    #[rstest]
    #[case(10, 19, 1019)]
    #[case(81, 40, 8140)]
    #[case(17, 5, 175)]
    #[case(15, 6, 156)]
    #[case(6, 8, 68)]
    #[case(16, 10, 1610)]
    #[case(17, 8, 178)]
    #[case(9, 7, 97)]
    #[case(11, 6, 116)]
    #[case(11, 0, 110)]
    fn test_concat_digits(#[case] lhs: usize, #[case] rhs: usize, #[case] expected: usize) {
        assert_eq!(concat_digits(lhs, rhs), expected);
    }

}