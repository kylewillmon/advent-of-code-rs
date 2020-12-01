use std::collections::hash_set;
use itertools::Itertools;

use super::parse;

pub fn part1(input: String) -> u32
{
    let nums: hash_set::HashSet<_> = parse::to_nums(input).into_iter().collect();

    let mut iter = nums.iter().cloned();
    while let Some(i) = iter.next() {
        let other = 2020 - i;
        if nums.contains(&other) {
            return i * other;
        }
    }
    panic!("No two items add up to 2020!");
}


pub fn part2(input: String) -> u32
{
    let nums = parse::to_nums(input);

    for (a, b, c) in nums.into_iter().tuple_combinations() {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }
    panic!("No triple adds up to 2020!");
}
