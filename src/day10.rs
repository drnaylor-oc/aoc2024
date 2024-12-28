use std::collections::{HashMap, HashSet};
use crate::util::{load_from, Errors};
use crate::Day;
use itertools::Itertools;
use tailcall::tailcall;

pub struct Day10 {}

impl Day for Day10 {
    fn part_1(&self) -> Result<String, Errors> {
        let file = load_from("day10a.txt")?;
        let map = parse_map(&file);
        let ends = find_end_trailheads(&map);
        let keys: Vec<HashSet<(usize, usize)>> = ends.iter()
            .map(|x| x.keys().map(|x| *x).collect())
            .collect_vec();
        let count = count_scores(&keys);
        Ok(count.to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        let file = load_from("day10a.txt")?;
        let map = parse_map(&file);
        let ends = find_end_trailheads(&map);
        let count = ends.iter().map(|x| x.values().sum::<usize>()).sum::<usize>();
        Ok(count.to_string())
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day10 {})
    }
}

fn count_scores(heads: &Vec<HashSet<(usize, usize)>>) -> usize {
    heads.iter().map(|x| x.len()).sum()
}

fn find_end_trailheads(map: &HashMap<(usize, usize), u8>) -> Vec<HashMap<(usize, usize), usize>> {
    map.iter()
        .filter(|(_, &v)| v == 0)
        .map(|(&c, _)| walk_trail(map, HashMap::from([(c, 1)]), 1))
        .collect()
}

#[tailcall]
fn walk_trail(map: &HashMap<(usize, usize), u8>, current_coords: HashMap<(usize, usize), usize>, next_step: u8) -> HashMap<(usize, usize), usize> {
    let mut new_coords = HashMap::new();
    for (coord, paths) in current_coords {
        for i in next(coord, next_step, map) {
            *new_coords.entry(i).or_default() += paths;
        }
    }
    if new_coords.is_empty() || next_step == 9 {
        new_coords
    } else {
        walk_trail(map, new_coords, next_step + 1)
    }
}

fn next(current_loc: (usize, usize), next_id: u8, map: &HashMap<(usize, usize), u8>) -> Vec<(usize, usize)> {
    let mut next_trailhead: Vec<(usize, usize)> = vec![];
    if let Some(r) = current_loc.0.checked_sub(1).iter().filter_map(|x| is_next((*x, current_loc.1), next_id, map)).next() {
        next_trailhead.push(r);
    }
    if let Some(r) = is_next((current_loc.0 + 1, current_loc.1), next_id, map) {
        next_trailhead.push(r);
    }
    if let Some(r) = current_loc.1.checked_sub(1).iter().filter_map(|y| is_next((current_loc.0, *y), next_id, map)).next() {
        next_trailhead.push(r);
    }
    if let Some(r) = is_next((current_loc.0, current_loc.1 + 1), next_id, map) {
        next_trailhead.push(r);
    }
    next_trailhead
}

fn is_next(coord: (usize, usize), next_id: u8, map: &HashMap<(usize, usize), u8>) -> Option<(usize, usize)> {
    map.get(&coord).filter(|&&x| x == next_id).map(|_| coord)
}

fn parse_map(input: &str) -> HashMap<(usize, usize), u8> {
    let mut map: HashMap<(usize, usize), u8> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert((row, col), c.to_digit(10).unwrap() as u8);
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day10::{count_scores, find_end_trailheads, next, parse_map};

    const TEST_INPUT_1: &str = "0123\n\
                                1234\n\
                                8765\n\
                                9876";

    // const TEST_INPUT_2: &str = "89010123\n\
    //                             78121874\n\
    //                             87430965\n\
    //                             96549874\n\
    //                             45678903\n\
    //                             32019012\n\
    //                             01329801\n\
    //                             10456732";

    lazy_static! {
        static ref TEST_MAP_1: HashMap<(usize, usize), u8> = HashMap::from([
            ((0, 0), 0),
            ((0, 1), 1),
            ((0, 2), 2),
            ((0, 3), 3),
            ((1, 0), 1),
            ((1, 1), 2),
            ((1, 2), 3),
            ((1, 3), 4),
            ((2, 0), 8),
            ((2, 1), 7),
            ((2, 2), 6),
            ((2, 3), 5),
            ((3, 0), 9),
            ((3, 1), 8),
            ((3, 2), 7),
            ((3, 3), 6),
        ]);


    }

    #[rstest]
    #[case(TEST_INPUT_1, &TEST_MAP_1)]
    fn test_parse_map(#[case] input: &str, #[case] map: &HashMap<(usize, usize), u8>) {
        assert_eq!(parse_map(input), *map);
    }

    #[rstest]
    #[case(&TEST_MAP_1, 1, (0, 0), vec![(1, 0), (0, 1)])]
    #[case(&TEST_MAP_1, 2, (0, 0), vec![])]
    fn test_next(#[case] map: &HashMap<(usize, usize), u8>, #[case] next_id: u8, #[case] coord: (usize, usize), #[case] expected: Vec<(usize, usize)>) {
        assert_eq!(next(coord, next_id, map), expected);
    }


    #[rstest]
    #[case(&TEST_MAP_1, vec![HashMap::from_iter([((3, 0), 16)])])]
    fn test_find_end_trailheads(#[case] map: &HashMap<(usize, usize), u8>, #[case] expected: Vec<HashMap<(usize, usize), usize>>) {
        assert_eq!(find_end_trailheads(map), expected);
    }

    #[rstest]
    #[case(vec![HashSet::from_iter([(3usize, 0usize)])], 1)]
    #[case(vec![HashSet::from_iter([(3usize, 0usize), (0, 3)])], 2)]
    #[case(vec![HashSet::from_iter([(3usize, 0usize), (0, 3)]), HashSet::from_iter([(0, 3)])], 3)]
    fn test_count(#[case] map: Vec<HashSet<(usize, usize)>>, #[case] expected: usize) {
        assert_eq!(count_scores(&map), expected);
    }
}