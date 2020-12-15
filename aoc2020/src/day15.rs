use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: String) -> Result<usize> {
    let mut nums: Vec<usize> = input.trim()
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<_, _>>()?;

    while nums.len() < 2020 {
        let last = *nums.last().unwrap();
        let pos = nums.iter().rev()
            .skip(1)
            .position(|&n| n == last);

        let next = match pos {
            Some(val) => val + 1,
            None => 0,
        };
        nums.push(next);
    }

    Ok(*nums.last().unwrap())
}

pub fn part2(input: String) -> Result<usize> {
    let mut init: Vec<usize> = input.trim()
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<_, _>>()?;

    let mut position_map = HashMap::new();
    let mut last = init.pop().unwrap();

    for (idx, num) in init.iter().enumerate() {
        position_map.insert(*num, idx);
    }

    for position in init.len()..29_999_999 {
        let last_pos = position_map.get(&last);

        let next = match last_pos {
            Some(last_pos) => position - last_pos,
            None => 0,
        };

        position_map.insert(last, position);
        last = next;
    }

    Ok(last)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "0,3,6";
    const EXAMPLE2: &str = "1,3,2";
    const EXAMPLE3: &str = "2,1,3";
    const EXAMPLE4: &str = "1,2,3";
    const EXAMPLE5: &str = "2,3,1";
    const EXAMPLE6: &str = "3,2,1";
    const EXAMPLE7: &str = "3,1,2";

    #[test]
    fn part1_examples() {
        assert_eq!(436,  part1(EXAMPLE1.to_string()).unwrap());
        assert_eq!(1,    part1(EXAMPLE2.to_string()).unwrap());
        assert_eq!(10,   part1(EXAMPLE3.to_string()).unwrap());
        assert_eq!(27,   part1(EXAMPLE4.to_string()).unwrap());
        assert_eq!(78,   part1(EXAMPLE5.to_string()).unwrap());
        assert_eq!(438,  part1(EXAMPLE6.to_string()).unwrap());
        assert_eq!(1836, part1(EXAMPLE7.to_string()).unwrap());
    }

    #[test]
    #[ignore]
    fn part2_examples() {
        assert_eq!(175594,  part2(EXAMPLE1.to_string()).unwrap());
        assert_eq!(2578,    part2(EXAMPLE2.to_string()).unwrap());
        assert_eq!(3544142, part2(EXAMPLE3.to_string()).unwrap());
        assert_eq!(261214,  part2(EXAMPLE4.to_string()).unwrap());
        assert_eq!(6895259, part2(EXAMPLE5.to_string()).unwrap());
        assert_eq!(18,      part2(EXAMPLE6.to_string()).unwrap());
        assert_eq!(362,     part2(EXAMPLE7.to_string()).unwrap());
    }
}
