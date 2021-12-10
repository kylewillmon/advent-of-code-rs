use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::Result;

use super::error::AocError;

pub fn part1(input: String) -> Result<usize> {
    let mut grid = input.parse::<Grid<Space>>()?;

    grid = run_simulation(
        grid,
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
    Ok(grid.iter().filter(|&&s| s == Space::TakenSeat).count())
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn part2(input: String) -> Result<usize> {
    let mut grid = input.parse::<Grid<Space>>()?;
    let mut other: Grid<Space> = Grid::with_size(grid.rows(), grid.cols());

    let mut cur = &mut grid;
    let mut next = &mut other;

    while *cur != *next {
        for r in 0..cur.rows() {
            for c in 0..cur.cols() {
                let mut count = 0;
                for (dr, dc) in NEIGHBORS.iter().cloned() {
                    let mut r = r.wrapping_add(dr as usize);
                    let mut c = c.wrapping_add(dc as usize);
                    let mut loc = cur.get((r, c));
                    while let Some(s) = loc {
                        if *s == Space::Floor {
                            r = r.wrapping_add(dr as usize);
                            c = c.wrapping_add(dc as usize);
                            loc = cur.get((r, c));
                        } else {
                            if *s == Space::TakenSeat {
                                count += 1;
                            }
                            loc = None;
                        }
                    }
                }

                let s = *cur.get((r, c)).unwrap();
                let new_space = if count == 0 {
                    match s {
                        Space::EmptySeat => Space::TakenSeat,
                        _ => s,
                    }
                } else if count >= 5 {
                    match s {
                        Space::TakenSeat => Space::EmptySeat,
                        _ => s,
                    }
                } else {
                    s
                };
                *next.get_mut((r, c)).unwrap() = new_space;
            }
        }
        std::mem::swap(&mut cur, &mut next);
    }

    Ok(grid.iter().filter(|&&s| s == Space::TakenSeat).count())
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
        up.map(|up| (up, pos.1)),
        up.and_then(|up| right.map(|right| (up, right))),
        left.map(|left| (pos.0, left)),
        right.map(|right| (pos.0, right)),
        down.and_then(|down| left.map(|left| (down, left))),
        down.map(|down| (down, pos.1)),
        down.and_then(|down| right.map(|right| (down, right))),
    ].into_iter()
        .flatten()
        .collect()
}

fn run_simulation<F, G>(mut grid: Grid<Space>, filter: F, translate: G) -> Grid<Space>
where
    F: Fn(&Space) -> bool,
    G: Fn(usize, &Space) -> Space,
{
    let mut other: Grid<Space> = Grid::with_size(grid.rows(), grid.cols());

    let mut cur = &mut grid;
    let mut next = &mut other;

    while *cur != *next {
        for r in 0..cur.rows() {
            for c in 0..cur.cols() {
                let mut count = 0;
                for (r, c) in _neighbors((r, c), (cur.rows(), cur.cols())) {
                    if filter(cur.get((r, c)).unwrap()) {
                        count += 1;
                    }
                }
                *next.get_mut((r, c)).unwrap() = translate(count, cur.get((r, c)).unwrap());
            }
        }
        std::mem::swap(&mut cur, &mut next);
    }

    grid
}

impl<T: Eq + Clone + Default> Grid<T> {
    fn from_vec(grid: Vec<T>, rows: usize) -> Self {
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

    fn with_size(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        if size == 0 {
            Grid {
                grid: Vec::new(),
                rows: 0,
                cols: 0,
            }
        } else {
            Grid {
                grid: vec![T::default(); size],
                rows,
                cols,
            }
        }
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    // calculate index for inner vector. This does no bounds checking and will never fail. Let the vector handle bounds checking.
    fn _pos_index(&self, pos: (usize, usize)) -> usize {
        pos.0.wrapping_mul(self.cols).wrapping_add(pos.1)
    }

    fn get(&self, pos: (usize, usize)) -> Option<&T> {
        if pos.0 < self.rows && pos.1 < self.cols {
            self.grid.get(self._pos_index(pos))
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        let idx = self._pos_index(pos);
        self.grid.get_mut(idx)
    }

    fn _as_mut_slices(&mut self) -> Vec<&mut[T]> {
        self.grid.as_mut_slice().chunks_mut(self.cols).collect()
    }

    fn _as_slices(&self) -> Vec<&[T]> {
        self.grid.as_slice().chunks(self.cols).collect()
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=&T> + 'a {
        self.grid.iter()
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
        let cols = s.lines().next()
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
        Ok(Grid::from_vec(v, rows))
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

    #[test]
    fn part2_example() {
        assert_eq!(26, part2(EXAMPLE.to_string()).unwrap());
    }
}
