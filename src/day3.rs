use std::{slice, str::Chars};

pub struct InstructionParser<'a> {
    input: &'a [u8],
    cursor: usize,
    is_enabled: bool,
    parse_enabled: bool,
}

/// Try to parse a number consisting of 1 to 3 digits
///
/// Returns None if the less than 1 or more than 3 digits, or if we reach the end of the string
/// Otherwise we return the number and the index of of the updated cursor
fn parse_1_to_3_digits(input: &[u8], cursor: usize) -> Option<(u16, usize)> {
    let digit1 = input.get(cursor)?;
    if !digit1.is_ascii_digit() {
        return None;
    }

    let digit2 = input.get(cursor + 1)?;
    if !digit2.is_ascii_digit() {
        return Some(((digit1 - b'0').into(), cursor + 1));
    }

    let digit3 = input.get(cursor + 2)?;
    if !digit3.is_ascii_digit() {
        let digit1 = (digit1 - b'0') as u16;
        let digit2 = (digit2 - b'0') as u16;
        return Some((digit1 * 10 + digit2, cursor + 2));
    }

    let digit4 = input.get(cursor + 3)?;

    if !digit4.is_ascii_digit() {
        let digit1 = (digit1 - b'0') as u16;
        let digit2 = (digit2 - b'0') as u16;
        let digit3 = (digit3 - b'0') as u16;
        return Some((digit1 * 100 + digit2 * 10 + digit3, cursor + 3));
    }

    None
}

impl<'a> InstructionParser<'a> {
    pub fn new(input: &'a str, parse_enabled: bool) -> Self {
        debug_assert!(input.is_ascii());
        Self {
            input: input.as_bytes(),
            cursor: 0,
            is_enabled: true,
            parse_enabled,
        }
    }

    fn get_do_instruction(&self) -> Option<bool> {
        let d = self.input.get(self.cursor)?;
        let o = self.input.get(self.cursor + 1)?;
        let first = self.input.get(self.cursor + 2)?;
        let second = self.input.get(self.cursor + 3)?;
        match (d, o, first, second) {
            (b'd', b'o', b'(', b')') => return Some(true),
            _ => return Some(false),
        }
    }

    fn get_dont_instruction(&self) -> Option<bool> {
        let d = self.input.get(self.cursor)?;
        let o = self.input.get(self.cursor + 1)?;
        let n = self.input.get(self.cursor + 2)?;
        let ti = self.input.get(self.cursor + 3)?;
        let t = self.input.get(self.cursor + 4)?;
        let first = self.input.get(self.cursor + 5)?;
        let second = self.input.get(self.cursor + 6)?;
        match (d, o, n, ti, t, first, second) {
            (b'd', b'o', b'n', b'\'', b't', b'(', b')') => return Some(true),
            _ => return Some(false),
        }
    }

    fn try_get_single_instruction(&mut self) -> Option<(u32, usize)> {
        let fast_cursor = self.cursor;
        let m = self.input.get(fast_cursor)?;
        let u = self.input.get(fast_cursor + 1)?;
        let l = self.input.get(fast_cursor + 2)?;
        match (m, u, l) {
            (b'm', b'u', b'l') => (),
            _ => return None,
        };

        let first_sep = self.input.get(fast_cursor + 3)?;
        if *first_sep != b'(' {
            return None;
        }

        // now we expect 1 to 3 digits
        let (digit1, fast_cursor) = parse_1_to_3_digits(&self.input, fast_cursor + 4)?;
        let seperator = self.input.get(fast_cursor)?;
        if *seperator != b',' {
            return None;
        }

        let (digit2, fast_cursor) = parse_1_to_3_digits(&self.input, fast_cursor + 1)?;
        let final_sep = self.input.get(fast_cursor)?;
        if *final_sep != b')' {
            return None;
        }

        return Some((digit1 as u32 * digit2 as u32, fast_cursor + 1));
    }
}

impl<'a> Iterator for InstructionParser<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.input.get(self.cursor).is_some() {
            let do_inst = self.get_do_instruction();
            let dont_inst = self.get_dont_instruction();

            match (do_inst, dont_inst) {
                // Some and None don't matter because we can't have other instruction later anyways
                (Some(do_inst), Some(dont_inst)) => {
                    // do starts like dont, so only if we have dont = false and do = true we should enable
                    if do_inst && !dont_inst {
                        self.is_enabled = true
                    } else if dont_inst {
                        self.is_enabled = false;
                    }
                }
                _ => (),
            }

            if let Some((result, new_cursor)) = self.try_get_single_instruction() {
                self.cursor = new_cursor;
                // parse enabled is part2
                if !self.parse_enabled || self.is_enabled {
                    return Some(result);
                } else {
                    continue;
                }
            }
            self.cursor += 1;
        }

        None
    }
}

pub fn part1(input: &str) -> u32 {
    let parser = InstructionParser::new(input, false);
    parser.into_iter().sum()
}

pub fn part2(input: &str) -> u32 {
    let parser = InstructionParser::new(input, true);
    parser.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))s"#;
        assert_eq!(part2(input), 48);
    }
}
