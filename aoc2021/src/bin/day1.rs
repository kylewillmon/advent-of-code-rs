use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{self, Read};

use nom::character::complete::{line_ending, u64};
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        return Err(anyhow!("Invalid number of arguments: {}", args.len()));
    }

    let input = get_input(args.nth(1).unwrap()).unwrap();

    let nums = parse_nums(&input)?;

    let mut latest = nums.get(0).cloned().unwrap_or(0u64);
    let mut count = 0;

    for i in nums.iter().cloned().skip(1) {
        if i > latest {
            count += 1;
        }
        latest = i;
    }

    println!("Part 1 Count: {}", count);

    let mut latest: VecDeque<u64> = nums.get(..=2).unwrap_or(&[]).iter().cloned().collect();
    count = 0;

    for i in nums.iter().cloned().skip(3) {
        if i > latest.pop_front().unwrap() {
            count += 1;
        }
        latest.push_back(i);
    }

    println!("Part 2 Count: {}", count);

    Ok(())
}

fn parse_nums(s: &str) -> Result<Vec<u64>> {
    fn inner(s: &str) -> IResult<&str, Vec<u64>> {
        terminated(
            separated_list1(line_ending, u64),
            preceded(many0(line_ending), eof),
        )(s)
    }
    match inner(s) {
        Ok((_s, v)) => Ok(v),
        Err(e) => Err(e.to_owned().into()),
    }
}

fn get_input<P: AsRef<str>>(filename: P) -> io::Result<String> {
    if filename.as_ref() == "-" {
        let mut data = String::new();
        return io::stdin().read_to_string(&mut data).map(move |_| data);
    }
    fs::read_to_string(filename.as_ref())
}
