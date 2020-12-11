use std::collections::{HashSet, HashMap};

use anyhow::{anyhow, Result};

pub fn part1(input: String) -> Result<i32> {
    let (wire1, wire2) = {
        let mut split = input.lines();
        split.next()
            .and_then(|a| {
                split.next().map(|b| (a, b))
            })
            .ok_or(anyhow!("input too short"))?
    };

    let points1: HashSet<(i32, i32)> = wire_points(wire1).collect();
    let points2: HashSet<(i32, i32)> = wire_points(wire2).collect();

    points1.intersection(&points2)
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .ok_or(anyhow!("wires do not intersect"))
}

pub fn part2(input: String) -> Result<usize> {
    let (wire1, wire2) = {
        let mut split = input.lines();
        split.next()
            .and_then(|a| {
                split.next().map(|b| (a, b))
            })
            .ok_or(anyhow!("input too short"))?
    };

    let points1: HashMap<(i32, i32), usize> = wire_points(wire1).enumerate().map(|(a, b)| (b, a)).collect();

    let mut min = usize::MAX;

    for (i, p) in wire_points(wire2).enumerate() {
        if let Some(steps) = points1.get(&p) {
            let distance = steps + i + 2;
            if distance < min {
                min = distance;
            }
        }
    }
    Ok(min)
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

    const EXAMPLE1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const EXAMPLE2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const EXAMPLE3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_examples() {
        assert_eq!(6, part1(EXAMPLE1.to_string()).unwrap());
        assert_eq!(159, part1(EXAMPLE2.to_string()).unwrap());
        assert_eq!(135, part1(EXAMPLE3.to_string()).unwrap());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(30, part2(EXAMPLE1.to_string()).unwrap());
        assert_eq!(610, part2(EXAMPLE2.to_string()).unwrap());
        assert_eq!(410, part2(EXAMPLE3.to_string()).unwrap());
    }
}
