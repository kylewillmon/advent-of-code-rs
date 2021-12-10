use std::collections::HashSet;
use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use aoclib::strtools;
use itertools::Itertools;

pub fn part1(input: String) -> Result<usize> {
    let (fields, _, tickets) = parse_input(input)?;

    let mut error_rate = 0;
    for ticket in tickets {
        error_rate += ticket.error_rate(&fields).unwrap_or(0);
    }
    Ok(error_rate)
}

pub fn part2(input: String) -> Result<usize> {
    let (fields, mine, nearby) = parse_input(input)?;

    let indexes = solve_fields(&fields, &nearby)?;

    let vals = mine
        .0
        .into_iter()
        .enumerate()
        .map(|(i, val)| (fields[indexes[i]].name.as_str(), val))
        .filter(|&(name, _val)| name.starts_with("departure"))
        .map(|(_name, val)| val);

    Ok(vals.product())
}

fn solve_single_field(options: &[HashSet<usize>]) -> Option<(usize, usize)> {
    for (col, options) in options.iter().enumerate() {
        if let Ok(field) = options.iter().exactly_one() {
            return Some((col, *field));
        }
    }

    for field in 0..options.len() {
        let cols = options
            .iter()
            .enumerate()
            .filter(|&(_col, options)| options.contains(&field))
            .map(|(col, _options)| col);

        if let Ok(col) = cols.exactly_one() {
            return Some((col, field));
        }
    }
    None
}

fn solve_fields(fields: &[Field], tickets: &[Ticket]) -> Result<Vec<usize>> {
    let tickets: Vec<Ticket> = tickets
        .iter()
        .cloned()
        .filter(|t| t.error_rate(fields).is_none())
        .collect();

    let mut options: Vec<HashSet<usize>> = vec![(0..fields.len()).collect(); fields.len()];

    for t in tickets {
        for (num, options) in t.0.into_iter().zip_eq(options.iter_mut()) {
            for (idx, field) in fields.iter().enumerate() {
                if options.contains(&idx) && !field.is_valid(num) {
                    options.remove(&idx);
                }
            }
        }
    }

    let mut result: Vec<usize> = vec![usize::MAX; fields.len()];

    while let Some((col, field)) = solve_single_field(&options) {
        result[col] = field;
        options[col].clear();
        for options in options.iter_mut() {
            options.remove(&field);
        }
    }

    if result.iter().any(|&field| field == usize::MAX) {
        return Err(anyhow!("no solution for given fields"));
    }
    Ok(result)
}

fn parse_input(input: String) -> Result<(Vec<Field>, Ticket, Vec<Ticket>)> {
    let (fields, tickets) = strtools::split_once(input.trim(), "\n\n");

    let fields: Vec<Field> = fields
        .lines()
        .map(Field::from_line)
        .collect::<Result<_, _>>()?;

    let (mine, nearby) = strtools::split_once(tickets, "\n\n");
    let mine = Ticket::from_line(
        mine.lines()
            .skip(1)
            .exactly_one()
            .map_err(|_| anyhow!("too many lines for my ticket"))?,
    )?;
    let nearby: Vec<Ticket> = nearby
        .lines()
        .skip(1)
        .map(|n| Ticket::from_line(n.trim()))
        .collect::<Result<_, _>>()?;

    Ok((fields, mine, nearby))
}

#[derive(Debug, Clone)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn from_line(line: &str) -> Result<Self> {
        Ok(Self(
            line.split(',')
                .map(|n| n.trim().parse::<usize>())
                .collect::<Result<_, _>>()?,
        ))
    }

    fn error_rate(&self, fields: &[Field]) -> Option<usize> {
        let mut error_rate = None;
        for num in self.0.iter() {
            if !fields.iter().any(|f| f.is_valid(*num)) {
                error_rate = Some(*num + error_rate.unwrap_or(0));
            }
        }
        error_rate
    }
}

#[derive(Debug, Clone)]
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

        Ok(Self {
            name: name.to_string(),
            low: parse_range(low)?,
            high: parse_range(high)?,
        })
    }

    fn is_valid(&self, num: usize) -> bool {
        self.low.contains(&num) || self.high.contains(&num)
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

    #[test]
    fn part2_example() {
        let (fields, _, tickets) = parse_input(EXAMPLE.to_string()).unwrap();

        assert_eq!(vec![1, 0, 2], solve_fields(&fields, &tickets).unwrap());
    }
}
