use anyhow::{anyhow, ensure, Result};
use aoclib::strtools;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String) -> Result<u64> {
    let tiles = parse_input(input)?;

    let corners: Vec<u64> = tiles
        .into_iter()
        .filter_map(|(num, t)| if t.is_corner() { Some(num) } else { None })
        .collect();

    assert_eq!(4, corners.len());

    Ok(corners.into_iter().product())
}

pub fn part2(input: String) -> Result<u64> {
    let mut tiles = parse_input(input)?;

    let num_tiles = tiles.len();
    let bigside = {
        let mut bigside = 1;
        while bigside * bigside < num_tiles {
            bigside += 1;
        }
        bigside
    };

    assert!(bigside * bigside == tiles.len());

    let first_corner: u64 = tiles
        .iter()
        .filter_map(|(&num, t)| if t.is_corner() { Some(num) } else { None })
        .next()
        .unwrap();

    let mut first = tiles.remove(&first_corner).unwrap();

    while first.has_match(Position::Top) || first.has_match(Position::Left) {
        first.rotate();
    }

    let mut grid = vec![first];

    for i in 1..num_tiles {
        let (last, last_side, next_side) = if i % bigside == 0 {
            (i - bigside, Position::Bottom, Position::Top)
        } else {
            (i - 1, Position::Right, Position::Left)
        };
        let next = grid[last].get_match(last_side);
        let last_side = grid[last].tile.get_side(last_side);

        let mut next = tiles.remove(&next.unwrap()).unwrap();

        next.fiddle_until(|p: &PartialTile| p.tile.get_side(next_side).matches(&last_side));

        grid.push(next);
    }

    let mut big_tile = Tile::from_partial_tiles(bigside, &grid)?;

    big_tile.fiddle_until(|t| t.has_sea_monsters());
    big_tile.clear_sea_monster();
    Ok(big_tile.roughness())
}

fn parse_input(input: String) -> Result<HashMap<u64, PartialTile>> {
    let mut tiles: HashMap<u64, PartialTile> = input
        .split("\n\n")
        .map(PartialTile::from_input)
        .collect::<Result<_, _>>()?;

    let keys: Vec<u64> = tiles.keys().copied().collect();

    for k in keys {
        let matches = tiles[&k].calc_matches(tiles.iter().filter(|&(&num, _)| num != k));
        tiles.get_mut(&k).unwrap().side_matches = matches;
    }

    Ok(tiles)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

struct Side {
    /// The characters on the side when traversed clockwise.
    side: Vec<u8>,
}

impl Side {
    /// Determines if sides match with only rotation
    fn matches(&self, other: &Self) -> bool {
        self.side.iter().rev().eq(&other.side)
    }

    /// Determines if sides match with rotation after one tile is flipped
    fn matches_flipped(&self, other: &Self) -> bool {
        self.side == other.side
    }
}

#[derive(Debug, Clone)]
struct Tile {
    side: usize,
    tile: Vec<u8>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.side {
            writeln!(f, "{}", unsafe {
                std::str::from_utf8_unchecked(self.row(row).unwrap())
            })?;
        }
        Ok(())
    }
}

impl Tile {
    fn from_input(input: &str) -> Result<Self> {
        let mut tile = Vec::new();
        let side = input.trim().lines().count();

        for line in input.trim().lines() {
            ensure!(line.trim().bytes().count() == side, "tile must be square");
            tile.extend(line.bytes());
        }

        debug_assert!(tile.len() == side * side);

        Ok(Self { side, tile })
    }

    fn from_partial_tiles(side: usize, tiles: &[PartialTile]) -> Result<Self> {
        let small_side = tiles
            .first()
            .and_then(|p| p.tile.side.checked_sub(2))
            .ok_or_else(|| anyhow!("failed to get small_side"))?;
        let big_side = small_side * side;

        let mut tile = Vec::with_capacity(big_side * big_side);

        for grid_row in 0..side {
            for small_row in 0..small_side {
                for grid_col in 0..side {
                    let row = tiles[grid_row * side + grid_col]
                        .tile
                        .row(small_row + 1)
                        .map(|r| &r[1..(small_side + 1)])
                        .ok_or_else(|| {
                            anyhow!(
                                "row {} not found for tile at ({}, {})",
                                small_row,
                                grid_row,
                                grid_col
                            )
                        })?;
                    tile.extend(row);
                }
            }
        }

        debug_assert!(tile.len() == big_side * big_side);

        Ok(Self {
            side: big_side,
            tile,
        })
    }

    fn get(&self, r: usize, c: usize) -> Option<u8> {
        if r < self.side && c < self.side {
            Some(self.tile[r * self.side + c])
        } else {
            None
        }
    }

    fn get_side(&self, pos: Position) -> Side {
        let last = self.side - 1;
        let side: Vec<u8> = match pos {
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

    fn get_sides(&self) -> Vec<(Position, Side)> {
        vec![
            (Position::Top, self.get_side(Position::Top)),
            (Position::Right, self.get_side(Position::Right)),
            (Position::Bottom, self.get_side(Position::Bottom)),
            (Position::Left, self.get_side(Position::Left)),
        ]
    }

    // Clockwise 90 degree rotation
    fn rotate(&mut self) {
        let mut new: Vec<u8> = Vec::with_capacity(self.tile.len());

        for c in 0..self.side {
            for r in (0..self.side).rev() {
                new.push(self.tile[r * self.side + c]);
            }
        }

        debug_assert!(new.len() == self.side * self.side);

        self.tile = new;
    }

    // Flip across vertical axis
    fn flip(&mut self) {
        for chunk in self.tile.chunks_exact_mut(self.side) {
            chunk.reverse();
        }

        debug_assert!(self.tile.len() == self.side * self.side);
    }

    fn row(&self, row: usize) -> Option<&[u8]> {
        self.tile.chunks_exact(self.side).nth(row)
    }

    fn fiddle_until<Pred>(&mut self, mut pred: Pred) -> bool
    where
        Pred: FnMut(&Self) -> bool,
    {
        for _ in 0..3 {
            if pred(self) {
                return true;
            }
            self.rotate();
        }
        if pred(self) {
            return true;
        }
        self.flip();
        for _ in 0..3 {
            if pred(self) {
                return true;
            }
            self.rotate();
        }
        pred(self)
    }

    fn sea_monster_check(&self, row: usize, col: usize) -> bool {
        SEA_MONSTER_POINTS.iter().all(|&(dr, dc)| {
            let r = row + dr;
            let c = col + dc;
            self.tile[r * self.side + c] == b'#'
        })
    }

    fn has_sea_monsters(&self) -> bool {
        for r in 0..(self.side - 3) {
            for c in 0..(self.side - 20) {
                if self.sea_monster_check(r, c) {
                    return true;
                }
            }
        }
        false
    }

    fn clear_sea_monster(&mut self) {
        let mut points = Vec::new();
        for r in 0..(self.side - 3) {
            for c in 0..(self.side - 20) {
                if self.sea_monster_check(r, c) {
                    points.push((r, c))
                }
            }
        }

        for (r, c) in points {
            for (dr, dc) in &SEA_MONSTER_POINTS {
                let row = r + dr;
                let col = c + dc;
                self.tile[row * self.side + col] = b'.';
            }
        }
    }

    fn roughness(&self) -> u64 {
        self.tile.iter().filter(|&&x| x == b'#').count() as u64
    }
}

//00000000001111111111
//01234567890123456789
//                  #
//#    ##    ##    ###
// #  #  #  #  #  #
static SEA_MONSTER_POINTS: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Debug, Clone)]
struct PartialTile {
    side_matches: Vec<(Position, u64)>,
    tile: Tile,
}

impl PartialTile {
    fn from_input(input: &str) -> Result<(u64, Self)> {
        let (title, tile) = strtools::split_once(input.trim(), "\n");

        let num = title
            .trim()
            .strip_prefix("Tile ")
            .and_then(|l| l.strip_suffix(':'))
            .ok_or(anyhow!("invalid tile header"))?
            .parse::<u64>()?;

        let tile = Tile::from_input(tile)?;

        Ok((
            num,
            Self {
                side_matches: Vec::new(),
                tile,
            },
        ))
    }

    fn calc_matches<'a, T: IntoIterator<Item = (&'a u64, &'a Self)> + Clone>(
        &self,
        others: T,
    ) -> Vec<(Position, u64)> {
        let mut side_matches = Vec::new();
        for (pos, side) in self.tile.get_sides() {
            let matches = others.clone().into_iter().filter(|&(_, oth)| {
                oth.tile
                    .get_sides()
                    .into_iter()
                    .any(|(_, s)| s.matches(&side) || s.matches_flipped(&side))
            });
            if let Ok((&num, _)) = matches.exactly_one() {
                side_matches.push((pos, num))
            }
        }
        side_matches
    }

    fn is_corner(&self) -> bool {
        self.side_matches.len() == 2
    }

    fn rotate(&mut self) {
        self.tile.rotate();

        for &mut (ref mut pos, _) in self.side_matches.iter_mut() {
            let rotated = match pos {
                Position::Top => Position::Right,
                Position::Right => Position::Bottom,
                Position::Bottom => Position::Left,
                Position::Left => Position::Top,
            };
            *pos = rotated;
        }
    }

    fn flip(&mut self) {
        self.tile.flip();

        for &mut (ref mut pos, _) in self.side_matches.iter_mut() {
            let rotated = match pos {
                Position::Top => Position::Top,
                Position::Right => Position::Left,
                Position::Bottom => Position::Bottom,
                Position::Left => Position::Right,
            };
            *pos = rotated;
        }
    }

    fn has_match(&self, pos: Position) -> bool {
        matches!(self.side_matches.iter().find(|(p, _)| *p == pos), Some(_))
    }

    fn get_match(&self, pos: Position) -> Option<u64> {
        self.side_matches
            .iter()
            .find(|&(p, _)| *p == pos)
            .map(|&(_, num)| num)
    }

    fn fiddle_until<Pred>(&mut self, mut pred: Pred) -> bool
    where
        Pred: FnMut(&Self) -> bool,
    {
        for _ in 0..3 {
            if pred(self) {
                return true;
            }
            self.rotate();
        }
        if pred(self) {
            return true;
        }
        self.flip();
        for _ in 0..3 {
            if pred(self) {
                return true;
            }
            self.rotate();
        }
        pred(self)
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

    #[test]
    fn part2_example() {
        assert_eq!(273u64, part2(EXAMPLE.to_string()).unwrap());
    }
}
