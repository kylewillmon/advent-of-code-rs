use std::ops::{Add, Sub};
use std::convert::{TryInto, TryFrom};
use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use anyhow::Result;

pub fn part1(input: String) -> Result<usize> {
    let mut grid = Grid::from_input(input.as_str());

    for _ in 0..6 {
        grid = grid.run_cycle();
    }

    Ok(
        grid.visit()
            .map(|(_, c)| c)
            .filter(|&c| c == Cube::Active)
            .count()
    )
}

pub fn part2(input: String) -> Result<usize> {
    let mut grid = Hypergrid::from_input(input.as_str());

    for _ in 0..6 {
        grid = grid.run_cycle();
    }

    Ok(grid.count())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hypercube {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Hypercube {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self{ x, y, z, w }
    }

    fn coords(&self) -> (isize, isize, isize, isize) {
        (self.x, self.y, self.z, self.w)
    }

    fn neighbors(&self) -> Vec<Self> {
        let (x, y, z, w) = self.coords();

        let mut cubes = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        cubes.push(Self::new(x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }

        // Remove self from list
        cubes.remove(cubes.iter().position(|c| *c == *self).unwrap());

        cubes
    }
}

struct Hypergrid(HashSet<Hypercube>);

impl Hypergrid {
    fn run_cycle(self) -> Self {
        let mut neighbor_count: HashMap<Hypercube, usize> = HashMap::new();

        for cube in self.0.iter().cloned() {
            for c in cube.neighbors() {
                neighbor_count.entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let mut grid = HashSet::new();

        for (cube, count) in neighbor_count {
            if count == 3 || (count == 2 && self.0.contains(&cube)) {
                grid.insert(cube);
            }
        }

        Self(grid)
    }

    fn from_input(input: &str) -> Self {
        let mut grid = HashSet::new();
        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    grid.insert(Hypercube::new(x as isize, y as isize, 0, 0));
                }
            }
        }
        Self(grid)
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

struct Grid {
    grid: Vec<Cube>,
    size: Point,
    origin: Point,
}

fn _minmax(a: i32, b: i32) -> (i32, i32) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn _normalize_points(a: &Point, b: &Point) -> (Point, Point) {
    let (xmin, xmax) = _minmax(a.x, b.x);
    let (ymin, ymax) = _minmax(a.y, b.y);
    let (zmin, zmax) = _minmax(a.z, b.z);
    (
        Point::new(xmin, ymin, zmin),
        Point::new(xmax, ymax, zmax),
    )
}

impl Grid {
    fn run_cycle(&self) -> Self {
        let mut next = {
            let (min, max) = self.minmax_points();
            let one = Point::new(1, 1, 1);
            Grid::from_points(&(&min - &one), &(&max + &one))
        };

        let points = next.point_iter();

        for p in points {
            let neighbors = self.active_neighbors(&p);
            let cube = self.get(&p).unwrap_or(Cube::Inactive);

            if cube == Cube::Active && neighbors >= 2 && neighbors <= 3 {
                *next.get_mut(&p).unwrap() = Cube::Active;
            } else if cube == Cube::Inactive && neighbors == 3 {
                *next.get_mut(&p).unwrap() = Cube::Active;
            }
        }

        next
    }

    fn active_neighbors(&self, p: &Point) -> usize {
        let mut count = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let p = p + &Point::new(x, y, z);

                    if let Some(Cube::Active) = self.get(&p) {
                        count += 1;
                    }
                }
            }
        }

        // Remove the cube at point 'p' fromt the count
        if let Some(Cube::Active) = self.get(&p) {
            count - 1
        } else {
            count
        }
    }

    fn from_points(a: &Point, b: &Point) -> Self {
        let (a, b) = _normalize_points(a, b);
        let size = Point::new(
            b.x - a.x + 1,
            b.y - a.y + 1,
            b.z - a.z + 1,
        );
        let grid = vec![Cube::Inactive; (size.x*size.y*size.z).try_into().unwrap()];
        assert!(grid.len() != 0);
        Self {
            grid,
            size,
            origin: Point::zero() - a,
        }
    }

    fn from_input(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap().trim().chars().count();

        let mut grid = Grid::from_points(&Point::zero(), &Point::new(height.try_into().unwrap(), width.try_into().unwrap(), 0));

        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.trim().chars().enumerate() {
                let c = match c {
                    '.' => Cube::Inactive,
                    '#' => Cube::Active,
                    _ => panic!("invalid character"),
                };

                *grid.get_mut(&Point::new(x.try_into().unwrap(), y.try_into().unwrap(), 0)).unwrap() = c;
            }
        }

        grid
    }

    fn get(&self, p: &Point) -> Option<Cube> {
        let p = &self.origin + p;

        self.grid
            .chunks_exact((self.size.y * self.size.z).try_into().unwrap())
            .nth(p.x.try_into().ok()?)?
            .chunks_exact(self.size.z.try_into().unwrap())
            .nth(p.y.try_into().ok()?)?
            .get(usize::try_from(p.z).ok()?)
            .map(|&c| c)
    }

    fn get_mut(&mut self, p: &Point) -> Option<&mut Cube> {
        let p = &self.origin + p;

        self.grid
            .chunks_exact_mut((self.size.y * self.size.z).try_into().unwrap())
            .nth(p.x.try_into().unwrap())?
            .chunks_exact_mut(self.size.z.try_into().unwrap())
            .nth(p.y.try_into().unwrap())?
            .get_mut(usize::try_from(p.z).unwrap())
    }

    fn point_iter(&self) -> impl Iterator<Item=Point> + 'static {
        let first = &Point::zero() - &self.origin;
        let ranges = vec![
            (first.x..(first.x + self.size.x)),
            (first.y..(first.y + self.size.y)),
            (first.z..(first.z + self.size.z)),
        ];
        ranges.into_iter().multi_cartesian_product().map(|v| Point::new(v[0], v[1], v[2]))
    }

    fn visit(&self) -> impl Iterator<Item=(Point, Cube)> + '_ {
        let mut iter = self.point_iter();
        std::iter::from_fn(move || {
            let p = iter.next()?;
            Some((p.clone(), self.get(&p).unwrap()))
        })
    }

    fn minmax_points(&self) -> (Point, Point) {
        let mut minmax = None;

        for (p, c) in self.visit() {
            if c == Cube::Active {
                minmax = match minmax {
                    None => Some((p.clone(), p.clone())),
                    Some((min, max)) => Some((Point::min(&p, &min), Point::max(&p, &max))),
                }
            }
        }

        minmax.unwrap_or((Point::zero(), Point::zero()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cube {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    fn zero() -> Self {
        Point::new(0, 0, 0)
    }

    fn min(a: &Self, b: &Self) -> Self {
        Point::new(
            std::cmp::min(a.x, b.x),
            std::cmp::min(a.y, b.y),
            std::cmp::min(a.z, b.z),
        )
    }

    fn max(a: &Self, b: &Self) -> Self {
        Point::new(
            std::cmp::max(a.x, b.x),
            std::cmp::max(a.y, b.y),
            std::cmp::max(a.z, b.z),
        )
    }
}

impl Add for &'_ Point {
    type Output = Point;

    fn add(self, other: &'_ Point) -> Self::Output {
        Point::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Add::add(&self, &other)
    }
}

impl Sub for &'_ Point {
    type Output = Point;

    fn sub(self, other: &'_ Point) -> Self::Output {
        Point::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Sub::sub(&self, &other)
    }
}

impl From<(i32, i32, i32)> for Point {
    fn from(p: (i32, i32, i32)) -> Self {
        Point::new(p.0, p.1, p.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = ".#.\n..#\n###";

    #[test]
    fn part1_example() {
        assert_eq!(112, part1(EXAMPLE.to_string()).unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(848, part2(EXAMPLE.to_string()).unwrap());
    }
}
