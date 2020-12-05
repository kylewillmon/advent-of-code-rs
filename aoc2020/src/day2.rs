use std::convert::Infallible;
use std::str::FromStr;

pub fn part1(input: String) -> Result<usize, Infallible>
{
    let res = parse_input(input)
        .into_iter()
        .filter(|(pol, pass)| pol.check(pass))
        .count();
    Ok(res)
}

pub fn part2(input: String) -> Result<usize, Infallible>
{
    let res = parse_input(input)
        .into_iter()
        .filter(|(pol, pass)| pol.check2(pass))
        .count();
    Ok(res)
}

struct PasswordPolicy {
    min: u16,
    max: u16,
    letter: char,
}

fn has_char_at<T: AsRef<str>>(s: T, c: char, idx: usize) -> bool
{
    s
        .as_ref()
        .chars()
        .nth(idx)
        .map(|val| val == c)
        .unwrap_or(false)
}

impl PasswordPolicy {
    fn check<T: AsRef<str>>(&self, password: T) -> bool
    {
        let num = password
            .as_ref()
            .chars()
            .filter(|&c| c == self.letter)
            .count() as u16;

        self.min <= num && num <= self.max
    }

    fn check2<T: Copy + AsRef<str>>(&self, password: T) -> bool
    {
        let first = has_char_at(password, self.letter, (self.min-1).into());
        let second = has_char_at(password, self.letter, (self.max-1).into());
        first ^ second
    }
}

impl FromStr for PasswordPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts : Vec<&str> = s.splitn(2, " ").collect();
        let times : Vec<&str> = parts[0].splitn(2, "-").collect();
        let letter = parts[1].chars().nth(0).ok_or("no letter provided")?;
        let min = times[0].parse::<u16>().map_err(|e| format!("{}", e))?;
        let max = times[1].parse::<u16>().map_err(|e| format!("{}", e))?;

        Ok(Self {min, max, letter})
    }
}

fn parse_input(input: String) -> Vec<(PasswordPolicy, String)>
{
    input
        .lines()
        .map(|l| {
            let parts : Vec<&str> = l.splitn(2, ": ").collect();
            let pol = parts[0].parse::<PasswordPolicy>().unwrap();
            let pass = parts[1];
            (pol, pass.to_string())
        })
        .collect()
}
