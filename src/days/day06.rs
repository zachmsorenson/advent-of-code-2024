use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }
    }

    fn to_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    input: Vec<Vec<char>>,
    start: Position,
}

pub fn parse_input(input: &str) -> Input {
    let mut start = Position {
        x: 0,
        y: 0,
        direction: Direction::North,
    };
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match Direction::from_char(c) {
                Some(direction) => {
                    start.x = j as i32;
                    start.y = i as i32;
                    start.direction = direction;
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }

    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    input[start.y as usize][start.x as usize] = '.';
    Input { input, start }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut grid = input.input.clone();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // println!("input: {:?}", input);

    let mut sum = 0;
    let mut curr = input.start;
    loop {
        if grid[curr.y as usize][curr.x as usize] == '.' {
            sum += 1;
            grid[curr.y as usize][curr.x as usize] = 'X';
        }

        let step = Direction::step(&curr.direction);
        let next = (curr.x + step.0, curr.y + step.1);

        let c: Option<&char> = grid
            .get(next.1 as usize)
            .and_then(|row| row.get(next.0 as usize));

        match c {
            Some('#') => {
                curr.direction = Direction::turn(&curr.direction);
            }
            Some(_) => {
                (curr.x, curr.y) = (next.0, next.1);
            }
            None => {
                break;
            }
        }
    }

    Some(sum)
}

// Returns 1 if it finds a loop or 0 otherwise
pub fn backtrack(start: Position, grid: &[Vec<char>]) -> u64 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut curr = start;
    let mut turns = HashSet::new();

    let mut step = curr.direction.step();
    let mut next = (curr.x + step.0, curr.y + step.1);

    loop {
        while (0..num_rows).contains(&(next.1 as usize))
            && (0..num_cols).contains(&(next.0 as usize))
            && (grid[next.1 as usize][next.0 as usize] == '.'
                || grid[next.1 as usize][next.0 as usize] == 'X')
        {
            (curr.x, curr.y) = (next.0, next.1);
            (next.0, next.1) = (next.0 + step.0, next.1 + step.1);
        }

        let c: Option<&char> = grid
            .get(next.1 as usize)
            .and_then(|row| row.get(next.0 as usize));

        match c {
            Some('#') => {
                // println!("found obstacle @ (x,y)=({},{})", next.0, next.1);
                let new_turn = turns.insert((curr.x, curr.y, curr.direction.to_char()));
                if !new_turn {
                    return 1;
                }

                curr.direction = curr.direction.turn();
                step = curr.direction.step();
                next = (curr.x + step.0, curr.y + step.1);
            }
            None => {
                return 0;
            }
            Some(_) => {
                panic!("bad path");
            }
        }
    }
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut grid = input.input.clone();
    let num_rows = input.input.len();
    let num_cols = input.input[0].len();

    // Strategy: we don't care about putting down on obstacle unless it'd be
    // placed on the original path, so we can do one outer-walk through the original
    // path. Everytime we'd walk into an open space, begin a new sub-iteration as if
    // an obstacle is there. We can reduce the walk down to only tracking the turns
    // we need to take - any time we would make the same turn a 2nd time, we've made
    // a loop

    let mut sum = 0;
    let mut curr = input.start;
    loop {
        grid[curr.y as usize][curr.x as usize] = 'X';

        let step = Direction::step(&curr.direction);
        let next = (curr.x + step.0, curr.y + step.1);

        let c: Option<&char> = grid
            .get(next.1 as usize)
            .and_then(|row| row.get(next.0 as usize));

        match c {
            Some('#') => {
                curr.direction = Direction::turn(&curr.direction);
            }
            Some('.') => {
                // Backtrack pattern on subiteration here.
                // Do the sub iteration here.
                grid[next.1 as usize][next.0 as usize] = '#';
                sum += backtrack(curr, &grid);

                // Restore original state.
                grid[next.1 as usize][next.0 as usize] = '.';

                // Continue as usual
                (curr.x, curr.y) = (next.0, next.1);
            }
            Some('X') => {
                // We've already visited this square, not eligible for subiteration from this
                // direction
                // Continue as usual
                (curr.x, curr.y) = (next.0, next.1);
            }
            Some(_) => {
                panic!("bad path");
            }
            None => {
                break;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day06/test.txt");
    #[test]
    fn test_day06_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(41));
    }

    #[test]
    fn test_day06_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(6));
    }
}
