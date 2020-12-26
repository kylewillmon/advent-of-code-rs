use anyhow::Result;

pub fn part1(_: String) -> Result<String> {
    part1_answer("974618352")
}

pub fn part2(_: String) -> Result<u64> {
    part2_answer("974618352")
}

struct CupList {
    cups: Vec<usize>,
    head: usize,
    tail: usize,
    max: usize,
}

impl CupList {
    fn with_size(size: usize) -> Self {
        Self {
            cups: vec![usize::MAX; size],
            head: usize::MAX,
            tail: usize::MAX,
            max: 0,
        }
    }
}

impl Extend<usize> for CupList {
    fn extend<T: IntoIterator<Item=usize>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();

        if self.head == usize::MAX {
            if let Some(num) = iter.next() {
                self.head = num;
                self.tail = num;
                self.cups[num] = num;
                self.max = num;
            }
        }

        for num in iter {
            self.cups[self.tail] = num;
            self.tail = num;
            if num > self.max {
                self.max = num;
            }
        }
        self.cups[self.tail] = self.head;
    }
}

fn part1_cups(s: &str) -> CupList {
    let mut cups = CupList::with_size(11);

    cups.extend(
        s.bytes().map(|c| c - b'0').map(|c| c.into())
    );

    cups
}

fn part2_cups(s: &str) -> CupList {
    let mut cups = CupList::with_size(1_000_001);

    cups.extend(
        s.bytes().map(|c| c - b'0').map(|c| c.into())
    );

    let next = s.bytes().count()+1;
    cups.extend(next..=1_000_000);

    cups
}

fn play_crab_cups(mut cups: CupList, iterations: usize) -> CupList {
    for _ in 0..iterations {
        make_move(&mut cups);
    }

    cups
}

fn part1_answer(input: &str) -> Result<String> {
    let cups = play_crab_cups(
        part1_cups(input),
        100,
    );

    let mut idx = cups.cups[1];
    let mut s = String::new();

    while idx != 1 {
        let c = ((idx as u8) + b'0').into();
        s.push(c);
        idx = cups.cups[idx];
    }

    Ok(s)
}

fn part2_answer(input: &str) -> Result<u64> {
    let cups = play_crab_cups(
        part2_cups(input),
        10_000_000,
    );

    let a = cups.cups[1];
    let b = cups.cups[a];

    Ok((a as u64) * (b as u64))
}

fn destination_cup(cur: usize, max: usize, a: usize, b: usize, c: usize) -> usize {
    let dest = {
        if cur == 1 {
            max
        } else {
            cur - 1
        }
    };

    if a == dest || b == dest || c == dest {
        destination_cup(dest, max, a, b, c)
    } else {
        dest
    }
}

fn make_move(cups: &mut CupList) {
    let cur = cups.head;

    let first_removed = cups.cups[cur];
    let second_removed = cups.cups[first_removed];
    let last_removed = cups.cups[second_removed];

    let next_head = cups.cups[last_removed];

    let dest = destination_cup(cur, cups.max, first_removed, second_removed, last_removed);
    let dest_tail = cups.cups[dest];

    cups.cups[cur] = next_head;
    cups.cups[dest] = first_removed;
    cups.cups[last_removed] = dest_tail;

    cups.tail = cur;
    cups.head = next_head;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!("67384529", part1_answer("389125467").unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(149245887792, part2_answer("389125467").unwrap());
    }
}
