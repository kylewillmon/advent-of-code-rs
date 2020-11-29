use std::fmt;

pub trait Solver {
    fn solve(self: Box<Self>, input: String) -> String;
}

impl<F, T: fmt::Display> Solver for F
    where F: FnOnce(String) -> T
{
    fn solve(self: Box<Self>, input: String) -> String
    {
        self(input).to_string()
    }
}
