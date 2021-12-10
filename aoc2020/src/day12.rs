use std::ops::{Add, AddAssign, Mul, MulAssign};

use anyhow::{anyhow, Result};

pub fn part1(input: String) -> Result<i32> {
    let mut ferry = Ferry::new();

    for instruction in input.lines() {
        let (action, value) = instruction.split_at(1);
        let action = action.chars().next().unwrap();
        let value = value.parse::<i32>()?;

        ferry = match action {
            'L' => ferry.turn_ferry(value),
            'R' => ferry.turn_ferry(-value),
            'F' => ferry.forward(value),
            'N' => ferry.move_ferry(NORTH * value),
            'E' => ferry.move_ferry(EAST * value),
            'S' => ferry.move_ferry(SOUTH * value),
            'W' => ferry.move_ferry(WEST * value),
            _ => return Err(anyhow!("unknown action '{}'", action)),
        };
    }

    let pos = ferry.position;
    Ok(pos.x.abs() + pos.y.abs())
}

pub fn part2(input: String) -> Result<i32> {
    let mut ferry = Ferry::new();

    for instruction in input.lines() {
        let (action, value) = instruction.split_at(1);
        let action = action.chars().next().unwrap();
        let value = value.parse::<i32>()?;

        ferry = match action {
            'L' => ferry.turn_waypoint(value),
            'R' => ferry.turn_waypoint(-value),
            'F' => ferry.go_to_waypoint(value),
            'N' => ferry.move_waypoint(NORTH * value),
            'E' => ferry.move_waypoint(EAST * value),
            'S' => ferry.move_waypoint(SOUTH * value),
            'W' => ferry.move_waypoint(WEST * value),
            _ => return Err(anyhow!("unknown action '{}'", action)),
        };
    }

    let pos = ferry.position;
    Ok(pos.x.abs() + pos.y.abs())
}

const NORTH: Coord = Coord::new(0, 1);
const EAST: Coord = Coord::new(1, 0);
const SOUTH: Coord = Coord::new(0, -1);
const WEST: Coord = Coord::new(-1, 0);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ferry {
    direction: i32,
    position: Coord,
    waypoint: Coord,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            direction: 0,
            position: Coord::new(0, 0),
            waypoint: Coord::new(10, 1),
        }
    }

    fn forward(self, value: i32) -> Self {
        match self.direction {
            0 => self.move_ferry(EAST * value),
            90 => self.move_ferry(NORTH * value),
            180 => self.move_ferry(WEST * value),
            270 => self.move_ferry(SOUTH * value),
            _ => panic!("I refuse to do trigonometry..."),
        }
    }

    fn turn_ferry(self, value: i32) -> Self {
        let newdir = (self.direction + value).rem_euclid(360);
        Ferry {
            direction: newdir,
            position: self.position,
            waypoint: self.waypoint,
        }
    }

    fn move_ferry(self, translation: Coord) -> Self {
        Ferry {
            direction: self.direction,
            position: self.position + translation,
            waypoint: self.waypoint,
        }
    }

    fn turn_waypoint(self, dir: i32) -> Self {
        let dir = dir.rem_euclid(360);
        let new_waypoint = self.waypoint.turn(dir);
        Ferry {
            direction: self.direction,
            position: self.position,
            waypoint: new_waypoint,
        }
    }

    fn move_waypoint(self, translation: Coord) -> Self {
        Ferry {
            direction: self.direction,
            position: self.position,
            waypoint: self.waypoint + translation,
        }
    }

    fn go_to_waypoint(self, value: i32) -> Self {
        let translation = self.waypoint * value;
        self.move_ferry(translation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    const fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn turn(self, dir: i32) -> Self {
        match dir {
            0 => self,
            90 => Coord::new(-self.y, self.x),
            180 => Coord::new(-self.x, -self.y),
            270 => Coord::new(self.y, -self.x),
            _ => panic!("NEVER TRIG!"),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul<i32> for Coord {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<i32> for Coord {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "F10\nN3\nF7\nR90\nF11";

    #[test]
    fn part1_example() {
        assert_eq!(25, part1(EXAMPLE.to_string()).unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(286, part2(EXAMPLE.to_string()).unwrap());
    }
}
