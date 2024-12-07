use std::{collections::HashSet, convert::Infallible, fmt::Write, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapItem {
    Guard,
    Obstruction,
    Empty,
}

impl From<char> for MapItem {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstruction,
            '^' => Self::Guard,
            i => panic!("invalid map item: {i}"),
        }
    }
}

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapItem::Guard => f.write_char('^'),
            MapItem::Obstruction => f.write_char('#'),
            MapItem::Empty => f.write_char('.'),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[derive(Debug, Clone)]
struct Grid {
    // If we want to be more efficient, we can change this to be slice of slice
    // that way clones are cheap
    rows: Vec<Vec<MapItem>>,
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut guard_start_position = None;
        for (row_index, line) in s.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (column, c) in line.chars().enumerate() {
                let map_item: MapItem = c.into();
                if map_item == MapItem::Guard {
                    guard_start_position = Some((row_index, column));
                }
                row.push(map_item);
            }

            rows.push(row);
        }

        Ok(Self {
            rows,
            guard_position: guard_start_position.unwrap(),
            guard_direction: Direction::Up,
        })
    }
}

impl Grid {
    fn move_guard(&mut self) -> bool {
        let guard_next = match self.guard_direction {
            Direction::Left => {
                if self.guard_position.1 == 0 {
                    return false;
                }
                (self.guard_position.0, self.guard_position.1 - 1)
            }
            Direction::Right => {
                if self.guard_position.1 == self.rows[0].len() - 1 {
                    return false;
                }
                (self.guard_position.0, self.guard_position.1 + 1)
            }
            Direction::Up => {
                if self.guard_position.0 == 0 {
                    return false;
                }
                (self.guard_position.0 - 1, self.guard_position.1)
            }
            Direction::Down => {
                if self.guard_position.0 == self.rows.len() - 1 {
                    return false;
                }
                (self.guard_position.0 + 1, self.guard_position.1)
            }
        };
        let next_map_item = self.rows[guard_next.0][guard_next.1];
        match next_map_item {
            MapItem::Guard => panic!("impossible"),
            MapItem::Obstruction => {
                // turn 90 degress
                self.guard_direction = match self.guard_direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
            }
            MapItem::Empty => {
                self.rows[self.guard_position.0][self.guard_position.1] = MapItem::Empty;
                self.guard_position = guard_next;
                self.rows[self.guard_position.0][self.guard_position.1] = MapItem::Guard;
            }
        }

        true
    }
}

pub fn part1(input: &str) -> u32 {
    let mut grid: Grid = input.parse().unwrap();
    // yes we can be more efficient by having a custom "hash" function
    let mut locations = HashSet::new();
    locations.insert(grid.guard_position);
    while grid.move_guard() {
        locations.insert(grid.guard_position);
    }

    locations.len() as u32
}

fn try_find_single_loop(grid: &Grid, position: (usize, usize)) -> bool {
    let mut cloned_grid = grid.clone();
    // after looking at flamegraph it looks like most of the time is spent in
    // hash, so we try to be more efficient.
    // we know the max rows and columns and that there are 4 directions
    // a custom "hash" can be row * max_rows * max_columns + columns * max_columns + direction
    let num_rows = cloned_grid.rows.len();
    let num_columns = cloned_grid.rows[0].len();
    let num_directions = 4;
    // funny thing is that now most of the time is allocation, which we can also remove by only allocating
    // this once and just clearing it every run probably
    // but for now good enough because it finishes in like 2 seconds in debug and 1 in release
    // still not that fast but ok
    let mut current_locations =
        vec![false; ((num_rows + 1) * (num_columns + 1)) * (num_columns + 1) * num_directions];
    current_locations[cloned_grid.guard_position.0 * num_rows * num_columns
        + cloned_grid.guard_position.1 * num_columns
        + cloned_grid.guard_direction as usize] = true;
    cloned_grid.rows[position.0][position.1] = MapItem::Obstruction;

    while cloned_grid.move_guard() {
        if current_locations[cloned_grid.guard_position.0 * num_rows * num_columns
            + cloned_grid.guard_position.1 * num_columns
            + cloned_grid.guard_direction as usize]
        {
            // found a loop
            return true;
        }
        current_locations[cloned_grid.guard_position.0 * num_rows * num_columns
            + cloned_grid.guard_position.1 * num_columns
            + cloned_grid.guard_direction as usize] = true;
    }

    // exited without a loop
    false
}

pub fn part2(input: &str) -> u32 {
    let grid: Grid = input.parse().unwrap();
    // first we find all locations and direction the guard moved without adding objects
    let mut locations = HashSet::new();
    let initial_position = grid.guard_position;

    let mut cloned_grid = grid.clone();

    while cloned_grid.move_guard() {
        locations.insert(cloned_grid.guard_position);
    }

    let mut num_options = 0;
    // now we try to switch each position he walked at with an obstacle and see if we get a loop
    for position in locations {
        if position == initial_position {
            // can't place an obstacle in the intial position
            continue;
        }

        if try_find_single_loop(&grid, position) {
            num_options += 1;
        }
    }

    num_options
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."##;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
