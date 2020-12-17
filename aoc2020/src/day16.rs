use std::ops::RangeInclusive;

use anyhow::Result;
use aoclib::strtools;

pub fn part1(input: String) -> Result<usize> {
    let (fields, tickets) = strtools::split_once(input.trim(), "\n\n");

    let fields: Vec<Field> = fields.lines()
        .map(|l| Field::from_line(l))
        .collect::<Result<_, _>>()?;

    let (_, tickets) = strtools::split_once(tickets, "\n\n");
    let nums: Vec<usize> = tickets
        .lines().skip(1)
        .flat_map(|l| l.split(','))
        .map(|n| n.trim().parse::<usize>())
        .collect::<Result<_, _>>()?;

    let mut error_rate = 0;
    for num in nums {
        if !fields.iter().any(|f| f.is_valid(num)) {
            error_rate += num;
        }
    }
    Ok(error_rate)
}

pub fn part2(_input: String) -> Result<usize> {
    Ok(0)
}

struct Field {
    name: String,
    low: RangeInclusive<usize>,
    high: RangeInclusive<usize>,
}

fn parse_range(r: &str) -> Result<RangeInclusive<usize>> {
    let (start, end) = strtools::split_once(r, "-");
    let start = start.trim().parse::<usize>()?;
    let end = end.trim().parse::<usize>()?;
    Ok(start..=end)
}

impl Field {
    fn from_line(line: &str) -> Result<Self> {
        let (name, ranges) = strtools::split_once(line, ": ");
        let (low, high) = strtools::split_once(ranges, " or ");

        println!("name={}, low={}, high={}", name, low, high);

        Ok(Self {
            name: name.to_string(),
            low: parse_range(low)?,
            high: parse_range(high)?,
        })
    }

    fn is_valid(&self, num: usize) -> bool {
        self.low.contains(&num)
        || self.high.contains(&num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn part1_example() {
        assert_eq!(71, part1(EXAMPLE.to_string()).unwrap());
    }
}
