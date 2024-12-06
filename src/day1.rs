fn part1(input: &str) -> u32 {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace().fuse();
        let num1: u32 = parts.next().unwrap().parse().unwrap();
        let num2: u32 = parts.next().unwrap().parse().unwrap();
        first.push(num1);
        second.push(num2);
    }

    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| {
            if first > second {
                first - second
            } else {
                second - first
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace().fuse();
        let num1: u32 = parts.next().unwrap().parse().unwrap();
        let num2: u32 = parts.next().unwrap().parse().unwrap();
        first.push(num1);
        second.push(num2);
    }

    let max_second = *second.iter().max().unwrap() as usize;
    let mut occurences = vec![0; max_second + 1];
    for s in &second {
        occurences[*s as usize] += 1;
    }
    first
        .iter()
        .map(|f| *f * occurences.get(*f as usize).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3   4
        4   3
        2   5
        1   3
        3   9
        3   3"#;
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"3   4
        4   3
        2   5
        1   3
        3   9
        3   3"#;
        assert_eq!(part2(input), 31);
    }
}
