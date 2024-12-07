use itertools::{repeat_n, Itertools};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

pub struct Equation {
    result: u64,
    numbers: Vec<u16>,
}

impl FromStr for Equation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(": ").fuse();
        let result = it.next().unwrap().parse().unwrap();
        let numbers = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        Ok(Self { result, numbers })
    }
}

impl Equation {
    fn is_satisfiable(&self) -> bool {
        for it in (0..self.numbers.len() - 1)
            .map(|_| [Op::Add, Op::Mul])
            .multi_cartesian_product()
        {
            let result =
                it.iter()
                    .enumerate()
                    .fold(self.numbers[0] as u64, |acc, (i, op)| match op {
                        Op::Mul => acc as u64 * self.numbers[i + 1] as u64,
                        Op::Add => acc as u64 + self.numbers[i + 1] as u64,
                        Op::Concat => panic!("impossible"),
                    });
            if result == self.result {
                return true;
            }
        }

        false
    }

    fn is_satisfiable_with_concat(&self) -> bool {
        for it in (0..self.numbers.len() - 1)
            .map(|_| [Op::Add, Op::Mul, Op::Concat])
            .multi_cartesian_product()
        {
            let result =
                it.iter()
                    .enumerate()
                    .fold(self.numbers[0] as u64, |acc, (i, op)| match op {
                        Op::Mul => acc as u64 * self.numbers[i + 1] as u64,
                        Op::Add => acc as u64 + self.numbers[i + 1] as u64,
                        Op::Concat => {
                            let log10 = (self.numbers[i + 1] as f64).log10();
                            let mut num_digits = log10.ceil() as u64;

                            if log10 as u64 == num_digits {
                                num_digits += 1;
                            }

                            acc as u64 * 10_u64.pow((num_digits).try_into().unwrap())
                                + self.numbers[i + 1] as u64
                        }
                    });
            if result == self.result {
                return true;
            }
        }

        false
    }
}

pub fn part1(input: &str) -> u64 {
    let equations: Vec<Equation> = input.lines().map(|line| line.parse().unwrap()).collect();
    equations
        .iter()
        .filter(|eq| eq.is_satisfiable())
        .map(|x| x.result)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let equations: Vec<Equation> = input.lines().map(|line| line.parse().unwrap()).collect();
    equations
        .iter()
        .filter(|eq| eq.is_satisfiable_with_concat())
        .map(|x| x.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
