use std::fmt::Write;

use super::solver;

pub struct Day<'a> {
    pub day: u8,
    parts: Vec<Part<'a>>,
}

impl<'a> Day<'a> {
    pub fn new(day: u8) -> Self
    {
        Day {
            day,
            parts: Vec::new(),
        }
    }

    pub fn part<F>(mut self, part: u8, solver: F) -> Self
        where F: 'a + solver::Solver
    {
        self.parts.push(Part::new(part, solver));
        self
    }

    pub fn solve(self, input: String) -> String {
        let mut out = String::new();
        for part in self.parts.into_iter() {
            writeln!(out, "Part: {}", part.part).unwrap();
            writeln!(out, "Solution: {}\n", part.solve(input.clone())).unwrap();
        }
        out
    }
}

struct Part<'a> {
    part: u8,
    solver: Box<dyn solver::Solver + 'a>,
}

impl<'a> Part<'a> {
    fn new<F>(part: u8, solver: F) -> Self
        where F: 'a + solver::Solver
    {
        Part {
            part,
            solver: Box::new(solver),
        }
    }

    fn solve(self, input: String) -> String {
        self.solver.solve(input)
    }
}
