use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use itertools::Itertools;

// Yeah, the name isn't that great because we use it for both antenas and antinodes
// pretty much this is just a 2d location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    row: u8,
    column: u8,
}

fn get_num_with_inc_until_max(num: i16, max: i16, diff: i16) -> Vec<i16> {
    let mut num = num;
    let mut nums = vec![];
    while num < max {
        nums.push(num);
        num += diff;
    }

    nums
}

fn get_num_with_dec_until_min(num: i16, min: i16, diff: i16) -> Vec<i16> {
    let mut num = num;
    let mut nums = vec![];
    while num >= min {
        nums.push(num);
        num -= diff;
    }

    nums
}

impl Antenna {
    fn get_antinodes(
        &self,
        other: &Self,
        num_rows: u8,
        num_columns: u8,
    ) -> (Option<Antenna>, Option<Antenna>) {
        let (self_row, self_column) = (self.row as i16, self.column as i16);
        let (other_row, other_column) = (other.row as i16, other.column as i16);

        // We now want to reduce the difference of rows from the antena that has a lower
        // row number, and add the diff to the greater row number

        // should always fit in i16 because we are dealing with a grid of max u8 columns and rows
        let row_diff = self_row.abs_diff(other_row) as i16;
        let (self_row, other_row) = if self_row > other_row {
            (self_row + row_diff, other_row - row_diff)
        } else {
            (self_row - row_diff, other_row + row_diff)
        };

        let column_diff = self_column.abs_diff(other_column) as i16;
        let (self_column, other_column) = if self_column > other_column {
            (self_column + column_diff, other_column - column_diff)
        } else {
            (self_column - column_diff, other_column + column_diff)
        };

        // now make sure we are in valid bounds
        let a1 = if self_row < num_rows as i16
            && self_row >= 0
            && self_column >= 0
            && self_column < num_columns as i16
        {
            Some(Antenna {
                row: self_row as u8,
                column: self_column as u8,
            })
        } else {
            None
        };

        let a2 = if other_row < num_rows as i16
            && other_row >= 0
            && other_column >= 0
            && other_column < num_columns as i16
        {
            Some(Antenna {
                row: other_row as u8,
                column: other_column as u8,
            })
        } else {
            None
        };

        (a1, a2)
    }

    fn get_antinodes_any_distance(
        &self,
        other: &Self,
        num_rows: u8,
        num_columns: u8,
    ) -> Vec<Antenna> {
        let (self_row, self_column) = (self.row as i16, self.column as i16);
        let (other_row, other_column) = (other.row as i16, other.column as i16);

        // should always fit in i16 because we are dealing with a grid of max u8 columns and rows
        let row_diff = self_row.abs_diff(other_row) as i16;
        let (self_rows, other_rows) = if self_row > other_row {
            let self_rows = get_num_with_inc_until_max(self_row, num_rows as i16, row_diff);
            let other_rows = get_num_with_dec_until_min(other_row, 0, row_diff);
            (self_rows, other_rows)
        } else {
            let other_rows = get_num_with_inc_until_max(other_row, num_rows as i16, row_diff);
            let self_rows = get_num_with_dec_until_min(self_row, 0, row_diff);
            (self_rows, other_rows)
        };

        let column_diff = self_column.abs_diff(other_column) as i16;
        let (self_columns, other_columns) = if self_column > other_column {
            let self_columns =
                get_num_with_inc_until_max(self_column, num_columns as i16, column_diff);
            let other_columns = get_num_with_dec_until_min(other_column, 0, column_diff);
            (self_columns, other_columns)
        } else {
            let other_columns =
                get_num_with_inc_until_max(other_column, num_columns as i16, column_diff);
            let self_columns = get_num_with_dec_until_min(self_column, 0, column_diff);
            (self_columns, other_columns)
        };

        self_rows
            .into_iter()
            .zip(self_columns)
            .chain(other_rows.into_iter().zip(other_columns))
            .map(|(row, column)| Antenna {
                row: row as u8,
                column: column as u8,
            })
            .collect()
    }
}

struct AntennaGrid {
    num_rows: u8,
    num_columns: u8,
    antenas: HashMap<char, Vec<Antenna>>,
}

impl AntennaGrid {
    fn find_antinodes(&self) -> HashSet<Antenna> {
        // Iterate over all antenas in pairs and find their absolute difference
        let mut locations = HashSet::new();
        for signal_antenas in self.antenas.values() {
            for (a1, a2) in signal_antenas.iter().tuple_combinations() {
                let (anti1, anti2) = a1.get_antinodes(a2, self.num_rows, self.num_columns);
                if let Some(anti1) = anti1 {
                    locations.insert(anti1);
                }

                if let Some(anti2) = anti2 {
                    locations.insert(anti2);
                }
            }
        }

        locations
    }

    fn find_antinodes_any_distance(&self) -> HashSet<Antenna> {
        let mut locations = HashSet::new();
        for signal_antenas in self.antenas.values() {
            for (a1, a2) in signal_antenas.iter().tuple_combinations() {
                let antenas = a1.get_antinodes_any_distance(a2, self.num_rows, self.num_columns);
                locations.extend(antenas.into_iter());
            }
        }

        locations
    }
}

impl FromStr for AntennaGrid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_rows = 0;
        let mut num_columns = 0;
        let mut antenas = HashMap::new();
        for row in s.lines() {
            num_columns = 0;
            for c in row.chars() {
                if c != '.' {
                    let antena = Antenna {
                        row: num_rows,
                        column: num_columns,
                    };
                    antenas
                        .entry(c)
                        .and_modify(|e: &mut Vec<Antenna>| e.push(antena))
                        .or_insert_with(|| vec![antena]);
                }
                num_columns += 1;
            }
            num_rows += 1;
        }

        Ok(Self {
            num_rows,
            num_columns,
            antenas,
        })
    }
}

pub fn part1(input: &str) -> usize {
    let grid: AntennaGrid = input.parse().unwrap();
    let antinodes = grid.find_antinodes();
    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let grid: AntennaGrid = input.parse().unwrap();
    let antinodes = grid.find_antinodes_any_distance();
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }
}
