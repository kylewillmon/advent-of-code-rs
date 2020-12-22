use std::ops::{Add, Sub};
use std::convert::TryInto;

use itertools::Itertools;
use anyhow::Result;

pub fn part1(_input: String) -> Result<usize> {
    Ok(0)
}

pub fn part2(_input: String) -> Result<usize> {
    Ok(0)
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

    fn get(&self, p: &Point) -> Cube {
        
    }

    fn _point_iter(&self) -> impl Iterator<Item=Point> + 'static {
        let first = &Point::zero() - &self.origin;
        let ranges = vec![
            (first.x..(first.x + self.size.x)),
            (first.y..(first.y + self.size.y)),
            (first.z..(first.z + self.size.z)),
        ];
        ranges.into_iter().multi_cartesian_product().map(|v| Point::new(v[0], v[1], v[2]))
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
}