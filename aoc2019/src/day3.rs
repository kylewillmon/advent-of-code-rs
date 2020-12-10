use anyhow::{anyhow, Result};

pub fn part1(_input: String) -> Result<u32> {
    Err(anyhow!("not yet implemented"))
}

pub fn part2(_input: String) -> Result<u32> {
    Err(anyhow!("not yet implemented"))
}

fn move_point(start: (i32, i32), dir: char, len: i32) -> (i32, i32) {
    match dir {
        'R' => (start.0 + len, start.1),
        'L' => (start.0 - len, start.1),
        'U' => (start.0, start.1 + len),
        'D' => (start.0, start.1 - len),
        _ => panic!("invalid direction"),
    }
}

fn wire_points<'a>(line: &'a str) -> impl Iterator<Item = (i32, i32)> + 'a {
    line
        .split(',')
        .scan((0, 0), |state, motion| {
            let (dir, len) = motion.split_at(1);
            let dir = dir.chars().nth(0).unwrap();
            let len = len.parse::<i32>().unwrap();

            let start = *state;

            *state = move_point(start, dir, len);

            Some((1..=len).map(move |l| move_point(start, dir, l)))
        })
        .flatten()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wire_points() {
        let input = "R3,U1,L2,D1";
        let points = vec![
            (1, 0), (2, 0), (3, 0),
            (3, 1),
            (2, 1), (1, 1),
            (1, 0),
        ];

        assert_eq!(points, wire_points(input).collect::<Vec<(i32, i32)>>());
    }
}