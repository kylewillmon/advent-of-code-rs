use std::collections::{HashSet, HashMap};

use anyhow::Result;

pub fn part1(input: String) -> Result<usize> {
    let tiles = TileMap::from_input(input.as_str());

    Ok(tiles.count())
}

pub fn part2(input: String) -> Result<usize> {
    let mut tiles = TileMap::from_input(input.as_str());

    for _ in 0..100 {
        tiles = tiles.next_day();
    }

    Ok(tiles.count())
}

struct TileMap(HashMap<(i32,i32), bool>);

fn _neighbors(tile: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        Direction::East,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
        Direction::NorthEast,
    ].into_iter()
        .map(|d| d.into_coords())
        .map(|(x, y)| (x+tile.0, y+tile.1))
        .collect()
}

impl TileMap {
    fn from_input(input: &str) -> Self {
        let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();

        for line in input.trim().lines() {
            let tile = DirList::new(line)
                .map(|d| d.into_coords())
                .fold((0, 0), |(a, b), (x, y)| (a+x, b+y));
            tiles.entry(tile)
                .and_modify(|t| *t = !*t)
                .or_insert(true);
        }

        Self(tiles)
    }

    fn count(&self) -> usize {
        self.0.iter().filter(|(_, &b)| b).count()
    }

    fn next_day(self) -> Self {
        let mut tiles: HashMap<(i32, i32), usize> = HashMap::new();

        let black_tiles: HashSet<(i32, i32)> = self.0.into_iter()
            .filter_map(|(t, b)| if b { Some(t) } else { None })
            .collect();

        for tile in black_tiles.iter().cloned() {
            for neighbor in _neighbors(tile) {
                tiles.entry(neighbor)
                    .and_modify(|t| *t += 1)
                    .or_insert(1);
            }
        }

        let mut new_tiles: HashMap<(i32, i32), bool> = HashMap::new();

        for (tile, count) in tiles {
            if count == 2 || (count == 1 && black_tiles.contains(&tile)) {
                new_tiles.insert(tile, true);
            }
        }

        Self(new_tiles)
    }
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn into_coords(self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (0, 1),
            Direction::NorthWest => (-1, 1),
            Direction::SouthEast => (1, -1),
            Direction::SouthWest => (0, -1),
        }
    }
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

    #[test]
    fn part2_example() {
        assert_eq!(2208, part2(EXAMPLE.to_string()).unwrap());
    }
}
