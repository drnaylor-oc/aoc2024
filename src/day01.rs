use itertools::Itertools;
use sorted_vec::SortedVec;
use crate::Day;
use crate::util::{load_from, Errors};

pub struct Day01 {}

impl Day for Day01 {
    fn part_1(&self) -> Result<String, Errors> {
        let file = load_from("day01a.txt")?;
        let (first, second) = create_lists(file);
        Ok(format!("{}", get_distance_sum(first, second)))
    }

    fn part_2(&self) -> Result<String, Errors> {
        let file = load_from("day01a.txt")?;
        let (first, second) = create_lists_unsorted(file);
        Ok(format!("{}", count_lists(first, second)))
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day01 {})
    }
}

fn count_lists(first: Vec<u64>, second: Vec<u64>) -> u64 {
    let s_counts = second.iter().counts();

    let mut result: u64 = 0;
    for val in first {
        result += val * s_counts.get(&val).map(usize::clone).map(|x| x as u64).unwrap_or(0);
    }
    result
}

fn create_lists_unsorted(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut first: Vec<u64> = Vec::new();
    let mut second: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut iter = line.trim().split_whitespace();
        first.push(iter.next().unwrap().parse().unwrap());
        second.push(iter.next().unwrap().parse().unwrap());
    }
    (first, second)
}

fn create_lists(input: String) -> (SortedVec<u64>, SortedVec<u64>) {
    let mut first: SortedVec<u64> = SortedVec::new();
    let mut second: SortedVec<u64> = SortedVec::new();
    for line in input.lines() {
        let mut iter = line.trim().split_whitespace();
        first.insert(iter.next().unwrap().parse().unwrap());
        second.insert(iter.next().unwrap().parse().unwrap());
    }
    (first, second)
}

fn get_distance_sum(first: SortedVec<u64>, second: SortedVec<u64>) -> u64 {
    let mut r: u64 = 0;
    for i in 0..first.len() {
        r += second[i].abs_diff(first[i]);
    }
    r
}

#[cfg(test)]
mod test {
    use sorted_vec::SortedVec;
    use crate::day01::{count_lists, create_lists, get_distance_sum};

    const EXAMPLE1: &str = "3   4\n\
                            4   3\n\
                            2   5\n\
                            1   3\n\
                            3   9\n\
                            3   3";

    #[test]
    fn test_create_lists() {
        let (first, second) = create_lists(EXAMPLE1.to_string());
        assert_eq!(first.to_vec(), vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(second.to_vec(), vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn test_get_distance_sum() {
        let first: SortedVec<u64> = SortedVec::from(vec![1, 2, 3, 3, 3, 4]);
        let second: SortedVec<u64> = SortedVec::from(vec![3, 3, 3, 4, 5, 9]);
        assert_eq!(get_distance_sum(first, second), 11);
    }

    #[test]
    fn test_count_lists() {
        let first: Vec<u64> = vec![1, 2, 3, 3, 3, 4];
        let second: Vec<u64> = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(count_lists(first, second), 31);
    }

}