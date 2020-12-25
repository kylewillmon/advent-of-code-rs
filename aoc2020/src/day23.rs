use std::collections::VecDeque;

use anyhow::{anyhow, Result};

pub fn part1(_: String) -> Result<String> {
    part1_answer("974618352")
}

pub fn part2(_: String) -> Result<u64> {
    part2_answer("974618352")
}

fn part1_cups(s: &str) -> VecDeque<u32> {
    s.bytes().map(|c| c - b'0').map(|c| c.into()).collect()
}

fn part2_cups(s: &str) -> VecDeque<u32> {
    let mut cups = VecDeque::with_capacity(1_000_000);

    cups.append(&mut part1_cups(s));

    let next = (cups.len()+1) as u32;
    cups.extend(
        next..=1_000_000
    );

    cups.shrink_to_fit();

    cups
}

fn play_crab_cups(mut cups: VecDeque<u32>, iterations: usize) -> VecDeque<u32> {
    for _ in 0..iterations {
        make_move(&mut cups);
    }

    cups
}

fn part1_answer(input: &str) -> Result<String> {
    let mut cups = play_crab_cups(
        part1_cups(input),
        100,
    );
    let idx = cups.iter().cloned()
        .position(|c| c == 1)
        .ok_or(anyhow!("cup '1' not found"))?;

    cups.rotate_left(idx+1);
    cups.pop_back();

    Ok(
        cups.into_iter()
            .map(|cup| char::from((cup as u8) + b'0'))
            .collect()
    )
}

fn part2_answer(input: &str) -> Result<u64> {
    let cups = play_crab_cups(
        part2_cups(input),
        10_000_000,
    );

    let idx = cups.iter().cloned()
        .position(|c| c == 1)
        .ok_or(anyhow!("cup '1' not found"))?;

    let a = if idx + 1 == cups.len() { 0 } else { idx + 1 };
    let b = if a + 1 == cups.len() { 0 } else { a + 1 };

    let a = cups[a] as u64;
    let b = cups[b] as u64;

    Ok(a * b)
}

fn destination_cup(cur: u32, max: u32, removed: &[u32]) -> u32 {
    let dest = {
        if cur == 1 {
            max
        } else {
            cur - 1
        }
    };

    if removed.contains(&dest) {
        destination_cup(dest, max, removed)
    } else {
        dest
    }
}

fn make_move(cups: &mut VecDeque<u32>) {
    let max = cups.len() as u32;
    let cur = cups[0];
    cups.rotate_left(1);

    let removed = vec![
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    ];

    let dest = destination_cup(cur, max, &removed);
    let dest_idx = cups.iter().cloned().position(|cup| cup == dest).unwrap();

    for cup in removed.into_iter().rev() {
        cups.insert(dest_idx+1, cup);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!("67384529", part1_answer("389125467").unwrap());
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(149245887792, part2_answer("389125467").unwrap());
    }
}
