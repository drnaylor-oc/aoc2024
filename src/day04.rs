use std::collections::{HashMap, HashSet};
use std::str::Lines;
use itertools::Itertools;
use crate::Day;
use crate::util::{load_from, Errors};
use crate::util::Errors::NoImplementationError;

pub struct Day04 {}

impl Day for Day04 {
    fn part_1(&self) -> Result<String, Errors> {
        let file = load_from("day04a.txt");
        let grid = parse_grid(file.lines());
        let start_points = find_all_x(&grid);
        let result = check_all_points(&start_points, &grid);
        Ok(format!("{}", result))
    }

    fn part_2(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day04 {})
    }
}

type XmasMap = HashMap<(usize, usize), Xmas>;

#[derive(Debug, PartialEq, PartialOrd)]
enum Xmas {
    X,
    M,
    A,
    S
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Direction::Up,
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
    ];

    fn coords(&self, start: (usize, usize)) -> Option<[(usize, usize); 3]> {
        match *self {
            Direction::Up => if (start.0 > 2) {
                Some([(start.0 - 1, start.1), (start.0 - 2, start.1), (start.0 - 3, start.1)])
            } else {
                None
            },
            Direction::UpRight => if (start.0 > 2) {
                Some([(start.0 - 1, start.1 + 1), (start.0 - 2, start.1 + 2), (start.0 - 3, start.1 + 3)])
            } else {
                None
            },
            Direction::Right => Some([(start.0, start.1 + 1), (start.0, start.1 + 2), (start.0, start.1 + 3)]),
            Direction::DownRight => Some([(start.0 + 1, start.1 + 1), (start.0 + 2, start.1 + 2), (start.0 + 3, start.1 + 3)]),
            Direction::Down => Some([(start.0 + 1, start.1), (start.0 + 2, start.1), (start.0 + 3, start.1)]),
            Direction::DownLeft => if (start.1 > 2) {
                Some([(start.0 + 1, start.1 - 1), (start.0 + 2, start.1 - 2), (start.0 + 3, start.1 - 3)])
            } else {
                None
            },
            Direction::Left => if (start.1 > 2) {
                Some([(start.0, start.1 - 1), (start.0, start.1 - 2), (start.0, start.1 - 3)])
            } else {
                None
            },
            Direction::UpLeft => if (start.0 > 2 && start.1 > 2) {
                Some([(start.0 - 1, start.1 - 1), (start.0 - 2, start.1 - 2), (start.0 - 3, start.1 - 3)])
            } else {
                None
            },
        }
    }
}

fn to_xmas(i: char) -> Option<Xmas> {
    match i {
        'X' => Some(Xmas::X),
        'M' => Some(Xmas::M),
        'A' => Some(Xmas::A),
        'S' => Some(Xmas::S),
        _   => None
    }
}

fn parse_grid(lines: Lines) -> XmasMap {
    let mut map = HashMap::new();
    let mut row = 0usize;
    let mut col = 0usize;
    for line in lines {
        for char in line.chars() {
            if let Some(xmas) = to_xmas(char) {
                map.insert((row, col), xmas);
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }
    map
}

fn find_all_x(map: &XmasMap) -> HashSet<(usize, usize)> {
    map.iter().filter_map(|((x, y), xmas)| {
        if *xmas == Xmas::X {
            Some((*x, *y))
        } else {
            None
        }
    }).collect()
}

fn test_for_xmas(start: (usize, usize), map: &XmasMap) -> usize {
    Direction::ALL.iter().filter(|x| {
        if let Some([m, a, s]) = x.coords(start) {
            map.get(&m) == Some(&Xmas::M) && map.get(&a) == Some(&Xmas::A) && map.get(&s) == Some(&Xmas::S)
        } else {
            false
        }
    }).count()

}

fn check_all_points(start_points: &HashSet<(usize, usize)>, grid: &XmasMap) -> usize {
    start_points.iter().map(|x| test_for_xmas(*x, grid)).sum()
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day04::{find_all_x, parse_grid, Xmas, XmasMap};

    const TEST_GRID: &str = "MMMSXXMASM\n\
                             MSAMXMSMSA\n\
                             AMXSXMAAMM\n\
                             MSAMASMSMX\n\
                             XMASAMXAMM\n\
                             XXAMMXXAMA\n\
                             SMSMSASXSS\n\
                             SAXAMASAAA\n\
                             MAMMMXMMMM\n\
                             MXMXAXMASX";

    const TEST_GRID_PARSE: &str = "MMMSX\n\
                             MSAMX\n\
                             AMXSX\n\
                             MSAMA\n\
                             XMASA";

    lazy_static! {
        static ref MAPPED_GRID: XmasMap = HashMap::from(
            [
                ((0, 0), Xmas::M),
                ((0, 1), Xmas::M),
                ((0, 2), Xmas::M),
                ((0, 3), Xmas::S),
                ((0, 4), Xmas::X),
                ((1, 0), Xmas::M),
                ((1, 1), Xmas::S),
                ((1, 2), Xmas::A),
                ((1, 3), Xmas::M),
                ((1, 4), Xmas::X),
                ((2, 0), Xmas::A),
                ((2, 1), Xmas::M),
                ((2, 2), Xmas::X),
                ((2, 3), Xmas::S),
                ((2, 4), Xmas::X),
                ((3, 0), Xmas::M),
                ((3, 1), Xmas::S),
                ((3, 2), Xmas::A),
                ((3, 3), Xmas::M),
                ((3, 4), Xmas::A),
                ((4, 0), Xmas::X),
                ((4, 1), Xmas::M),
                ((4, 2), Xmas::A),
                ((4, 3), Xmas::S),
                ((4, 4), Xmas::A)
            ]
        );
    }

    #[test]
    fn test_parse_grid() {
        assert_eq!(*MAPPED_GRID, parse_grid(TEST_GRID_PARSE.lines()))
    }

    #[rstest]
    #[case(Xmas::X, Xmas::M, true, false)]
    #[case(Xmas::X, Xmas::A, true, false)]
    #[case(Xmas::X, Xmas::S, true, false)]
    #[case(Xmas::M, Xmas::A, true, false)]
    #[case(Xmas::M, Xmas::S, true, false)]
    #[case(Xmas::A, Xmas::S, true, false)]
    #[case(Xmas::X, Xmas::X, false, false)]
    #[case(Xmas::M, Xmas::X, false, true)]
    #[case(Xmas::M, Xmas::M, false, false)]
    #[case(Xmas::A, Xmas::X, false, true)]
    #[case(Xmas::A, Xmas::M, false, true)]
    #[case(Xmas::A, Xmas::A, false, false)]
    #[case(Xmas::S, Xmas::X, false, true)]
    #[case(Xmas::S, Xmas::M, false, true)]
    #[case(Xmas::S, Xmas::A, false, true)]
    #[case(Xmas::S, Xmas::S, false, false)]
    fn test_order(#[case] first: Xmas, #[case] second: Xmas, #[case] first_is_less_than: bool, #[case] first_is_greater_than: bool) {
        assert_eq!(first < second, first_is_less_than)
    }

    #[test]
    fn test_find_all_x() {
        assert_eq!(find_all_x(&*MAPPED_GRID), HashSet::from([
            (0, 4),
            (1, 4),
            (2, 2),
            (2, 4),
            (4, 0),
        ]))
    }

}