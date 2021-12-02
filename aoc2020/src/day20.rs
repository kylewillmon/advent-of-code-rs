use anyhow::{anyhow, ensure, Result};
use aoclib::strtools;
use itertools::Itertools;

pub fn part1(input: String) -> Result<u64> {
    let tiles = parse_input(input)?;

    let corners: Vec<u64> = tiles
        .into_iter()
        .filter(|t| t.is_corner())
        .map(|t| t.num)
        .collect();

    assert_eq!(4, corners.len());

    Ok(corners.into_iter().product())
}

pub fn part2(_input: String) -> Result<u64> {
    Ok(0)
}

fn parse_input(input: String) -> Result<Vec<PartialTile>> {
    let mut tiles: Vec<PartialTile> = input
        .split("\n\n")
        .map(|t| PartialTile::from_input(t))
        .collect::<Result<_, _>>()?;

    for i in 0..tiles.len() {
        let matches = tiles[i].calc_matches(&tiles);
        tiles[i].side_matches = matches;
    }

    Ok(tiles)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

struct Side {
    /// The characters on the side when traversed clockwise.
    side: String,
}

impl Side {
    /// Determines if sides match with only rotation
    fn matches(&self, other: &Self) -> bool {
        self.side.bytes().rev().eq(other.side.bytes())
    }

    /// Determines if sides match with rotation after one tile is flipped
    fn matches_flipped(&self, other: &Self) -> bool {
        self.side == other.side
    }
}

#[derive(Debug, Clone)]
struct Tile {
    side: usize,
    tile: Vec<char>,
}

impl Tile {
    fn from_input(input: &str) -> Result<Self> {
        let mut tile = Vec::new();
        let side = input.trim().lines().count();

        for line in input.trim().lines() {
            ensure!(line.trim().chars().count() == side, "tile must be square");
            tile.extend(line.chars());
        }

        Ok(Self { side, tile })
    }

    fn get(&self, r: usize, c: usize) -> Option<char> {
        self.tile
            .chunks_exact(self.side)
            .nth(r)
            .and_then(|row| row.get(c))
            .map(|&c| c)
    }

    fn get_side(&self, pos: Position) -> Side {
        let last = self.side - 1;
        let side: String = match pos {
            Position::Top => (0..self.side).map(|c| self.get(0, c).unwrap()).collect(),
            Position::Right => (0..self.side).map(|r| self.get(r, last).unwrap()).collect(),
            Position::Bottom => (0..self.side)
                .rev()
                .map(|c| self.get(last, c).unwrap())
                .collect(),
            Position::Left => (0..self.side)
                .rev()
                .map(|r| self.get(r, 0).unwrap())
                .collect(),
        };
        Side { side }
    }

    fn get_sides(&self) -> Vec<Side> {
        let mut sides = Vec::new();

        sides.push(self.get_side(Position::Top));
        sides.push(self.get_side(Position::Right));
        sides.push(self.get_side(Position::Bottom));
        sides.push(self.get_side(Position::Left));

        sides
    }
}

#[derive(Debug, Clone)]
struct PartialTile {
    num: u64,
    side_matches: Vec<Option<u64>>,
    tile: Tile,
}

impl PartialTile {
    fn from_input(input: &str) -> Result<Self> {
        let (title, tile) = strtools::split_once(input.trim(), "\n");

        let num = title
            .strip_prefix("Tile ")
            .and_then(|l| l.strip_suffix(":"))
            .ok_or(anyhow!("invalid tile header"))?
            .parse::<u64>()?;

        let tile = Tile::from_input(tile)?;

        Ok(Self {
            num,
            side_matches: Vec::new(),
            tile,
        })
    }

    fn calc_matches(&self, others: &[Self]) -> Vec<Option<u64>> {
        let mut side_matches = Vec::new();
        for side in self.tile.get_sides() {
            let matches = others
                .iter()
                .filter(|oth| oth.num != self.num)
                .filter(|oth| {
                    oth.tile
                        .get_sides()
                        .into_iter()
                        .any(|s| s.matches(&side) || s.matches_flipped(&side))
                });
            if let Ok(other) = matches.exactly_one() {
                side_matches.push(Some(other.num))
            }
        }
        side_matches
    }

    fn is_corner(&self) -> bool {
        self.side_matches.iter().filter(|s| s.is_some()).count() == 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn part1_example() {
        assert_eq!(20_899_048_083_289u64, part1(EXAMPLE.to_string()).unwrap());
    }
}
