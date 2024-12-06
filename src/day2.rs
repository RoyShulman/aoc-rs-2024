use std::{collections::HashMap, str::FromStr};

fn maybe_skip_nth<T>(it: impl Iterator<Item = T>, skip: Option<usize>) -> impl Iterator<Item = T> {
    it.into_iter()
        .enumerate()
        .filter_map(move |(i, item)| match &skip {
            Some(skip) => {
                if i == *skip {
                    None
                } else {
                    Some(item)
                }
            }
            None => Some(item),
        })
}

pub struct Report {
    levels: Vec<u8>,
}

impl Report {
    fn from_line(line: &str) -> Self {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Self { levels }
    }

    fn is_safe(&self) -> bool {
        let mut asc = None;
        for (a, b) in self.levels.iter().zip(self.levels.iter().skip(1)) {
            match asc {
                Some(asc) => {
                    if asc {
                        if a < b {
                            return false;
                        }
                    } else {
                        if a > b {
                            return false;
                        }
                    }
                }
                None => {
                    if a > b {
                        asc = Some(true);
                    } else {
                        asc = Some(false);
                    }
                }
            }

            let diff = a.abs_diff(*b);
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }

    fn invalid_index(&self, skip: Option<usize>) -> Option<usize> {
        let mut asc = None;
        for (i, (a, b)) in maybe_skip_nth(self.levels.iter(), skip)
            .zip(maybe_skip_nth(
                self.levels.iter().skip(1),
                skip.map(|x| if x >= 1 { x - 1 } else { x }),
            ))
            .enumerate()
        {
            // eprint!("{}, ", a);
            match asc {
                Some(asc) => {
                    if asc {
                        if a < b {
                            return Some(i + 1);
                        }
                    } else {
                        if a > b {
                            return Some(i + 1);
                        }
                    }
                }
                None => {
                    if a > b {
                        asc = Some(true);
                    } else {
                        asc = Some(false);
                    }
                }
            }

            let diff = a.abs_diff(*b);
            if diff < 1 || diff > 3 {
                return Some(i + 1);
            }
        }

        None
    }

    fn is_safe_with_toleration(&self) -> bool {
        let mut counter = HashMap::new();
        for i in &self.levels {
            counter.entry(*i).and_modify(|x| *x += 1).or_insert(1);
        }
        for l in counter.values() {
            if *l > 2 {
                return false;
            }
        }

        let num_2 = counter.values().filter(|x| (**x == 2)).count();
        if num_2 > 1 {
            return false;
        }

        let invalid_index = self.invalid_index(None);
        let Some(index) = invalid_index else {
            return true;
        };

        if self.invalid_index(Some(index)).is_none() {
            return true;
        }

        if self.invalid_index(Some(0)).is_none() {
            return true;
        }

        if index >= 1 {
            if self.invalid_index(Some(index - 1)).is_none() {
                eprintln!("index:{} - {:?}", index, self.levels);
                return true;
            }
        }

        false
    }
}

pub fn part1(inputs: &str) -> u16 {
    let reports = inputs.lines().map(|line| Report::from_line(line));
    reports.into_iter().map(|r| r.is_safe() as u16).sum()
}

pub fn part2(inputs: &str) -> u16 {
    let reports = inputs.lines().map(|line| Report::from_line(line));
    reports
        .into_iter()
        .map(|r| r.is_safe_with_toleration() as u16)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUTS), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUTS), 4);
    }
}
