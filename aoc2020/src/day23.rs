use anyhow::{anyhow, Result};

pub fn part1(_: String) -> Result<String> {
    play_crab_cups("974618352".to_string(), 100)
}

pub fn part2(_: String) -> Result<usize> {
    Ok(0)
}

fn play_crab_cups(input: String, iterations: usize) -> Result<String> {
    let mut cups = input.into_bytes();

    for _ in 0..iterations {
        make_move(&mut cups);
    }

    let idx = cups.iter().cloned()
        .position(|c| c == b'1')
        .ok_or(anyhow!("cup '1' not found"))?;

    cups.rotate_left(idx+1);
    cups.pop();

    String::from_utf8(cups)
        .map_err(|e| e.into())
}

fn find_destination(cups: &[u8], cur: u8) -> usize {
    cups.iter().cloned()
        .enumerate()
        .max_by_key(|&(_i, c)| {
            let val = if c < cur { 1 } else { 0 };
            (val, c)
        })
        .map(|(i, _)| i)
        .unwrap()
}

fn make_move(cups: &mut [u8]) {
    let cur = cups[0];

    // move current cup to end
    cups.rotate_left(1);

    let dest = find_destination(&cups[3..], cur) + 3;

    // place removed cups after destination
    cups[..=dest].rotate_left(3);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!("92658374", play_crab_cups("389125467".to_string(), 10).unwrap());
        assert_eq!("67384529", play_crab_cups("389125467".to_string(), 100).unwrap());
    }
}
