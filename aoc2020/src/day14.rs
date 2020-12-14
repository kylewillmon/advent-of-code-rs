use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;

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
                .and_then(|t| t.strip_suffix("]"))
                .ok_or(anyhow!("invalid target: {}", target))
                .and_then(|l| Ok(l.parse::<u64>()?))?;

            let value = value.parse::<u64>()?;
            computer = computer.write_mem(memtarget, value);
        }
    }
    Ok(computer.mem_iter().map(|(&_loc, &value)| value).sum())
}

pub fn part2(_input: String) -> Result<u64> {
    Err(anyhow!("not implemented"))
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

    fn write_mem(mut self, location: u64, value: u64) -> Self {
        self.memory.insert(location, self.mask.apply(value));
        self
    }

    fn mem_iter(&self) -> impl Iterator<Item = (&u64, &u64)> + '_ {
        self.memory.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mask {
    /// Bits which will be overwritten
    bits: u64,
    /// The value to overwrite them with
    value: u64,
}

impl Mask {
    fn new() -> Self {
        Self {
            bits: 0,
            value: 0,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & !self.bits) | self.value
    }
}

impl FromStr for Mask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = u64::from_str_radix(&s.replace("0", "1").replace("X", "0"), 2)?;
        let value = u64::from_str_radix(&s.replace("X", "0"), 2)?;

        Ok(Self {
            bits: bits,
            value: value,
        })
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
}
