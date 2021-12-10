use std::collections::HashMap;
use std::convert::TryInto;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoclib::strtools;

pub fn part1(input: String) -> Result<u64> {
    let mut computer = Computer::new();
    for line in input.trim().lines() {
        let line = line.trim();

        let (target, value) = strtools::split_once(line, "=");
        let target = target.trim();
        let value = value.trim();

        if target == "mask" {
            computer = computer.set_mask(value)?;
        } else {
            let memtarget = target
                .strip_prefix("mem[")
                .and_then(|t| t.strip_suffix(']'))
                .ok_or_else(|| anyhow!("invalid target: {}", target))
                .and_then(|l| Ok(l.parse::<u64>()?))?;

            let value = value.parse::<u64>()?;
            computer = computer.write_masked(memtarget, value);
        }
    }
    Ok(computer.mem_iter().map(|(&_loc, &value)| value).sum())
}

pub fn part2(input: String) -> Result<u64> {
    let mut computer = Computer::new();
    for line in input.trim().lines() {
        let line = line.trim();

        let (target, value) = strtools::split_once(line, "=");
        let target = target.trim();
        let value = value.trim();

        if target == "mask" {
            computer = computer.set_mask(value)?;
        } else {
            let memtarget = target
                .strip_prefix("mem[")
                .and_then(|t| t.strip_suffix(']'))
                .ok_or_else(|| anyhow!("invalid target: {}", target))
                .and_then(|l| Ok(l.parse::<u64>()?))?;

            let value = value.parse::<u64>()?;
            computer = computer.write_decoded(memtarget, value);
        }
    }
    Ok(computer.mem_iter().map(|(&_loc, &value)| value).sum())
}

struct Computer {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Computer {
    fn new() -> Self {
        Self {
            mask: Mask::new(),
            memory: HashMap::new(),
        }
    }

    fn set_mask(mut self, mask: &str) -> Result<Self> {
        self.mask = Mask::from_str(mask)?;
        Ok(self)
    }

    fn write_masked(mut self, location: u64, value: u64) -> Self {
        self.memory.insert(location, self.mask.apply(value));
        self
    }

    fn write_decoded(mut self, location: u64, value: u64) -> Self {
        for addr in self.mask.decode(location) {
            self.memory.insert(addr, value);
        }
        self
    }

    fn mem_iter(&self) -> impl Iterator<Item = (&u64, &u64)> + '_ {
        self.memory.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mask {
    xs: Vec<u64>,
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn new() -> Self {
        Self {
            xs: Vec::new(),
            ones: 0,
            zeros: u64::MAX,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & !self.zeros) | self.ones
    }

    fn decode(&self, value: u64) -> Vec<u64> {
        let value = value | self.ones;
        let mut addrs = Vec::new();

        let numvals = 2u64.checked_pow(self.xs.len().try_into().unwrap()).unwrap();
        for i in 0..numvals {
            let mut value = value;
            let mut i = i;
            for x in self.xs.iter().cloned() {
                if i % 2 == 1 {
                    value |= x;
                } else {
                    value &= !x;
                }
                i >>= 1;
            }
            addrs.push(value);
        }

        addrs
    }
}

impl FromStr for Mask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xs = Vec::new();
        let mut ones = 0u64;
        let mut zeros = 0u64;

        for (i, c) in s.trim().chars().rev().enumerate() {
            let val = 2u64.checked_pow(i.try_into().unwrap()).unwrap();

            match c {
                'X' => xs.push(val),
                '0' => zeros |= val,
                '1' => ones |= val,
                _ => panic!("invalid character in mask"),
            }
        }

        Ok(Self { xs, ones, zeros })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";

    #[test]
    pub fn part1_example() {
        assert_eq!(165, part1(EXAMPLE.to_string()).unwrap());
    }

    const EXAMPLE2: &str = "
    mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";

    #[test]
    pub fn part2_example() {
        assert_eq!(208, part2(EXAMPLE2.to_string()).unwrap());
    }
}
