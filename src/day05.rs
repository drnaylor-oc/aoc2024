use crate::util::Errors::NoImplementationError;
use crate::util::{load_from, Errors};
use crate::Day;
use core::str::Lines;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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
        let text = load_from("day05a.txt");
        let mut lines = text.lines();
        let ordering = parse_ordering(&mut lines);
        // we only want lines that are not valid.
        let updates: Vec<Vec<usize>> = parse_update(&mut lines).iter().filter(|x| check_order(&ordering, *x).is_none()).map(Vec::clone).collect();
        let listed = get_listed_numbers(&ordering);
        let result = updates.iter().map(|x| sort_and_get_middle(&ordering, &listed, x)).sum::<usize>();
        Ok(format!("{}", result))
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day05 {})
    }
}

lazy_static! {
    static ref ORDERING_REGEX: Regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
}

fn parse_ordering(lines: &mut Lines) -> HashMap<(usize, usize), Ordering> {
    let mut order: HashMap<(usize, usize), Ordering> = HashMap::new();
    while let Some(Some(m)) = lines.next().map(|x| (*ORDERING_REGEX).find(x)) {
        let r: (usize, usize) = m
            .as_str()
            .split_once("|")
            .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
            .unwrap();
        order.insert(r, Ordering::Less); // first then second
        order.insert((r.1, r.0), Ordering::Greater);
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

fn check_order(ordering: &HashMap<(usize, usize), Ordering>, vec: &Vec<usize>) -> Option<usize> {
    let mut is_unspecified = false;
    for (first_idx, first_value) in vec.iter().enumerate() {
        let mut inner_unspecified = true;
        for second_value in vec.iter().skip(first_idx + 1) {
            match ordering.get(&(*first_value, *second_value)) {
                Some(Ordering::Less) => { inner_unspecified = false; },
                Some(Ordering::Greater) => { return None }, // not valid, return immediately
                _ => {} // status quo
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

fn sort_and_get_middle(ordering: &HashMap<(usize, usize), Ordering>, listed: &HashSet<usize>, vec: &Vec<usize>) -> usize {
    let mut actual_vec = vec.iter().filter(|x| listed.contains(x)).collect::<Vec<&usize>>();
    actual_vec.sort_by(|first, second| {
        *ordering.get(&(**first, **second)).unwrap_or(&Ordering::Equal)
    });
    *actual_vec[vec.len() / 2]
}

fn get_listed_numbers(map: &HashMap<(usize, usize), Ordering>) -> HashSet<usize> {
    map.keys().map(|(first, _)| *first).collect()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day05::{check_order, get_listed_numbers, parse_ordering, parse_update, sort_and_get_middle};

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
        static ref TEST_ORDERING: HashMap<(usize, usize), Ordering> = HashMap::from([
            ((47,53), Ordering::Less),
            ((97,13), Ordering::Less),
            ((97,61), Ordering::Less),
            ((97,47), Ordering::Less),
            ((75,29), Ordering::Less),
            ((61,13), Ordering::Less),
            ((75,53), Ordering::Less),
            ((29,13), Ordering::Less),
            ((97,29), Ordering::Less),
            ((53,29), Ordering::Less),
            ((61,53), Ordering::Less),
            ((97,53), Ordering::Less),
            ((61,29), Ordering::Less),
            ((47,13), Ordering::Less),
            ((75,47), Ordering::Less),
            ((97,75), Ordering::Less),
            ((47,61), Ordering::Less),
            ((75,61), Ordering::Less),
            ((47,29), Ordering::Less),
            ((75,13), Ordering::Less),
            ((53,13), Ordering::Less),
            ((53,47), Ordering::Greater),
            ((13,97), Ordering::Greater),
            ((61,97), Ordering::Greater),
            ((47,97), Ordering::Greater),
            ((29,75), Ordering::Greater),
            ((13,61), Ordering::Greater),
            ((53,75), Ordering::Greater),
            ((13,29), Ordering::Greater),
            ((29,97), Ordering::Greater),
            ((29,53), Ordering::Greater),
            ((53,61), Ordering::Greater),
            ((53,97), Ordering::Greater),
            ((29,61), Ordering::Greater),
            ((13,47), Ordering::Greater),
            ((47,75), Ordering::Greater),
            ((75,97), Ordering::Greater),
            ((61,47), Ordering::Greater),
            ((61,75), Ordering::Greater),
            ((29,47), Ordering::Greater),
            ((13,75), Ordering::Greater),
            ((13,53), Ordering::Greater),
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

    #[rstest]
    #[case(vec![75,97,47,61,53], 47)]
    #[case(vec![61,13,29], 29)]
    #[case(vec![97,13,75,29,47], 47)]
    fn test_sort(#[case] input: Vec<usize>, #[case] expected: usize) {
        assert_eq!(sort_and_get_middle(&*TEST_ORDERING, &get_listed_numbers(&*TEST_ORDERING), &input), expected);
    }

}