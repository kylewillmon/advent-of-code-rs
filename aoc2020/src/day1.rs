use itertools::Itertools;
use std::collections::hash_set;
use std::convert::Infallible;

use super::parse;

pub fn part1(input: String) -> Result<u32, Infallible> {
    let nums: hash_set::HashSet<_> = parse::to_nums(input).into_iter().collect();

    for i in nums.iter().cloned() {
        let other = 2020 - i;
        if nums.contains(&other) {
            return Ok(i * other);
        }
    }
    panic!("No two items add up to 2020!");
}

pub fn part2(input: String) -> Result<u32, Infallible> {
    let nums = parse::to_nums(input);

    for (a, b, c) in nums.into_iter().tuple_combinations() {
        if a + b + c == 2020 {
            return Ok(a * b * c);
        }
    }
    panic!("No triple adds up to 2020!");
}
