use crate::util::Errors::NoImplementationError;
use crate::util::{load_from, Errors};
use crate::Day;
use core::str::Lines;
use std::collections::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day05 {}

impl Day for Day05 {
    fn part_1(&self) -> Result<String, Errors> {
        let text = load_from("day05a.txt");
        let mut lines = text.lines();
        let ordering = parse_ordering(&mut lines);
        let updates = parse_update(&mut lines);
        let result = updates.iter().filter_map(|x| check_order(&ordering, x)).sum::<usize>();
        Ok(format!("{}", result))
    }

    fn part_2(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day05 {})
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Order {
    Forwards,
    Backwards
}

lazy_static! {
    static ref ORDERING_REGEX: Regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
}

fn parse_ordering(lines: &mut Lines) -> HashMap<(usize, usize), Order> {
    let mut order: HashMap<(usize, usize), Order> = HashMap::new();
    while let Some(Some(m)) = lines.next().map(|x| (*ORDERING_REGEX).find(x)) {
        let r: (usize, usize) = m
            .as_str()
            .split_once("|")
            .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
            .unwrap();
        order.insert(r, Order::Forwards); // first then second
        order.insert((r.1, r.0), Order::Backwards);
    };
    order
}

fn parse_update(lines: &mut Lines) -> Vec<Vec<usize>> {
    let mut updates: Vec<Vec<usize>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.contains(",") {
            updates.push(
                line.split(",").map(|x| x.parse().unwrap()).collect()
            );
        }
    };
    updates
}

fn check_order(ordering: &HashMap<(usize, usize), Order>, vec: &Vec<usize>) -> Option<usize> {
    let mut is_unspecified = false;
    for (first_idx, first_value) in vec.iter().enumerate() {
        let mut inner_unspecified = true;
        for second_value in vec.iter().skip(first_idx + 1) {
            match ordering.get(&(*first_value, *second_value)) {
                Some(Order::Forwards) => { inner_unspecified = false; },
                Some(Order::Backwards) => { return None }, // not valid, return immediately
                None => {} // status quo
            }
        }
        if is_unspecified && !inner_unspecified {
            return None;
        }
        is_unspecified = inner_unspecified;
    };
    // If we get here, then the sequence is valid, so we find the middle.
    let middle_idx = vec.iter().count() / 2;
    vec.get(middle_idx).map(|x| *x)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day05::{check_order, parse_ordering, parse_update, Order};

    const TEST_ENTRY: &str = "47|53\n\
                              97|13\n\
                              97|61\n\
                              97|47\n\
                              75|29\n\
                              61|13\n\
                              75|53\n\
                              29|13\n\
                              97|29\n\
                              53|29\n\
                              61|53\n\
                              97|53\n\
                              61|29\n\
                              47|13\n\
                              75|47\n\
                              97|75\n\
                              47|61\n\
                              75|61\n\
                              47|29\n\
                              75|13\n\
                              53|13\n\n\
                              75,47,61,53,29\n\
                              97,61,53,29,13\n\
                              75,29,13\n\
                              75,97,47,61,53\n\
                              61,13,29\n\
                              97,13,75,29,47";

    lazy_static! {
        static ref TEST_ORDERING: HashMap<(usize, usize), Order> = HashMap::from([
            ((47,53), Order::Forwards),
            ((97,13), Order::Forwards),
            ((97,61), Order::Forwards),
            ((97,47), Order::Forwards),
            ((75,29), Order::Forwards),
            ((61,13), Order::Forwards),
            ((75,53), Order::Forwards),
            ((29,13), Order::Forwards),
            ((97,29), Order::Forwards),
            ((53,29), Order::Forwards),
            ((61,53), Order::Forwards),
            ((97,53), Order::Forwards),
            ((61,29), Order::Forwards),
            ((47,13), Order::Forwards),
            ((75,47), Order::Forwards),
            ((97,75), Order::Forwards),
            ((47,61), Order::Forwards),
            ((75,61), Order::Forwards),
            ((47,29), Order::Forwards),
            ((75,13), Order::Forwards),
            ((53,13), Order::Forwards),
            ((53,47), Order::Backwards),
            ((13,97), Order::Backwards),
            ((61,97), Order::Backwards),
            ((47,97), Order::Backwards),
            ((29,75), Order::Backwards),
            ((13,61), Order::Backwards),
            ((53,75), Order::Backwards),
            ((13,29), Order::Backwards),
            ((29,97), Order::Backwards),
            ((29,53), Order::Backwards),
            ((53,61), Order::Backwards),
            ((53,97), Order::Backwards),
            ((29,61), Order::Backwards),
            ((13,47), Order::Backwards),
            ((47,75), Order::Backwards),
            ((75,97), Order::Backwards),
            ((61,47), Order::Backwards),
            ((61,75), Order::Backwards),
            ((29,47), Order::Backwards),
            ((13,75), Order::Backwards),
            ((13,53), Order::Backwards),
        ]);

         static ref TEST_DATA: Vec<Vec<usize>> = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ];
    }

    #[test]
    fn test_parse_entry() {
        let mut lines = TEST_ENTRY.lines();
        let result = parse_ordering(&mut lines);
        assert_eq!(result.iter().count(), (*TEST_ORDERING).iter().count());
        assert_eq!(result, *TEST_ORDERING);
        assert_eq!(lines.next().unwrap(), "75,47,61,53,29");
    }

    #[test]
    fn test_parse_update() {
        let mut l = TEST_ENTRY.lines();
        while l.next().unwrap().contains("|") {} // read lines until we get the newline.
        let result = parse_update(&mut l);
        assert_eq!(result.iter().count(), (*TEST_DATA).iter().count());
        assert_eq!(result, *TEST_DATA)
    }

    #[rstest]
    #[case(vec![75,47,61,53,29], Some(61))]
    #[case(vec![97,61,53,29,13], Some(53))]
    #[case(vec![75,29,13], Some(29))]
    #[case(vec![75,97,47,61,53], None)]
    #[case(vec![61,13,29], None)]
    #[case(vec![97,13,75,29,47], None)]
    fn test_check_order(#[case] input: Vec<usize>, #[case] expected: Option<usize>) {
        assert_eq!(check_order(&*TEST_ORDERING, &input), expected);
    }

}