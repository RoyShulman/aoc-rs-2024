//! Solve day5
//!
//! Of course as per usual everything can fail, but here we know the input is valid
//! and it's just less code to deal with errors :)
use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
pub struct OrderingRule {
    before: u8,
    after: u8,
}

impl FromStr for OrderingRule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("|").fuse();
        let before = it.next().unwrap().parse().unwrap();
        let after = it.next().unwrap().parse().unwrap();
        Ok(Self { before, after })
    }
}

/// Ordering rules in a more efficient storage for faster lookup
///
/// Each rules consists of a `u8` number that must come before another `u8` number
/// For faster lookup we convert each rule into a single number by taking the before and multiplying by 256
/// This way we get a new `u16`. Now to check if a rule exists for 2 numbers we need to do the same conversion and check
/// if it exists in our rules by a simple index lookup
#[derive(Debug)]
pub struct OrderingRules {
    // can probably be more efficient with a bitvec but whatevs
    rules: [bool; u16::MAX as usize + 1],
}

impl OrderingRules {
    fn from_ordering_rules_slice(ordering_rules: &[OrderingRule]) -> Self {
        let mut rules = [false; u16::MAX as usize + 1];
        for rule in ordering_rules {
            rules[Self::convert_ordering_rule(&rule)] = true;
        }

        Self { rules }
    }

    fn convert_ordering_rule(rule: &OrderingRule) -> usize {
        rule.before as usize * 256 + rule.after as usize
    }

    fn exists(&self, rule: &OrderingRule) -> bool {
        let index = Self::convert_ordering_rule(rule);
        self.rules[index]
    }
}

impl From<Vec<OrderingRule>> for OrderingRules {
    fn from(value: Vec<OrderingRule>) -> Self {
        Self::from_ordering_rules_slice(&value)
    }
}

#[derive(Debug, Clone)]
pub struct PageUpdate {
    pages: Vec<u8>,
}

impl FromStr for PageUpdate {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s.split(",").map(|x| x.parse().unwrap()).collect();
        Ok(Self { pages })
    }
}

fn convert_update_to_right_order(update: PageUpdate, ordering_rules: &OrderingRules) -> PageUpdate {
    let mut current_update = update;
    while !current_update.is_in_right_order(ordering_rules) {
        let mut to_flip = None;
        for (i, before) in current_update.pages.iter().enumerate() {
            for (j, after) in current_update.pages.iter().skip(i).enumerate() {
                let rule = OrderingRule {
                    before: *after,
                    after: *before,
                };

                if ordering_rules.exists(&rule) {
                    // a rule exists, but the number show up incorrectly so we must flip them
                    to_flip = Some((i, j + i));
                }
            }
        }

        if let Some((i, j)) = to_flip {
            current_update.pages.swap(i, j);
        }
    }

    current_update
}

impl PageUpdate {
    fn is_in_right_order(&self, ordering_rules: &OrderingRules) -> bool {
        for (i, before) in self.pages.iter().enumerate() {
            for after in self.pages.iter().skip(i) {
                let opposite_rule = OrderingRule {
                    before: *after,
                    after: *before,
                };

                if ordering_rules.exists(&opposite_rule) {
                    return false;
                }
            }
        }

        true
    }

    fn middle(&self) -> u8 {
        self.pages[self.pages.len() / 2]
    }

    /// Assume we have an incorrect order and fix it. This clones the object
    fn convert_to_right_order(&self, ordering_rules: &OrderingRules) -> Self {
        let update = Self {
            pages: self.pages.clone(),
        };
        convert_update_to_right_order(update, ordering_rules)
    }
}

#[derive(Debug)]
pub struct SafetyManual {
    ordering_rules: OrderingRules,
    update: Vec<PageUpdate>,
}

impl FromStr for SafetyManual {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsing_ordering = true;
        let mut ordering_rules = Vec::new();
        let mut update = Vec::new();
        for line in s.lines() {
            if line == "" {
                parsing_ordering = false;
                continue;
            }
            if parsing_ordering {
                ordering_rules.push(line.parse().unwrap());
            } else {
                update.push(line.parse().unwrap());
            }
        }

        Ok(Self {
            ordering_rules: ordering_rules.into(),
            update,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let manual: SafetyManual = input.parse().unwrap();

    manual
        .update
        .iter()
        .filter(|update| update.is_in_right_order(&manual.ordering_rules))
        .map(|update| update.middle() as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let manual: SafetyManual = input.parse().unwrap();

    manual
        .update
        .iter()
        .filter(|update| !update.is_in_right_order(&manual.ordering_rules))
        .map(|update| {
            update
                .convert_to_right_order(&manual.ordering_rules)
                .middle() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
