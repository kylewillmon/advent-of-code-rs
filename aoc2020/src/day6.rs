use std::str::FromStr;

use super::error::AocError;

struct CustomsForm {
    anyone: [bool; 26],
    everyone: [bool; 26],
}

impl CustomsForm {
    fn anyone_yes_count(&self) -> usize {
        self.anyone.iter().filter(|&&v| v).count()
    }

    fn everyone_yes_count(&self) -> usize {
        self.everyone.iter().filter(|&&v| v).count()
    }
}

impl FromStr for CustomsForm {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = CustomsForm {
            anyone: [false; 26],
            everyone: [true; 26],
        };

        for line in s.lines() {
            let mut person: [bool; 26] = [false; 26];

            for letter in line.bytes() {
                if letter < b'a' || letter > b'z' {
                    return Err(AocError::ParseError("invalid character in customs form".into()))
                }
                person[(letter-b'a') as usize] = true;
            }

            for (i, &ans) in person.iter().enumerate() {
                if ans {
                    c.anyone[i] = true;
                } else {
                    c.everyone[i] = false;
                }
            }
        }
        Ok(c)
    }
}

pub fn part1(input: String) -> Result<usize, AocError> {
    let mut total = 0;
    for entry in input.split("\n\n") {
        let cf = entry.parse::<CustomsForm>()?;
        total += cf.anyone_yes_count();
    }
    Ok(total)
}

pub fn part2(input: String) -> Result<usize, AocError> {
    let mut total = 0;
    for entry in input.split("\n\n") {
        let cf = entry.parse::<CustomsForm>()?;
        total += cf.everyone_yes_count();
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_example() {
        assert_eq!(Ok(11), part1(EXAMPLE.to_string()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(Ok(6), part2(EXAMPLE.to_string()));
    }
}