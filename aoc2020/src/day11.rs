use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use super::error::AocError;

pub fn part1(input: String) -> Result<usize> {
    let mut grid = input.parse::<Grid<Space>>()?;

    grid = grid.run_simulation(
        |&s| s == Space::TakenSeat,
        |count, &s| {
            if count == 0 {
                match s {
                    Space::EmptySeat => Space::TakenSeat,
                    _ => s,
                }
            } else if count >= 4 {
                match s {
                    Space::TakenSeat => Space::EmptySeat,
                    _ => s,
                }
            } else {
                s
            }
        }
    );
    Ok(grid.count(|&s| s == Space::TakenSeat))
}

pub fn part2(_input: String) -> Result<usize> {
    Err(anyhow!("not yet implemented"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    EmptySeat,
    TakenSeat,
    Floor,
}

impl Default for Space {
    fn default() -> Self { Space::Floor }
}

impl TryFrom<char> for Space {
    type Error = AocError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Space::EmptySeat),
            '#' => Ok(Space::TakenSeat),
            '.' => Ok(Space::Floor),
            _ => Err(AocError::ParseError("unknown character".to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<T: Eq + Clone + Default> {
    grid: Vec<T>,
    rows: usize,
    cols: usize,
}

fn _neighbors(pos: (usize, usize), grid_size: (usize, usize)) -> Vec<(usize, usize)> {
    let up = if pos.0 == 0 { None } else { Some(pos.0 - 1) };
    let down = {
        let down = pos.0 + 1;
        if down == grid_size.0 { None } else { Some(down) }
    };
    let left = if pos.1 == 0 { None } else { Some(pos.1 - 1) };
    let right = {
        let right = pos.1 + 1;
        if right == grid_size.1 { None } else { Some(right) }
    };

    vec![
        up.and_then(|up| left.map(|left| (up, left))),
        up.and_then(|up| Some((up, pos.1))),
        up.and_then(|up| right.map(|right| (up, right))),
        left.and_then(|left| Some((pos.0, left))),
        right.and_then(|right| Some((pos.0, right))),
        down.and_then(|down| left.map(|left| (down, left))),
        down.and_then(|down| Some((down, pos.1))),
        down.and_then(|down| right.map(|right| (down, right))),
    ].into_iter()
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect()
}

impl<T: Eq + Clone + Default> Grid<T> {
    fn new(grid: Vec<T>, rows: usize) -> Self {
        let cols = if rows == 0 {
            0
        } else {
            grid.len() / rows
        };
        assert_eq!(grid.len(), cols * rows);
        Grid {
            grid,
            rows,
            cols,
        }
    }

    fn _as_mut_slices(&mut self) -> Vec<&mut[T]> {
        self.grid.as_mut_slice().chunks_mut(self.cols).collect()
    }

    fn _as_slices(&self) -> Vec<&[T]> {
        self.grid.as_slice().chunks(self.cols).collect()
    }

    fn count<F>(&self, filter: F) -> usize
    where
        F: Fn(&T) -> bool
    {
        self.grid.iter()
            .filter(|i| filter(i))
            .count()
    }

    fn run_simulation<F, G>(mut self, filter: F, translate: G) -> Self
    where
        F: Fn(&T) -> bool,
        G: Fn(usize, &T) -> T,
    {
        let (rows, cols) = (self.rows, self.cols);
        let mut other = Grid {
            grid: vec![T::default(); self.rows * self.cols],
            rows,
            cols,
        };

        let mut cur = &mut self;
        let mut next = &mut other;

        while *cur != *next {
            {
                let cur = cur._as_slices();
                let mut next = next._as_mut_slices();
                for r in 0..rows {
                    for c in 0..cols {
                        let count = _neighbors((r, c), (rows, cols))
                            .into_iter()
                            .filter(|(r, c)| filter(&cur[*r][*c]))
                            .count();
                        next[r][c] = translate(count, &cur[r][c]);
                    }
                }
            }
            let tmp = cur; cur = next; next = tmp;
        }

        self
    }
}

impl<T: Eq + Clone + Default + TryFrom<char>> FromStr for Grid<T>
where
    <T as TryFrom<char>>::Error: Into<AocError>
{
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<T> = Vec::new();
        let rows = s.lines().count();
        let cols = s.lines()
            .nth(0)
            .map(|l| l.chars().count())
            .ok_or(AocError::ParseError("grid cannot have 0 rows".to_string()))?;
        for row in s.lines() {
            if cols != row.chars().count() {
                return Err(AocError::ParseError("each row must have the same number of columns".to_string()));
            }

            let items = row.chars().map(|c| T::try_from(c)).collect::<Result<Vec<T>, _>>();
            match items {
                Ok(mut items) => v.append(&mut items),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(Grid::new(v, rows))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part1_example() {
        assert_eq!(37, part1(EXAMPLE.to_string()).unwrap());
    }
}