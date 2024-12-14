use std::collections::HashMap;

#[derive(Debug)]
struct Stones {
    stones: Vec<u64>,
}

fn get_num_digits(num: u64) -> u32 {
    // log10 rounded down will give the number of digits -1
    // for example:
    // log10(9) = 0
    // log10(10) = 1
    // log10(11) = 1
    num.ilog10() + 1
}

impl Stones {
    fn from_line(line: &str) -> Stones {
        let stones = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Stones { stones }
    }

    fn blink(&mut self) {
        let mut to_add = Vec::new();
        for stone in self.stones.iter_mut() {
            if stone == &0 {
                *stone = 1
            } else if get_num_digits(*stone) % 2 == 0 {
                // split into 2 stones
                let num_digits = get_num_digits(*stone);
                let left_digits = *stone / (10_u64.pow(num_digits / 2));
                let right_digits = *stone % (10_u64.pow(num_digits / 2));
                *stone = left_digits;
                to_add.push(right_digits);
            } else {
                *stone *= 2024;
            }
        }

        // order doesn't actually matter as it's not used anywhere
        for stone in to_add {
            self.stones.push(stone);
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut stones = Stones::from_line(input);
    for _ in 0..25 {
        stones.blink();
        eprintln!("{:?}", stones.stones);
    }
    stones.stones.len() as u64
}

/// Instead of simulating each stone individually, keep a frequency hashmap for each stone
/// and its number of occurrences. This way instead of simulating each stone, we can
/// simulate the stone once and update the result for all stones.
/// Thank you gpt for the hints on this one :)
fn blink_with_frequencies(mut frequencies: HashMap<u64, u64>, iterations: u8) -> u64 {
    for i in 0..iterations {
        eprintln!("{:?}", i);
        let mut new_frequencies = HashMap::new();
        for (num, count) in frequencies.iter() {
            if num == &0 {
                new_frequencies
                    .entry(1)
                    .and_modify(|x| *x += *count)
                    .or_insert(*count);
            } else if get_num_digits(*num) % 2 == 0 {
                let num_digits = get_num_digits(*num);
                let left_digits = *num / (10_u64.pow(num_digits / 2));
                let right_digits = *num % (10_u64.pow(num_digits / 2));

                new_frequencies
                    .entry(left_digits)
                    .and_modify(|x| *x += *count)
                    .or_insert(*count);

                new_frequencies
                    .entry(right_digits)
                    .and_modify(|x| *x += *count)
                    .or_insert(*count);
            } else {
                new_frequencies
                    .entry(*num * 2024)
                    .and_modify(|x| *x += *count)
                    .or_insert(*count);
            }
        }
        frequencies = new_frequencies;
    }
    frequencies.values().sum()
}

pub fn part2(input: &str) -> u64 {
    let mut stones = Stones::from_line(input);
    let mut frequencies = HashMap::new();
    for stone in stones.stones {
        *frequencies.entry(stone).or_insert(0) += 1;
    }

    blink_with_frequencies(frequencies, 75)
}

mod tests {
    use super::*;
    const INPUT: &str = r#"125 17"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 55312);
    }
}
