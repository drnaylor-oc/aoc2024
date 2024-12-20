use crate::util::{load_from, Errors};
use crate::Day;
use core::str::Lines;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct Day08 {}

type Coord = (isize, isize);

impl Day for Day08 {
    fn part_1(&self) -> Result<String, Errors> {
        run(part_1_antinodes)
    }

    fn part_2(&self) -> Result<String, Errors> {
        run(part_2_antinodes)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day08 {})
    }
}

fn run(func: AntinodeFn) -> Result<String, Errors> {
    let str = load_from("day08a.txt")?;
    let (map, row_max, col_max) = parse_lines(str.lines());
    let antinodes = collect_antinodes(func, &map, row_max, col_max);
    Ok(antinodes.len().to_string())
}

fn parse_lines(line: Lines) -> (HashMap<char, Vec<Coord>>, isize, isize) {
    let mut coords: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut row_length = 0isize;
    let mut col_length = 0isize;
    for (row, line) in line.enumerate() {
        for (col, ant) in parse_line(line) {
            if let Some(v) = coords.get_mut(&ant) {
                v.push((row as isize, col as isize));
            } else {
                coords.insert(ant, vec![(row as isize, col as isize)]);
            }
        }
        if col_length == 0 {
            col_length = line.len() as isize;
        }
        row_length += 1;
    }

    (coords, row_length, col_length)
}

fn parse_line(line: &str) -> Vec<(usize, char)> {
    line.char_indices()
        .filter(|&(_, c)| c != '.')
        .collect_vec()
}

type AntinodeFn = fn(&Coord, &Coord, isize, isize) -> Vec<Coord>;

fn find_antinodes_for_antenna(func: AntinodeFn, like_antennas: &Vec<Coord>, row_max: isize, col_max: isize) -> HashSet<Coord> {
    like_antennas.iter().tuple_combinations()
        .flat_map(|(first, second)| {
            func(first, second, row_max, col_max)
        })
            .collect()
}

fn part_1_antinodes(first: &Coord, second: &Coord, row_max: isize, col_max: isize) -> Vec<Coord> {
    let col_step: isize = second.1 - first.1;
    let row_step: isize = second.0 - first.0;
    /*
     * As our delta is the step required from the first coord to the second coord, to get the
     * anti nodes, we must take one further step along the line (so, add the deltas from the
     * second coordinate), and one step back from the first point (so subtract from the first)
     */
    [
        (first.0 - row_step, first.1 - col_step),
        (second.0 + row_step, second.1 + col_step)
    ].iter().filter(|x| check_in_map(x, row_max, col_max)).cloned().collect()
}

fn part_2_antinodes(first: &Coord, second: &Coord, row_max: isize, col_max: isize) -> Vec<Coord> {
    let col_step: isize = second.1 - first.1;
    let row_step: isize = second.0 - first.0;
    let mut antinodes = vec![second.clone()];
    while let Some(next) = get_next_node(antinodes.last().unwrap(), row_step, col_step, row_max, col_max) {
        antinodes.push(next);
    }

    antinodes.push(first.clone());
    while let Some(next) = get_next_node(antinodes.last().unwrap(), -row_step, -col_step, row_max, col_max) {
        antinodes.push(next);
    }

    antinodes
}

fn get_next_node(current: &Coord, row_step: isize, col_step: isize, row_max: isize, col_max: isize) -> Option<Coord> {
    let next = (current.0 + row_step, current.1 + col_step);
    if check_in_map(&next, row_max, col_max) {
        Some(next)
    } else {
        None
    }
}

fn check_in_map(coord: &Coord, row_max: isize, col_max: isize) -> bool {
    coord.0 >= 0 && coord.1 >= 0 && coord.0 < row_max && coord.1 < col_max
}

fn collect_antinodes(func: AntinodeFn, all_antennas: &HashMap<char, Vec<Coord>>, row_max: isize, col_max: isize) -> HashSet<Coord> {
    all_antennas.iter()
        .flat_map(|(_, coords)| find_antinodes_for_antenna(func, coords, row_max, col_max))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use itertools::Itertools;
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day08::{collect_antinodes, find_antinodes_for_antenna, parse_lines, part_1_antinodes, part_2_antinodes, Coord};

    const TEST_INPUT: &str = "............\n\
                              ........0...\n\
                              .....0......\n\
                              .......0....\n\
                              ....0.......\n\
                              ......A.....\n\
                              ............\n\
                              ............\n\
                              ........A...\n\
                              .........A..\n\
                              ............\n\
                              ............";

    lazy_static! {
        static ref TEST_PARSED: HashMap<char, Vec<Coord>> = HashMap::from([
            ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
            ('A', vec![(5, 6), (8, 8), (9, 9)]),
        ]);
    }

    #[test]
    fn test_parse_lines() {
        let (parsed, r, c) = parse_lines(TEST_INPUT.lines());
        assert_eq!(parsed, *TEST_PARSED);
        assert_eq!(r, 12);
        assert_eq!(c, 12);
    }

    #[rstest]
    #[case('A', HashSet::from([(1, 3), (2, 4), (7, 7), (10, 10), (11, 10)]))]
    #[case('0', HashSet::from([(0, 6), (0, 11), (1, 3), (2, 10), (3, 2), (4, 9), (5, 1), (5, 6), (6, 3), (7, 0)]))]
    fn test_find_antinodes_for_antenna(#[case] key: char, #[case] expected: HashSet<Coord>) {
        assert_eq!(find_antinodes_for_antenna(part_1_antinodes,&(TEST_PARSED)[&key], 12, 12), expected);
    }

    #[rstest]
    #[case('A', HashSet::from([
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
        (10, 10),
        (11, 11),
        (1, 3),
        (2, 4),
        (5, 6),
        (11, 10),
    ]))]
    // #[case('0', HashSet::from([(0, 6), (0, 11), (1, 3), (2, 10), (3, 2), (4, 9), (5, 1), (5, 6), (6, 3), (7, 0)]))]
    fn test_find_antinodes_for_antenna_pt2(#[case] key: char, #[case] expected: HashSet<Coord>) {
        assert_eq!(find_antinodes_for_antenna(part_2_antinodes,&(TEST_PARSED)[&key], 12, 12), expected);
    }

    #[test]
    fn test_collect_antinodes() {
        assert_eq!(
            collect_antinodes(part_1_antinodes, &*TEST_PARSED, 12, 12),
            HashSet::from([(1, 3), (2, 4), (7, 7), (10, 10), (11, 10), (0, 6), (0, 11), (1, 3), (2, 10), (3, 2), (4, 9), (5, 1), (5, 6), (6, 3), (7, 0)])
        )
    }

    #[test]
    fn test_collect_antinodes_2() {
        assert_eq!(
            collect_antinodes(part_2_antinodes, &*TEST_PARSED, 12, 12).iter().sorted().collect_vec(),
            vec![
                (0, 0),
                (0, 1),
                (0, 6),
                (0, 11),
                (1, 1),
                (1, 3),
                (1, 8),
                (2, 2),
                (2, 4),
                (2, 5),
                (2, 10),
                (3, 2),
                (3, 3),
                (3, 7),
                (4, 4),
                (4, 9),
                (5, 1),
                (5, 5),
                (5, 6),
                (5, 11),
                (6, 3),
                (6, 6),
                (7, 0),
                (7, 5),
                (7, 7),
                (8, 2),
                (8, 8),
                (9, 4),
                (9, 9),
                (10, 1),
                (10, 10),
                (11, 3),
                (11, 10),
                (11, 11)
            ].iter().sorted().collect_vec()
        )
    }

}