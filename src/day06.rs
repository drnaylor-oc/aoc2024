use std::collections::HashSet;
use std::str::Lines;
use crate::util::Errors::NoImplementationError;
use crate::util::{load_from, Errors};
use crate::Day;
use itertools::Itertools;

pub struct Day06 {}

impl Day for Day06 {
    fn part_1(&self) -> Result<String, Errors> {
        let data = load_from("day06a.txt");
        let mut state = parse_grid(data.lines());
        walk(&mut state);
        Ok(state.visited.len().to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day06 {})
    }
}


// (row, col)
type Coord = (isize, isize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    position: Coord,
    direction: Direction,
    visited: HashSet<Coord>,
    obstacles: HashSet<Coord>,
    rows: usize,
    cols: usize
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self, current: &Coord) -> Coord {
        match self {
            Direction::North => (current.0 - 1, current.1),
            Direction::East => (current.0, current.1 + 1),
            Direction::South => (current.0 + 1, current.1),
            Direction::West => (current.0, current.1 - 1)
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East =>  Direction::South,
            Direction::South => Direction::West,
            Direction::West =>  Direction::North,
        }
    }
}

fn walk(state: &mut State) {
    loop {
        let proposed = state.direction.step(&state.position);
        // if we are in row or col -1, or beyond the last row or column (in row or col 130 in my input), we've left.
        if proposed.0 >= 0 && proposed.1 >= 0 && proposed.0 < state.rows as isize && proposed.1 < state.cols as isize {
            if state.obstacles.contains(&proposed) {
                // discard proposed move, rotate 90 deg to right
                state.direction = state.direction.rotate();
            } else {
                // confirm move, insert step into tracking set
                state.position = proposed;
                state.visited.insert(proposed.clone());
            }
        } else {
            break;
        }
    }
}

fn parse_grid(lines: Lines) -> State {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut obstacles: HashSet<Coord> = HashSet::new();
    let mut position: Coord = (0, 0);
    for line in lines {
        if cols == 0 {
            cols = line.len();
        }
        line.match_indices('#').map(|(pos, _)| { pos }).for_each(|pos| {
            obstacles.insert((rows as isize, pos as isize));
        });
        if let Some(idx) = line.find('^') {
            position = (rows as isize, idx as isize);
        }
        rows += 1;
    }
    State { position, direction: Direction::North, obstacles, visited: HashSet::from([position.clone()]), rows, cols }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use lazy_static::lazy_static;
    use rstest::rstest;
    use crate::day06::{parse_grid, walk, Coord, Direction, State};

    const TEST_GRID: &str = "....#.....\n\
                             .........#\n\
                             ..........\n\
                             ..#.......\n\
                             .......#..\n\
                             ..........\n\
                             .#..^.....\n\
                             ........#.\n\
                             #.........\n\
                             ......#...";

    lazy_static! {
        static ref TEST_STATE: State = State {
            position: (6, 4),
            visited: HashSet::from([(6, 4)]),
            direction: Direction::North,
            obstacles: HashSet::from([
                (0, 4),
                (1, 9),
                (3, 2),
                (4, 7),
                (6, 1),
                (7, 8),
                (8, 0),
                (9, 6)
            ]),
            rows: 10,
            cols: 10
        };
    }


    #[rstest]
    #[case(Direction::North, Direction::East)]
    #[case(Direction::East, Direction::South)]
    #[case(Direction::South, Direction::West)]
    #[case(Direction::West, Direction::North)]
    fn test_rotate(#[case] original: Direction, #[case] expected: Direction) {
        assert_eq!(original.rotate(), expected);
    }

    #[rstest]
    #[case(Direction::North, (0, 0), (-1, 0))]
    #[case(Direction::North, (1, 0), (0, 0))]
    #[case(Direction::North, (0, 1), (-1, 1))]
    #[case(Direction::North, (1, 1), (0, 1))]
    #[case(Direction::East, (0, 0), (0, 1))]
    #[case(Direction::East, (1, 0), (1, 1))]
    #[case(Direction::East, (0, 1), (0, 2))]
    #[case(Direction::East, (1, 1), (1, 2))]
    #[case(Direction::South, (0, 0), (1, 0))]
    #[case(Direction::South, (1, 0), (2, 0))]
    #[case(Direction::South, (0, 1), (1, 1))]
    #[case(Direction::South, (1, 1), (2, 1))]
    #[case(Direction::West, (0, 0), (0, -1))]
    #[case(Direction::West, (1, 0), (1, -1))]
    #[case(Direction::West, (0, 1), (0, 0))]
    #[case(Direction::West, (1, 1), (1, 0))]
    fn test_step(#[case] direction: Direction, #[case] original: Coord, #[case] expected: Coord) {
        assert_eq!(direction.step(&original), expected);
    }

    #[test]
    fn test_parse_grid() {
        assert_eq!(parse_grid(TEST_GRID.lines()), *TEST_STATE);
    }

    #[test]
    fn test_walk() {
        let mut state = (*TEST_STATE).clone();
        walk(&mut state);
        assert_eq!(state.visited.len(), 41);
    }

}