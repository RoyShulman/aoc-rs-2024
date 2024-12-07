/// Always assume the string is valid ascii
pub fn part1(input: &str) -> u32 {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.as_bytes());
    }

    let mut sum = 0;
    for line in &grid {
        for idx in 0..line.len() - 3 {
            if &line[idx..idx + 4] == b"XMAS" || &line[idx..idx + 4] == b"SAMX" {
                sum += 1;
            }
        }
    }

    let num_rows = grid.len();
    let num_columns = grid[0].len();

    for row in 0..num_rows {
        for column in 0..num_columns {
            if row + 3 < num_rows {
                if (grid[row][column] == b'X'
                    && grid[row + 1][column] == b'M'
                    && grid[row + 2][column] == b'A'
                    && grid[row + 3][column] == b'S')
                    || (grid[row][column] == b'S'
                        && grid[row + 1][column] == b'A'
                        && grid[row + 2][column] == b'M'
                        && grid[row + 3][column] == b'X')
                {
                    sum += 1;
                }
            }
        }
    }

    for row in 0..num_rows - 3 {
        for column in 0..num_columns - 3 {
            if (grid[row][column] == b'X'
                && grid[row + 1][column + 1] == b'M'
                && grid[row + 2][column + 2] == b'A'
                && grid[row + 3][column + 3] == b'S')
                || (grid[row][column] == b'S'
                    && grid[row + 1][column + 1] == b'A'
                    && grid[row + 2][column + 2] == b'M'
                    && grid[row + 3][column + 3] == b'X')
            {
                sum += 1;
            }
        }
    }

    for row in (3..num_rows).rev() {
        for column in 0..num_columns - 3 {
            if (grid[row][column] == b'X'
                && grid[row - 1][column + 1] == b'M'
                && grid[row - 2][column + 2] == b'A'
                && grid[row - 3][column + 3] == b'S')
                || (grid[row][column] == b'S'
                    && grid[row - 1][column + 1] == b'A'
                    && grid[row - 2][column + 2] == b'M'
                    && grid[row - 3][column + 3] == b'X')
            {
                sum += 1;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.as_bytes());
    }

    let num_rows = grid.len();
    let num_columns = grid[0].len();

    let mut sum = 0;
    for row in 1..num_rows - 1 {
        for column in 1..num_columns - 1 {
            if grid[row][column] != b'A' {
                continue;
            }

            if (grid[row - 1][column - 1] == b'M'
                && grid[row + 1][column - 1] == b'M'
                && grid[row + 1][column + 1] == b'S'
                && grid[row - 1][column + 1] == b'S')
                || (grid[row - 1][column - 1] == b'S'
                    && grid[row + 1][column - 1] == b'M'
                    && grid[row + 1][column + 1] == b'M'
                    && grid[row - 1][column + 1] == b'S')
                || (grid[row - 1][column - 1] == b'S'
                    && grid[row + 1][column - 1] == b'S'
                    && grid[row + 1][column + 1] == b'M'
                    && grid[row - 1][column + 1] == b'M')
                || (grid[row - 1][column - 1] == b'M'
                    && grid[row + 1][column - 1] == b'S'
                    && grid[row + 1][column + 1] == b'S'
                    && grid[row - 1][column + 1] == b'M')
            {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9);
    }
}
