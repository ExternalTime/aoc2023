static DIGIT_NAMES: [(&'static str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn leading_digit(str: &str) -> Option<u32> {
    str.chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .or_else(|| DIGIT_NAMES.iter()
            .find(|(text, _)| str.starts_with(text))
            .map(|(_, n)| *n))
}

pub struct Numbers<'a>(&'a str);

impl<'a> Iterator for Numbers<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut indices = self.0.char_indices();
        let res = indices.find_map(|(i, _)| leading_digit(&self.0[i..]));
        self.0 = indices.as_str();
        res
    }
}

fn calibration_value(line: &str) -> u32 {
    let mut nums = Numbers(line);
    let first = nums.next().unwrap_or(0);
    let last = nums.last().unwrap_or(first);
    10 * first + last
}

fn main() -> std::io::Result<()> {
    let res = std::io::read_to_string(std::io::stdin())?
        .lines()
        .map(calibration_value)
        .sum::<u32>();
    println!("result: {res}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_group(tests: &[(&str, u32)]) {
        for (text, c) in tests {
            assert_eq!(calibration_value(text), *c);
        }
    }
    
    #[test]
    fn first() {
        test_group(&[
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ])
    }

    #[test]
    fn second() {
        test_group(&[
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("twone", 21),
        ])
    }
}
