use std::collections::HashMap;
use crate::util::{load_from, Errors};
use crate::Day;
use tailcall::tailcall;

pub struct Day11 {}

impl Day for Day11 {
    fn part_1(&self) -> Result<String, Errors> {
        run(25)
    }

    fn part_2(&self) -> Result<String, Errors> {
        run(75)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day11 {})
    }
}

fn run(times: usize) -> Result<String, Errors> {
    let line = load_from("day11a.txt")?;
    let tokens = parse_tokens(&line);
    let result: HashMap<usize, usize> = blink_multiple(vec_to_map(&tokens), times, &mut HashMap::new());
    let sum = result.values().sum::<usize>();
    Ok(sum.to_string())
}

fn parse_tokens(line: &str) -> Vec<usize> {
    line.split(' ').map(|x| x.parse::<usize>().unwrap()).collect()
}

fn count_digits(value: usize) -> u32 {
    if value == 0 {
        1
    } else {
        value.ilog10() + 1
    }
}

// Order doesn't actually matter for what we're looking for.
fn vec_to_map(vec: &Vec<usize>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &v in vec {
        *map.entry(v).or_default() += 1
    }
    map
}

fn process_stone(input: usize, cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    cache.entry(input).or_insert_with(|| {
        if input == 0 {
            vec![1]
        } else {
            let digits_count = count_digits(input);
            if digits_count % 2 == 0 {
                // if we have 2 digits, then we need the units, and the tens, so 10^1 is 10.
                // Division: For 10, 10/10 gives 1, for 25, 25/10 gives 2.
                // Modulus, 10 % 10 gives 0, 25 % 10 gives 5.
                //
                // If we have four digits, 1000 -> 10^2 -> 100
                // 1000 / 100 -> 10, 1000 % 100 -> 0
                // 2555 / 100 -> 15, 2555 % 100 -> 55
                let half_point = 10usize.pow(digits_count / 2);
                vec![
                    input / half_point,
                    input % half_point
                ]
            } else {
                vec![input * 2024]
            }
        }
    }).clone()
}

fn blink(input: HashMap<usize, usize>, cache: &mut HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    let mut next_map: HashMap<usize, usize> = HashMap::new();
    for (key, count) in input {
        for v in process_stone(key, cache) {
            *next_map.entry(v).or_default() += count;
        }
    }
    next_map
}

#[tailcall]
fn blink_multiple(input: HashMap<usize, usize>, count: usize, cache: &mut HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    if count == 0 {
        input
    } else {
        blink_multiple(blink(input, cache), count - 1, cache)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day11::{blink, blink_multiple, count_digits, parse_tokens, process_stone, vec_to_map};
    use lazy_static::lazy_static;
    use rstest::rstest;

    const TEST_INPUT: &str = "125 17";

    lazy_static! {
        static ref TEST_PARSED: Vec<usize> = vec![125, 17];
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_tokens(TEST_INPUT), *TEST_PARSED)
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(20, 2)]
    #[case(99, 2)]
    #[case(200, 3)]
    #[case(12345, 5)]
    fn test_count_digits(#[case] input: usize, #[case] length: u32) {
        assert_eq!(count_digits(input), length);
    }

    #[rstest]
    #[case(0, vec![1])]
    #[case(1, vec![2024])]
    #[case(2, vec![4048])]
    #[case(10, vec![1, 0])]
    #[case(35, vec![3, 5])]
    #[case(99, vec![9, 9])]
    #[case(100, vec![202400])]
    #[case(1000, vec![10, 0])]
    fn test_process_stone(#[case] value: usize, #[case] expected: Vec<usize>) {
        assert_eq!(process_stone(value, &mut HashMap::new()), expected);
    }

    #[rstest]
    #[case(vec![125, 17], vec![253000, 1, 7])]
    #[case(vec![253000, 1, 7], vec![253, 0, 2024, 14168])]
    #[case(vec![253, 0, 2024, 14168], vec![512072, 1, 20, 24, 28676032])]
    #[case(vec![512072, 1, 20, 24, 28676032], vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])]
    #[case(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032], vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32])]
    #[case(vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32], vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2])]
    fn test_blink(#[case] input: Vec<usize>, #[case] expected: Vec<usize>) {
        assert_eq!(blink(vec_to_map(&input), &mut HashMap::new()), vec_to_map(&expected));
    }

    #[rstest]
    #[case(1, vec![253000, 1, 7])]
    #[case(2, vec![253, 0, 2024, 14168])]
    #[case(3, vec![512072, 1, 20, 24, 28676032])]
    #[case(4, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])]
    #[case(5, vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32])]
    #[case(6, vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2])]
    fn test_multi_blink(#[case] times: usize, #[case] expected: Vec<usize>) {
        assert_eq!(blink_multiple(vec_to_map(&vec![125, 17]), times, &mut HashMap::new()), vec_to_map(&expected));
    }

}