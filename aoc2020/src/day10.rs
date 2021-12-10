use std::convert::TryFrom;

use anyhow::Result;

use super::parse::to_nums;

pub fn part1(input: String) -> Result<usize> {
    let adapters = {
        let mut a = to_nums(input);
        a.sort_unstable();
        a
    };

    let mut joltage = 0;
    let mut ones = 0;
    let mut threes = 0;
    for adapter in adapters {
        let difference = adapter - joltage;
        if difference == 1 {
            ones += 1;
        } else if difference == 3 {
            threes += 1;
        }
        joltage = adapter;
    }

    // The final difference to device joltage is always three
    threes += 1;

    Ok(ones * threes)
}

pub fn part2(input: String) -> Result<usize> {
    let adapters = {
        let mut a = to_nums(input);
        a.sort_unstable();
        a.reverse();

        a.push(0);
        a
    };

    // Number of ways to connect to joltages: n, n+1, n+2, and n+3
    let mut ways: [usize; 4] = [1, 0, 0, 0];

    // Start with device joltage
    let mut joltage = adapters[0] + 3;

    for adapter in adapters {
        let difference: usize = usize::try_from(joltage - adapter)?;

        ways.rotate_right(difference);
        for i in 0..difference { ways[i] = 0; }

        let new_ways = ways.iter().sum();
        ways[0] = new_ways;

        joltage = adapter;
    }
    Ok(ways[0])
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const LARGE_EXAMPLE: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part1_examples() {
        assert_eq!(35, part1(SMALL_EXAMPLE.to_string()).unwrap());
        assert_eq!(220, part1(LARGE_EXAMPLE.to_string()).unwrap());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(8, part2(SMALL_EXAMPLE.to_string()).unwrap());
        assert_eq!(19208, part2(LARGE_EXAMPLE.to_string()).unwrap());
    }
}