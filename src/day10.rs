use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self, num_rows: usize, num_columns: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if self.x + 1 < num_rows {
            neighbors.push(Position {
                x: self.x + 1,
                y: self.y,
            })
        }

        if self.y + 1 < num_columns {
            neighbors.push(Position {
                x: self.x,
                y: self.y + 1,
            })
        }

        if self.x > 0 {
            neighbors.push(Position {
                x: self.x - 1,
                y: self.y,
            })
        }

        if self.y > 0 {
            neighbors.push(Position {
                x: self.x,
                y: self.y - 1,
            })
        }

        neighbors
    }
}

struct TrailMap {
    grid: Vec<Vec<u8>>,
    start_positions: Vec<Position>,
}

impl TrailMap {
    fn from_str(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut start_positions = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (x, c) in line.char_indices() {
                let digit = c.to_digit(10).unwrap() as u8;
                row.push(digit);

                if digit == 0 {
                    start_positions.push(Position { x, y });
                }
            }
            grid.push(row);
        }
        Self {
            grid,
            start_positions,
        }
    }
}

fn insert_possible_neighbors(stack: &mut Vec<Position>, current: Position, grid: &[Vec<u8>]) {
    let current_value = grid[current.y][current.x];
    for neighbor in current.neighbors(grid.len(), grid[0].len()) {
        let neighbor_value = grid[neighbor.y][neighbor.x];
        if current_value + 1 == neighbor_value {
            stack.push(neighbor);
        }
    }
}

/// Use dfs to find the number of trails.
///
/// A trail is a path that starts from 0, continues in one of 4 directions
/// increasing by exactly 1 each time and ends in a 9
/// Important to note the grid can be represented as a directed graph, as edges are only
/// from a node to a node with a value that is exactly 1 higher
fn find_num_trails(grid: &[Vec<u8>], position: Position) -> u32 {
    let mut stack = vec![position];
    let mut visited = HashSet::new();
    let mut num_trails = 0;
    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        let current_value = grid[current.y][current.x];
        visited.insert(current);
        if current_value == 9 {
            num_trails += 1;
            // no reason to continue exploring as 9 nodes are leafs
            continue;
        }
        insert_possible_neighbors(&mut stack, current, grid);
    }

    num_trails
}

/// This feels very not optimized...
fn get_trail_ratings(grid: &[Vec<u8>], position: Position) -> u32 {
    let mut stack = vec![position];
    let mut rating = 0;
    while let Some(current) = stack.pop() {
        let current_value = grid[current.y][current.x];
        if current_value == 9 {
            rating += 1;
            continue;
        }
        insert_possible_neighbors(&mut stack, current, grid);
    }

    rating
}

pub fn part1(input: &str) -> u32 {
    let trail_map = TrailMap::from_str(input);
    trail_map
        .start_positions
        .iter()
        .map(|start| find_num_trails(&trail_map.grid, *start))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let trail_map = TrailMap::from_str(input);
    trail_map
        .start_positions
        .iter()
        .map(|start| get_trail_ratings(&trail_map.grid, *start))
        .sum()
}

mod tests {
    use super::*;
    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 81);
    }
}
