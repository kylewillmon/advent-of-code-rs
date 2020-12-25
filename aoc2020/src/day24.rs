use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: String) -> Result<usize> {
    let mut tiles = HashMap::new();

    for line in input.trim().lines() {
        let tile = DirList::new(line)
            .map(|d| match d {
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
                Direction::NorthEast => (0, 1),
                Direction::NorthWest => (-1, 1),
                Direction::SouthEast => (1, -1),
                Direction::SouthWest => (0, -1),
            })
            .fold((0, 0), |(a, b), (x, y)| (a+x, b+y));
        tiles.entry(tile)
            .and_modify(|t| *t += 1)
            .or_insert(1);
    }

    Ok(
        tiles.into_iter()
        .filter(|&(_t, count)| count % 2 == 1)
        .count()
    )
}

pub fn part2(_input: String) -> Result<usize> {
    Ok(0)
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

struct DirList<'a>(&'a str);

impl<'a> DirList<'a> {
    fn new(line: &'a str) -> Self {
        Self(line.trim())
    }
}

impl Iterator for DirList<'_> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.0.chars();
        let dir = {
            match chars.next() {
                None => None,
                Some('e') => Some(Direction::East),
                Some('w') => Some(Direction::West),
                Some('n') => match chars.next() {
                    Some('e') => Some(Direction::NorthEast),
                    Some('w') => Some(Direction::NorthWest),
                    _ => panic!("invalid direction"),
                },
                Some('s') => match chars.next() {
                    Some('e') => Some(Direction::SouthEast),
                    Some('w') => Some(Direction::SouthWest),
                    _ => panic!("invalid direction"),
                },
                _ => panic!("invalid direction"),
            }
        };
        self.0 = chars.as_str();
        dir
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn part1_example() {
        assert_eq!(10, part1(EXAMPLE.to_string()).unwrap());
    }
}