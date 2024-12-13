use crate::util::{load_from, Errors};
use crate::Day;
use std::collections::{HashMap, HashSet};
use std::str::Lines;

pub struct Day06 {}

impl Day for Day06 {
    fn part_1(&self) -> Result<String, Errors> {
        let data = load_from("day06a.txt");
        let mut state = parse_grid(data.lines(), create_visited_set);
        state.walk();
        Ok(state.visited.len().to_string())
    }

    fn part_2(&self) -> Result<String, Errors> {
        let data = load_from("day06a.txt");
        let mut state = parse_grid(data.lines(), create_visited_map);
        state.walk();
        Ok(place_obstacles_and_walk(&state).to_string())
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(Day06 {})
    }
}


// (row, col)
type Coord = (isize, isize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct State<T> where T: Clone {
    original_pos: Coord,
    original_direction: Direction,
    position: Coord,
    direction: Direction,
    visited: T,
    obstacles: HashSet<Coord>,
    rows: usize,
    cols: usize
}

#[derive(Debug, PartialEq)]
enum ExitCondition {
    Loop,
    Grid
}

trait Recorder {
    /// Records a visit, returning true if the visit has been seen before (position and direction)
    ///
    /// If no direction is recorded, always returns false
    fn record_visit(&mut self, position: &Coord, direction: &Direction) -> bool;

    fn get_reset(&self) -> Self;
}

impl<T> State<T> where State<T>: Recorder, T: Clone {

    /// Walks around, ending when either:
    /// * a loop is detected (when the next move results in a location and position we've ended up in before), or
    /// * the guard moves out of the grid
    ///
    pub fn walk(&mut self) -> ExitCondition {
        loop {
            let proposed = self.direction.step(&self.position);
            // if we are in row or col -1, or beyond the last row or column (in row or col 130 in my input), we've left.
            if proposed.0 >= 0 && proposed.1 >= 0 && proposed.0 < self.rows as isize && proposed.1 < self.cols as isize {
                if self.obstacles.contains(&proposed) {
                    // discard proposed move, rotate 90 deg to right
                    self.direction = self.direction.rotate();
                } else {
                    // confirm move, insert step into tracking set
                    self.position = proposed;
                    let direction  = &self.direction.clone();
                    if self.record_visit(&proposed, direction) {
                        return ExitCondition::Loop;
                    }
                }
            } else {
                return ExitCondition::Grid;
            }
        }
    }
}

impl Recorder for State<HashSet<Coord>> {
    fn record_visit(&mut self, position: &Coord, _: &Direction) -> bool {
        self.visited.insert(position.clone());
        false
    }

    fn get_reset(&self) -> Self {
        State {
            position: self.original_pos.clone(),
            direction: self.original_direction.clone(),
            visited: HashSet::from([self.original_pos.clone()]),
            ..self.clone()
        }
    }
}

impl Recorder for State<HashMap<Coord, Vec<Direction>>> {
    fn record_visit(&mut self, position: &Coord, direction: &Direction) -> bool {
        if let Some(result) = self.visited.get_mut(position) {
            if result.contains(direction) {
                true
            } else {
                result.push(direction.clone());
                false
            }
        } else {
            self.visited.insert(position.clone(), vec![direction.clone()]);
            false
        }
    }

    fn get_reset(&self) -> Self {
        State {
            position: self.original_pos.clone(),
            direction: self.original_direction.clone(),
            visited: HashMap::from([(self.original_pos.clone(), vec![self.original_direction.clone()])]),
            ..self.clone()
        }
    }
}

fn place_obstacles_and_walk(original_state: &State<HashMap<Coord, Vec<Direction>>>) -> usize {
    // If we're only placing ONE obstacle, then it has to be somewhere on the original path.
    // So, with our original path, we place an item on each square and see what happens.
    // If we detect a loop, we count it.
    // We do not place an item on the first square
    let mut count: usize = 0;
    for (coord, direction) in original_state.visited.iter().filter(|(c, _)| **c != original_state.original_pos) {
        // create the obstacle.
        let mut new_state = original_state.get_reset();
        new_state.obstacles.insert(coord.clone());
        // We only walk from the point we first encounter this block.
        new_state.position = direction[0].back_one(coord);
        new_state.direction = direction[0].clone();
        if new_state.walk() == ExitCondition::Loop {
            count += 1;
        }
    }
    count
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

    fn back_one(&self, current: &Coord) -> Coord {
        match self {
            Direction::North => (current.0 + 1, current.1),
            Direction::East => (current.0, current.1 - 1),
            Direction::South => (current.0 - 1, current.1),
            Direction::West => (current.0, current.1 + 1)
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

fn create_visited_set(coord: &Coord) -> HashSet<Coord> {
    HashSet::from([coord.clone()])
}

fn create_visited_map(coord: &Coord) -> HashMap<Coord, Vec<Direction>> {
    HashMap::from([(coord.clone(), vec![Direction::North])])
}

fn parse_grid<T, F>(lines: Lines, create: F) -> State<T> where F: Fn(&Coord) -> T, T: Clone {
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
    let v = create(&position);
    State { original_pos: position.clone(), original_direction: Direction::North, position, direction: Direction::North, obstacles, visited: v, rows, cols }
}

#[cfg(test)]
mod tests {
    use crate::day06::{create_visited_map, create_visited_set, parse_grid, place_obstacles_and_walk, Coord, Direction, ExitCondition, Recorder, State};
    use lazy_static::lazy_static;
    use rstest::rstest;
    use std::collections::{HashMap, HashSet};

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
        static ref TEST_STATE: State<HashSet<Coord>> = State {
            position: (6, 4),
            original_pos: (6, 4),
            visited: HashSet::from([(6, 4)]),
            direction: Direction::North,
            original_direction: Direction::North,
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

        static ref TEST_DIRECTION_STATE: State<HashMap<Coord, Vec<Direction>>> = State {
            position: (6, 4),
            original_pos: (6, 4),
            visited: HashMap::from([((6, 4), vec![Direction::North])]),
            direction: Direction::North,
            original_direction: Direction::North,
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
    fn test_parse_grid_set() {
        assert_eq!(parse_grid(TEST_GRID.lines(), create_visited_set), *TEST_STATE);
    }

    #[test]
    fn test_parse_grid_map() {
        assert_eq!(parse_grid(TEST_GRID.lines(), create_visited_map), *TEST_DIRECTION_STATE);
    }

    #[test]
    fn test_walk() {
        let mut state = (*TEST_STATE).clone();
        assert_eq!(state.walk(), ExitCondition::Grid);
        assert_eq!(state.visited.len(), 41);
    }

    #[test]
    fn test_walk_map() {
        let mut state = (*TEST_DIRECTION_STATE).clone();
        state.walk();
        assert_eq!(state.walk(), ExitCondition::Grid);
        assert_eq!(state.visited.len(), 41);
    }

    #[test]
    fn test_walk_new_obs() {
        let mut state = (*TEST_DIRECTION_STATE).clone();
        state.walk();
        assert_eq!(state.walk(), ExitCondition::Grid);
        assert_eq!(state.visited.len(), 41);

        assert_eq!(place_obstacles_and_walk(&state), 6)
    }

}