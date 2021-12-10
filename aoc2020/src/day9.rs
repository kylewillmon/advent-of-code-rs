use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn part1(input: String) -> Result<u64> {
    part1_with_preamble_len(25, input)
}

pub fn part2(input: String) -> Result<u64> {
    part2_with_preamble_len(25, input)
}

fn part1_with_preamble_len(preamble_len: usize, input: String) -> Result<u64> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, _>>()?;
    ensure!(nums.len() >= preamble_len, "input too short");

    let mut xs = XmasState::new(&nums[..preamble_len]);
    for num in nums.into_iter().skip(preamble_len) {
        if !xs.is_valid(num) {
            return Ok(num);
        }
        xs = xs.push(num);
    }
    Err(anyhow!("input is valid"))
}

pub fn part2_with_preamble_len(preamble_len: usize, input: String) -> Result<u64> {
    use std::cmp::Ordering;
    let target = part1_with_preamble_len(preamble_len, input.clone())?;
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, _>>()?;

    let mut start = 0;
    let mut end = 0;
    loop {
        ensure!(end <= nums.len(), "target sum not found");
        let range = &nums[start..end];
        let sum: u64 = range.iter().cloned().sum();
        match sum.cmp(&target) {
            Ordering::Equal => {
                if let MinMax(min, max) = range.iter().cloned().minmax() {
                    return Ok(min + max);
                }
                panic!("range found, but min/max failed");
            }
            Ordering::Less => {
                end += 1;
            }
            Ordering::Greater => {
                start += 1;
                assert!(start <= end, "start crossed end!");
            }
        }
    }
}

struct XmasState {
    nums: VecDeque<u64>,
    sums: HashMap<u64, u8>,
}

impl XmasState {
    fn new(preamble: &[u64]) -> Self {
        let mut xs = XmasState {
            nums: VecDeque::new(),
            sums: HashMap::new(),
        };

        for num in preamble {
            xs = xs._push_num(*num);
        }
        xs
    }

    fn is_valid(&self, num: u64) -> bool {
        self.sums.contains_key(&num)
    }

    fn push(self, num: u64) -> Self {
        self._pop_num()._push_num(num)
    }

    fn _push_num(mut self, num: u64) -> Self {
        for oth in self.nums.iter() {
            if *oth == num {
                continue;
            }
            let sum = *oth + num;
            *self.sums.entry(sum).or_insert(0) += 1;
        }
        self.nums.push_back(num);
        self
    }

    fn _pop_num(mut self) -> Self {
        let num = self.nums.pop_front().unwrap();
        for oth in self.nums.iter() {
            if *oth == num {
                continue;
            }
            let sum = *oth + num;
            let v = self.sums.get_mut(&sum).unwrap();
            *v -= 1;
            if *v == 0 {
                self.sums.remove(&sum);
            }
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part1_example() {
        assert_eq!(
            127,
            part1_with_preamble_len(5, EXAMPLE.to_string()).unwrap()
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(62, part2_with_preamble_len(5, EXAMPLE.to_string()).unwrap());
    }
}
