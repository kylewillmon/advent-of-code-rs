use anyhow::{anyhow, ensure, Result};

pub fn part1(input: String) -> Result<u64> {
    let tiles: Vec<Tile> = input.split("\n\n")
        .map(|t| Tile::from_input(t))
        .collect::<Result<_, _>>()?;

    let mut corners = Vec::new();
    for t in tiles.iter() {
        let unmatch = t.unmatchable_sides(&tiles);
        if unmatch == 2 {
            corners.push(t.num);
        }
    }

    assert_eq!(4, corners.len());

    Ok(corners.into_iter().product())
}

pub fn part2(_input: String) -> Result<u64> {
    Ok(0)
}

#[derive(Debug, Clone)]
struct Tile {
    num: u64,
    side: usize,
    tile: Vec<char>,
}

impl Tile {
    fn from_input(input: &str) -> Result<Self> {
        let mut lines = input.trim().lines();

        let num = lines.next()
            .and_then(|l| l.strip_prefix("Tile "))
            .and_then(|l| l.strip_suffix(":"))
            .ok_or(anyhow!("invalid tile header"))?
            .parse::<u64>()?;

        let mut tile = Vec::new();
        let side = lines.clone().count();

        for line in lines {
            ensure!(line.trim().chars().count() == side, "tile must be square");
            tile.extend(line.chars());
        }

        Ok(Self { num, side, tile })
    }

    fn unmatchable_sides(&self, others: &[Self]) -> usize {
        let mut unmatchable_sides = 0;
        for side in self.get_sides().into_iter().take(4) {
            let matchable = others.iter()
                .filter(|oth| oth.num != self.num)
                .any(|oth| {
                    oth.get_sides().into_iter().any(|s| s == side)
                });
            if !matchable {
                unmatchable_sides += 1;
            }
        }
        unmatchable_sides
    }

    fn get(&self, r: usize, c: usize) -> Option<char> {
        self.tile
            .chunks_exact(self.side)
            .nth(r)
            .and_then(|row| row.get(c))
            .map(|&c| c)
    }

    fn get_sides(&self) -> Vec<String> {
        let mut sides = Vec::new();
        let last = self.side-1;

        sides.push(
            (0..self.side)
                .map(|c| self.get(0, c).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side)
                .map(|r| self.get(r, last).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side).rev()
                .map(|c| self.get(last, c).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side).rev()
                .map(|r| self.get(r, 0).unwrap())
                .collect()
        );

        // Flipped
        sides.push(
            (0..self.side)
                .map(|r| self.get(r, 0).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side)
                .map(|c| self.get(last, c).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side).rev()
                .map(|r| self.get(r, last).unwrap())
                .collect()
        );
        sides.push(
            (0..self.side).rev()
                .map(|c| self.get(0, c).unwrap())
                .collect()
        );

        sides
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
